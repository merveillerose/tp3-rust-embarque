use embassy_stm32::gpio::{Pull, Level, Input, Speed};
use crate::bsp_ensea::GamepadPins;
use defmt::Format;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Top,
    Bottom,
    Left,
    Right,
    Center,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GamepadState {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
    pub center: bool,
}

pub struct Gamepad {
   pins: GamepadPins,
}   


impl Gamepad {
    pub fn new(pins: GamepadPins) -> Self {
        Self {
            pins,
        }
    }

    pub fn is_pressed(&self, button: Button) -> bool {
        match button {
            Button::Top => self.btn_top.is_low(),
            Button::Bottom => self.btn_bottom.is_low(),
            Button::Left => self.btn_left.is_low(),
            Button::Right => self.btn_right.is_low(),
            Button::Center => self.btn_center.is_low(),
        }
    }
    
    /// Lit l'état des boutons et retourne une structure GamepadState   
    pub fn poll(&self) -> GamepadState {
        GamepadState {
            top: self.btn_top.is_low(),
            bottom: self.btn_bottom.is_low(),
            left: self.btn_left.is_low(),
            right: self.btn_right.is_low(),
            center: self.btn_center.is_low(),
        }
    }
}

// Implémentation du trait Format pour defmt
impl Format for GamepadState {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "top={} bottom={} left={} right={} center={}",
            self.top,
            self.bottom,
            self.left,
            self.right,
            self.center
        );
    }
}