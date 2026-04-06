use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::Peri;
use crate::bsp_ensea::BargraphPins;

pub struct Bargraph {
    leds: [Output<'static>; 8],
    min: i32,
    max: i32,
}

impl Bargraph {
    pub fn new(pins: BargraphPins) -> Self {
        Self {
            leds: [
                pins.led0.into(),
                pins.led1.into(),
                pins.led2.into(),
                pins.led3.into(),
                pins.led4.into(),
                pins.led5.into(),
                pins.led6.into(),
                pins.led7.into(),
            ],
            min: 0,
            max: 100,
        }
    }

    /// Définit la plage de valeurs (min, max)
    pub fn set_range(&mut self, min: i32, max: i32) {
        assert!(min < max, "min doit être strictement inférieur à max");
        self.min = min;
        self.max = max;
    }

    /// Allume un nombre de LEDs proportionnel à la valeur dans [min, max]
    pub fn set_value(&mut self, value: i32) {
        let value = value.clamp(self.min, self.max);
        let n = self.leds.len() as i32;
        
        // Calcul du nombre de LEDs à allumer (0 à 8)
        let lit = ((value - self.min) * n) / (self.max - self.min);

        for (i, led) in self.leds.iter_mut().enumerate() {
            if (i as i32) < lit {
                led.set_high();
            } else {
                led.set_low();
            }
        }
    }

    /// Éteint toutes les LEDs
    pub fn clear(&mut self) {
        for led in self.leds.iter_mut() {
            led.set_low();
        }
    }

    /// Allume toutes les LEDs
    pub fn fill(&mut self) {
        for led in self.leds.iter_mut() {
            led.set_high();
        }
    }
}