#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

use td3::bsp_ensea::Board;
use td3::stepper::{Direction, MicrosteppingMode, Stepper};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let board = Board::new(p);                           // p entièrement consommé ici
    let mut stepper = Stepper::new(board.steppers_pins); // TIM3 déjà dans le BSP

    stepper.set_microstepping(MicrosteppingMode::Half);
    stepper.enable();

    loop {
        info!("→ Clockwise 500 Hz");
        stepper.set_speed(500, Direction::Clockwise);
        Timer::after(Duration::from_secs(2)).await;

        info!("← CounterClockwise 1000 Hz");
        stepper.set_speed(1_000, Direction::CounterClockwise);
        Timer::after(Duration::from_secs(2)).await;

        info!("■ Stop");
        stepper.set_speed(0, Direction::Clockwise);
        Timer::after(Duration::from_secs(1)).await;
    }
}