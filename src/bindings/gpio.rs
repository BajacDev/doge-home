use rppal::gpio::Error;
use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;
use std::result::Result::*;
pub mod output_pin;
pub mod mem;

/// The enum type representing the avaiable Gpio
/// .
/// Each enum value is a Gpio that we can use in our project
pub enum GpioPinAvailable {
    Gpio0,
    Gpio1,
}

impl GpioPinAvailable {
    /// Return the BCM GPIO pin number of [GpioPinAvailable].
    fn to_bcm_gpio_pin_number(&self) -> u8 {
        match *self {
            //TODO map to the right bcm that we use
            GpioPinAvailable::Gpio0 => 0,
            GpioPinAvailable::Gpio1 => 1,
        }
    }
}