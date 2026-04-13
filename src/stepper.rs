use embassy_stm32::gpio::{Output, OutputType};
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::timer::Channel;
use embassy_stm32::time::hz;
use embassy_stm32::peripherals::TIM3;
use embedded_hal::Pwm; // ← trait qui fournit enable/disable/set_duty/get_max_duty
use crate::bsp_ensea::SteppersPins;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug, Clone, Copy)]
pub enum MicrosteppingMode {
    Full,
    Half,
    Quarter,
    Eighth,
}

pub struct Stepper {
    pwm:     SimplePwm<'static, TIM3>,
    dir:     Output<'static>,
    enable:  Output<'static>,
    ms1:     Output<'static>,
    ms2:     Output<'static>,
    running: bool,
}

impl Stepper {
    pub fn new(pins: SteppersPins) -> Self {
        // PA6 est un type concret → implémente TimerPin<TIM3, Ch1>
        let step_pin = PwmPin::new(pins.step, OutputType::PushPull);

        let pwm = SimplePwm::new(
            pins.timer,
            Some(step_pin),
            None, None, None,
            hz(1_000),
            Default::default(),
        );

        Self {
            pwm,
            dir:    pins.dir,
            enable: pins.enable,
            ms1:    pins.ms1,
            ms2:    pins.ms2,
            running: false,
        }
    }

    pub fn enable(&mut self) {
        self.enable.set_low();
    }

    pub fn disable(&mut self) {
        self.enable.set_high();
        self.pwm.disable(Channel::Ch1); // ← trait embedded_hal::Pwm
        self.running = false;
    }

    pub fn set_speed(&mut self, speed_hz: u32, direction: Direction) {
        match direction {
            Direction::Clockwise        => self.dir.set_high(),
            Direction::CounterClockwise => self.dir.set_low(),
        }

        if speed_hz == 0 {
            self.pwm.disable(Channel::Ch1);
            self.running = false;
            return;
        }

        self.pwm.set_frequency(hz(speed_hz));

        let max = self.pwm.get_max_duty();          // ← trait embedded_hal::Pwm
        self.pwm.set_duty(Channel::Ch1, max / 2);   // ← trait embedded_hal::Pwm

        if !self.running {
            self.pwm.enable(Channel::Ch1);           // ← trait embedded_hal::Pwm
            self.running = true;
        }
    }

    pub fn set_microstepping(&mut self, mode: MicrosteppingMode) {
        match mode {
            MicrosteppingMode::Full    => { self.ms1.set_low();  self.ms2.set_low();  }
            MicrosteppingMode::Half    => { self.ms1.set_high(); self.ms2.set_low();  }
            MicrosteppingMode::Quarter => { self.ms1.set_low();  self.ms2.set_high(); }
            MicrosteppingMode::Eighth  => { self.ms1.set_high(); self.ms2.set_high(); }
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}
