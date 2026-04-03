use embassy_stm32::gpio::{AnyPin, Output, PushPull};
use embassy_stm32::Peri;

pub struct Board{
    pub bargraph_pins: Bargraph_Pins,
    pub steppers_pins: Steppers_Pins,
    pub gamepad: Gamepad_Pins,
    pub spi2: Spin2_Pins,
    pub gps: GPS_Pins,
    pub Usart1: USART1_Pins,
    pub Usart2: USART2_Pins,
    pub spi2: Spin2_Pins,
    pub i2c1: I2C1_Pins,
    pub encoder: Encoder_Pins,
    pub magneto: Magneto_Pins,
    pub gpio: GPIO_Pins,
    pub connector: Connector_Pins,
}

// Bargraph 8 LEDs
pub struct Bargraph_Pins {
    pub led0: Peri<'static, AnyPin>,
    pub led1: Peri<'static, AnyPin>,    
    pub led2: Peri<'static, AnyPin>,
    pub led3: Peri<'static, AnyPin>,
    pub led4: Peri<'static, AnyPin>,
    pub led5: Peri<'static, AnyPin>,
    pub led6: Peri<'static, AnyPin>,
    pub led7: Peri<'static, AnyPin>,
}

// Gamepad 5 buttons
pub struct Gamepad_Pins {
    pub btn_top: Peri<'static, AnyPin>,
    pub btn_bottom: Peri<'static, AnyPin>,
    pub btn_left: Peri<'static, AnyPin>,
    pub btn_right: Peri<'static, AnyPin>,
    pub btn_center: Peri<'static, AnyPin>,
}

// Encodeur rotatif
pub struct Encoder_Pins {
    pub ch_a: Peri<'static, AnyPin>,
    pub ch_b: Peri<'static, AnyPin>,
    pub button: Peri<'static, AnyPin>
}

// 2 moteurs pas à pas
pub struct Steppers_Pins {
    pub dir: Peri<'static, AnyPin>,
    pub step: Peri<'static, AnyPin>,
    pub enable: Peri<'static, AnyPin>
    pub ms1: Peri<'static, AnyPin>,
    pub ms2: Peri<'static, AnyPin>,
}

// SPI2 pour le lecteur de cartes SD
pub struct Spin2_Pins {
    pub sck: Peri<'static, AnyPin>,
    pub miso: Peri<'static, AnyPin>,
    pub mosi: Peri<'static, AnyPin>,
    pub cs: Peri<'static, AnyPin>
}

pub struct GPS_Pins {
    pub enable: Peri<'static, AnyPin>,
}

pub struct USART1_Pins {
    pub tx: Peri<'static, AnyPin>, //PA9
    pub rx: Peri<'static, AnyPin> //PA10
}

pub struct USART2_Pins {
    pub tx: Peri<'static, AnyPin>, // PA2
    pub rx: Peri<'static, AnyPin> // PA3
}

pub struct I2C1_Pins {
    pub scl: Peri<'static, AnyPin>, // PB6
    pub sda: Peri<'static, AnyPin> // PB7
}

pub struct Magneto_Pins {
    pub status: Peri<'static, AnyPin>, // PC1
    pub int: Peri<'static, AnyPin>, // PB0
}

pub struct GPIO_Pins {
    pub ld2: Peri<'static, AnyPin>, // LD2 (PC13)pa5    
    pub blue_btn: Peri<'static, AnyPin>, // Blue button (PC13)
}

pub struct Connector_Pins {
    pub pc10: Peri<'static, AnyPin>,
    pub pc11: Peri<'static, AnyPin>,
    pub pc12: Peri<'static, AnyPin>,
    pub pb8: Peri<'static, AnyPin>,
    pub pb9: Peri<'static, AnyPin>,
    pub pd2: Peri<'static, AnyPin>,
}


impl Board {
    pub fn new(p: embassy_stm32::Peripherals) -> Self {
        Self {
            bargraph: Bargraph_Pins {
                led0: p.PC7.into(),
                led1: p.PB2.into(),
                led2: p.PA8.into(),
                led3: p.PB1.into(),
                led4: p.PB15.into(),
                led5: p.PB4.into(),
                led6: p.PB14.into(),
                led7: p.PB5.into(),
            },
            gamepad: Gamepad_Pins {
                btn_top:    p.PC8.into(),
                btn_bottom: p.PB11.into(),
                btn_right:  p.PC9.into(),
                btn_left:   p.PC6.into(),
                btn_center: p.PC5.into(),
            },
            encoder: Encoder_Pins {
                button: p.PA15.into(),
                ch_a:   p.PA0.into(),
                ch_b:   p.PA1.into(),
            },
            stepper: Steppers_Pins {
                dir:    p.PA7.into(),
                step:   p.PA6.into(),
                enable: p.PA12.into(),
                ms1:    p.PA11.into(),
                ms2:    p.PB12.into(),
            },
            gps: GPS_Pins {
                enable: p.PB13.into(),
            },
            usart1: Usart1_Pins {
                tx: p.PA9.into(),
                rx: p.PA10.into(),
            },
            usart2: Usart2Pins {
                tx: p.PA2.into(),
                rx: p.PA3.into(),
            },
            spi2: Spi2_Pins {
                sck:  p.PB10.into(),
                mosi: p.PC3.into(),
                miso: p.PC2.into(),
                cs:   p.PC0.into(),
            },
            i2c1: I2C1_Pins {
                scl: p.PB6.into(),
                sda: p.PB7.into(),
            },
            magneto: Magneto_Pins {
                status: p.PC1.into(),
                int:    p.PB0.into(),
            },
            gpio: GPIO_Pins {
                ld2:      p.PA5.into(),
                blue_btn: p.PC13.into(),
            },
            connector: Connector_Pins {
                pc10: p.PC10.into(),
                pc11: p.PC11.into(),
                pc12: p.PC12.into(),
                pb8:  p.PB8.into(),
                pb9:  p.PB9.into(),
                pd2:  p.PD2.into(),
            },
        }
    }
}