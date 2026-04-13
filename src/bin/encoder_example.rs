#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

use td3::bsp_ensea::Board;
use td3::encoder::RotaryEncoder;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let board = Board::new(p);
    let Board { encoder, enc_button, .. } = board;
    let mut encoder = RotaryEncoder::new(encoder);

    let mut last_pos = 0i32;

    loop {
        let pos = encoder.position();

        if pos != last_pos {
            info!("Position: {}", pos);
            last_pos = pos;
        }

        if enc_button.is_low() {
            info!("Button pressed -> reset.");
            encoder.reset();
            last_pos = 0;
        }

        Timer::after(Duration::from_millis(10)).await;
    }
}
