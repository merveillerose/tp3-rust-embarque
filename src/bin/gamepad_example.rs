#![no_std]
#![no_main]


use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};


use td3::bsp_ensea::Board;
use td3::gamepad::{Button, Gamepad};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let board = Board::new(p);
    let gamepad = Gamepad::new(board.gamepad);

    loop {
        let state = gamepad.poll();
        info!("Gamepad state: {}", state);
           if gamepad.is_pressed(Button::Center) {
            info!("Centre détecté via is_pressed()");
        }
        Timer::after(Duration::from_millis(200)).await;
    }
}
