use ssd1306::{
    prelude::*,
    I2CDisplayInterface,
    Ssd1306,
    mode::BufferedGraphicsMode,
};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use heapless::String;
use core::fmt::Write;

pub struct OledDisplay<I2C>
where
    I2C: embedded_hal_1::i2c::I2c,
{
    display: Ssd1306<
        I2CInterface<I2C>,
        DisplaySize128x64,
        BufferedGraphicsMode<DisplaySize128x64>,
    >,
}

impl<I2C: embedded_hal_1::i2c::I2c> OledDisplay<I2C> {
    pub fn new(i2c: I2C) -> Self {
        let iface = I2CDisplayInterface::new(i2c);
        let mut display = Ssd1306::new(iface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        display.init().unwrap();
        Self { display }
    }

    /// Met à jour l'écran avec les valeurs courantes des périphériques.
    /// `gamepad_bits` : bit 0 = haut, 1 = bas, 2 = gauche, 3 = droite, 4 = centre.
    pub fn show(&mut self, encoder_pos: i32, stepper_speed: u32, stepper_cw: bool, gamepad_bits: u8) {
        let style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();

        self.display.clear_buffer();

        let mut buf: String<32> = String::new();

        write!(buf, "Enc : {:+}", encoder_pos).ok();
        Text::with_baseline(&buf, Point::new(0, 0), style, Baseline::Top)
            .draw(&mut self.display).ok();
        buf.clear();

        write!(buf, "Vitesse: {} Hz", stepper_speed).ok();
        Text::with_baseline(&buf, Point::new(0, 14), style, Baseline::Top)
            .draw(&mut self.display).ok();
        buf.clear();

        let dir = if stepper_cw { "Dir : CW " } else { "Dir : CCW" };
        Text::with_baseline(dir, Point::new(0, 28), style, Baseline::Top)
            .draw(&mut self.display).ok();

        let btn = |bit: u8, c: &'static str| if gamepad_bits & bit != 0 { c } else { "-" };
        write!(
            buf, "Pad:{}{}{}{}{}",
            btn(0x01, "H"), btn(0x02, "B"), btn(0x04, "G"),
            btn(0x08, "D"), btn(0x10, "C"),
        ).ok();
        Text::with_baseline(&buf, Point::new(0, 42), style, Baseline::Top)
            .draw(&mut self.display).ok();

        self.display.flush().unwrap();
    }
}
