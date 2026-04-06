#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};



use td3::bsp_ensea::Board;
use td3::bargraph::Bargraph;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let board = Board::new(p);
    let mut bargraph = Bargraph::new(board.bargraph_pins);

    bargraph.set_range(0, 100);

    loop {
        // Montée progressive de 0 à 100
        for v in (0..=100).step_by(10) {
            info!("Bargraph value: {}", v);
            bargraph.set_value(v);
            Timer::after(Duration::from_millis(200)).await;
        }
        // Descente progressive de 100 à 0
        let mut v = 100;
        while v >= 0 {
            info!("Bargraph value: {}", v);
            bargraph.set_value(v);
            Timer::after(Duration::from_millis(200)).await;
            v -= 10;
        }
    }
}