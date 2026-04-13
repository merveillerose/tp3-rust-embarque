use embassy_stm32::timer::qei::{Qei, Config};
use embassy_stm32::peripherals::TIM2;
use crate::bsp_ensea::EncoderPins;

pub struct RotaryEncoder {
    qei:    Qei<'static, TIM2>,
  //button: Input<'static>,
}

impl RotaryEncoder {
    pub fn new(pins: EncoderPins) -> Self {  
        let qei = Qei::new(pins.timer, pins.ch_a, pins.ch_b, Config::default());

        let tim2 = embassy_stm32::pac::TIM2;
        tim2.arr().write_value(10_000);
        tim2.cnt().write_value(5_000);

        Self {
            qei,
          //button: pins.button,
        }
    }

    pub fn raw_count(&self) -> u32 {
        self.qei.count().into()
    }

    pub fn position(&self) -> i32 {
        self.qei.count() as i32 - 5_000
    }

    pub fn set_position(&mut self, position: i32) {
        let raw = (5_000 + position).clamp(0, 10_000) as u32;
        embassy_stm32::pac::TIM2.cnt().write_value(raw);
    }

    pub fn reset(&mut self) {
        embassy_stm32::pac::TIM2.cnt().write_value(5_000);
    }

    // pub fn is_pressed(&self) -> bool {
    //     self.button.is_low()
    // }
}
