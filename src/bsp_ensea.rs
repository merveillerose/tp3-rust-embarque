
use embassy_stm32::gpio::{AnyPin, Input, Level, Output, Speed, Pull};
use embassy_stm32::Peri;
use embassy_stm32::peripherals::{PA0, PA1, PA6, TIM2, TIM3};

pub struct Board{
    pub bargraph_pins: BargraphPins,
    pub steppers_pins: SteppersPins,
    pub gamepad: GamepadPins,
    pub spi2: Spi2Pins,
    pub gps: GPSPins,
    pub usart1: USART1Pins,
    pub usart2: USART2Pins,
    pub i2c1: I2C1Pins,
    pub encoder: EncoderPins,
    pub magneto: MagnetoPins,
    pub gpio: GPIOPins,
    pub connector: ConnectorPins,
    
}

// Bargraph 8 LEDs
pub struct BargraphPins {
    pub led0: Output<'static>, 
    pub led1: Output<'static>,    
    pub led2: Output<'static>,
    pub led3: Output<'static>,
    pub led4: Output<'static>,
    pub led5: Output<'static>,
    pub led6: Output<'static>,
    pub led7: Output<'static>,
}

// Gamepad 5 buttons
pub struct GamepadPins {
    pub btn_top: Input<'static>,
    pub btn_bottom: Input<'static>,
    pub btn_left: Input<'static>,
    pub btn_right: Input<'static>,
    pub btn_center: Input<'static>,
}

// Encodeur rotatif
pub struct EncoderPins {
    pub ch_a:   Peri<'static, PA0>,
    pub ch_b: Peri<'static, PA1>,
    pub button: Input<'static>,
    pub timer:  Peri<'static, TIM2>,
}

// 2 moteurs pas à pas
pub struct SteppersPins {
    pub dir: Output<'static>,  
    pub step: Peri<'static, PA6>,
    pub enable: Output<'static>,
    pub ms1: Output<'static>,
    pub ms2: Output<'static>,
    pub timer: Peri<'static, TIM3>,
}

// SPI2 pour le lecteur de cartes SD
pub struct Spi2Pins {
    pub sck: Output<'static>, // PB10
    pub miso: Input<'static>, // PC2
    pub mosi: Output<'static>, // PC3
    pub cs: Output<'static>  // PC0
}

pub struct GPSPins {
    pub enable: Output<'static>,
}

pub struct USART1Pins {
    pub tx: Output<'static>, //PA9
    pub rx: Input<'static> //PA10
}

pub struct USART2Pins {
    pub tx: Output<'static>, // PA2
    pub rx: Input<'static> // PA3
}

pub struct I2C1Pins {
    pub scl: Output<'static>, // PB6
    pub sda: Peri<'static, AnyPin> // PB7
}

pub struct MagnetoPins {
    pub status: Input<'static>, // PC1
    pub int: Input<'static>, // PB0
}

pub struct GPIOPins {
    pub ld2: Output<'static>,
    pub blue_btn: Input<'static>, // Blue button (PC13)
}

pub struct ConnectorPins {
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
           bargraph_pins: BargraphPins {
            led7: Output::new(p.PB5, Level::Low, Speed::Low), 
            led6: Output::new(p.PB14, Level::Low, Speed::Low),  
            led5: Output::new(p.PB4, Level::Low, Speed::Low),  
            led4: Output::new(p.PB15, Level::Low, Speed::Low),  
            led3: Output::new(p.PB1, Level::Low, Speed::Low),   
            led2: Output::new(p.PA8, Level::Low, Speed::Low),   
            led1: Output::new(p.PB2, Level::Low, Speed::Low),   
            led0: Output::new(p.PC7, Level::Low, Speed::Low),   
},
            gamepad: GamepadPins {
                btn_top:    Input::new(p.PC8, Pull::Up),
                btn_bottom: Input::new(p.PB11, Pull::Up),
                btn_right:  Input::new(p.PC9, Pull::Up),
                btn_left:   Input::new(p.PC6, Pull::Up),
                btn_center: Input::new(p.PC5, Pull::Up),
            },
            encoder: EncoderPins {
                button: Input::new(p.PA15, Pull::Up),
                ch_a:   p.PA0,
                ch_b:   p.PA1,
                timer:  p.TIM2.into(),
            },
            steppers_pins: SteppersPins {
                dir:    Output::new(p.PA7, Level::Low, Speed::Low),
                step:   p.PA6, 
                enable: Output::new(p.PA12, Level::Low, Speed::Low),
                ms1:    Output::new(p.PA11, Level::Low, Speed::Low),
                ms2:    Output::new(p.PB12, Level::Low, Speed::Low),
                timer:  p.TIM3.into(),
            },
            gps: GPSPins {
                enable: Output::new(p.PB13, Level::Low, Speed::Low),
            },
            usart1: USART1Pins {
                tx: Output::new(p.PA9, Level::High, Speed::Low),
                rx: Input::new(p.PA10, Pull::Up),
            },
            usart2: USART2Pins {
                tx: Output::new(p.PA2, Level::High, Speed::Low),
                rx: Input::new(p.PA3, Pull::Up),
            },
            spi2: Spi2Pins {
                sck:  Output::new(p.PB10, Level::Low, Speed::Low),
                mosi: Output::new(p.PC3, Level::Low, Speed::Low),
                miso: Input::new(p.PC2, Pull::Up),
                cs:   Output::new(p.PC0, Level::High, Speed::Low),
            },
            i2c1: I2C1Pins {
                scl: Output::new(p.PB6, Level::Low, Speed::Low),
                sda: p.PB7.into(),
            },
            magneto: MagnetoPins {
                status: Input::new(p.PC1, Pull::Up),
                int:    Input::new(p.PB0, Pull::Up),
            },
            gpio: GPIOPins {
                ld2:      Output::new(p.PA5, Level::Low, Speed::Low),
                blue_btn: Input::new(p.PC13, Pull::Up),
            },
            connector: ConnectorPins {
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