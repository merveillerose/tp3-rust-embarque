#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU8, Ordering};
use core::default::Default;
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_time::{Timer, Duration};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_sync::mutex::Mutex;
use td3::encoder::RotaryEncoder;
use td3::bargraph::Bargraph;
use td3::bsp_ensea::Board;
use td3::gamepad::Gamepad;
use td3::stepper::{Stepper, Direction};
use td3::bsp_ensea::I2C1Pins;
use td3::oled::OledDisplay;
use embassy_stm32::i2c::I2c;
use panic_probe as _;
use defmt_rtt as _;

// ----Variables partagées -----------------------------------------------
pub static BARGRAPH_VALUE:  AtomicU32 = AtomicU32::new(0);
pub static BARGRAPH_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

pub static STEPPER_SPEED:  AtomicU32  = AtomicU32::new(0);
pub static STEPPER_DIR:    AtomicBool = AtomicBool::new(true); // true = Clockwise
pub static STEPPER_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

pub static EMERGENCY_STOP: AtomicBool = AtomicBool::new(false);

pub static GAMEPAD_BITS: AtomicU8 = AtomicU8::new(0);

// Protège l'accès concurrent au compteur de l'encodeur (encoder_task / emergency_stop_task)
static ENCODER_MUTEX: Mutex<CriticalSectionRawMutex, ()> = Mutex::new(());

// ---Tâches--------------------------------------------------------------

#[embassy_executor::task]
async fn encoder_task(encoder: RotaryEncoder) {
    loop {
        let pos = {
            let _lock = ENCODER_MUTEX.lock().await;
            encoder.position()
        };

        BARGRAPH_VALUE.store(pos.unsigned_abs(), Ordering::Relaxed);
        BARGRAPH_SIGNAL.signal(());

        STEPPER_DIR.store(pos >= 0, Ordering::Relaxed);
        STEPPER_SPEED.store(pos.unsigned_abs().min(1_000), Ordering::Relaxed);
        STEPPER_SIGNAL.signal(());

        Timer::after(Duration::from_millis(100)).await;
    }
}

#[embassy_executor::task]
async fn bargraph_task(mut bargraph: Bargraph) {
    loop {
        bargraph.wait_and_update(&BARGRAPH_SIGNAL, &BARGRAPH_VALUE).await;
    }
}

#[embassy_executor::task]
async fn gamepad_task(gamepad: Gamepad) {
    loop {
        let state = gamepad.poll();
        let bits: u8 = (state.top    as u8)
            | ((state.bottom as u8) << 1)
            | ((state.left   as u8) << 2)
            | ((state.right  as u8) << 3)
            | ((state.center as u8) << 4);
        GAMEPAD_BITS.store(bits, Ordering::Relaxed);
        defmt::info!("Gamepad: {}", state);
        Timer::after(Duration::from_millis(50)).await;
    }
}

#[embassy_executor::task]
async fn stepper_update_task(mut stepper: Stepper) {
    stepper.enable();
    loop {
        STEPPER_SIGNAL.wait().await;

        if EMERGENCY_STOP.load(Ordering::Relaxed) {
            stepper.disable();
            continue;
        }

        let speed = STEPPER_SPEED.load(Ordering::Relaxed);
        let dir   = if STEPPER_DIR.load(Ordering::Relaxed) {
            Direction::Clockwise
        } else {
            Direction::CounterClockwise
        };
        stepper.set_speed(speed, dir);
    }
}

#[embassy_executor::task]
async fn emergency_stop_task(mut button: ExtiInput<'static>) {
    loop {
        button.wait_for_falling_edge().await;
        defmt::warn!("Emergency stop activated!");

        EMERGENCY_STOP.store(true, Ordering::Relaxed);
        STEPPER_SPEED.store(0, Ordering::Relaxed);
        STEPPER_SIGNAL.signal(());

        {
            let _lock = ENCODER_MUTEX.lock().await;
            let tim2 = embassy_stm32::pac::TIM2;
            tim2.cr1().modify(|w| w.set_cen(false));
            tim2.cnt().write_value(5_000);
            tim2.cr1().modify(|w| w.set_cen(true));
        }

        BARGRAPH_VALUE.store(0, Ordering::Relaxed);
        BARGRAPH_SIGNAL.signal(());

        button.wait_for_rising_edge().await;
        EMERGENCY_STOP.store(false, Ordering::Relaxed);
        defmt::info!("Emergency stop deactivated.");
    }
}

#[embassy_executor::task]
async fn oled_task(pins: I2C1Pins) {
    let i2c = I2c::new_blocking(pins.peri, pins.scl, pins.sda, Default::default());
    let mut oled = OledDisplay::new(i2c);
    loop {
        let pos    = BARGRAPH_VALUE.load(Ordering::Relaxed) as i32;
        let speed  = STEPPER_SPEED.load(Ordering::Relaxed);
        let dir_cw = STEPPER_DIR.load(Ordering::Relaxed);
        let pad    = GAMEPAD_BITS.load(Ordering::Relaxed);

        oled.show(pos, speed, dir_cw, pad);

        Timer::after(Duration::from_millis(200)).await;
    }
}

// ----Main------------------------------------------------------------------

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p     = embassy_stm32::init(Default::default());
    let board = Board::new(p);

    let encoder    = RotaryEncoder::new(board.encoder);
    let enc_button = board.enc_button;
    let bargraph   = Bargraph::new(board.bargraph_pins);
    let gamepad    = Gamepad::new(board.gamepad);
    let stepper    = Stepper::new(board.steppers_pins);
    let i2c_pins   = board.i2c1;

    spawner.spawn(encoder_task(encoder)).unwrap();
    spawner.spawn(bargraph_task(bargraph)).unwrap();
    spawner.spawn(gamepad_task(gamepad)).unwrap();
    spawner.spawn(stepper_update_task(stepper)).unwrap();
    spawner.spawn(emergency_stop_task(enc_button)).unwrap();
    spawner.spawn(oled_task(i2c_pins)).unwrap();
}
