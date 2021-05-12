use rppal::gpio::Error;
use rppal::gpio::Gpio;
use std::result::Result::*;
/// The enum type representing the avaiable Gpio
/// .
/// Each enum value is a Gpio that we can use in our project
pub enum GpioPinAvailable {
    Gpio0,
    Gpio1,
}

impl GpioPinAvailable {
    /// Return the BCM GPIO pin number of [GpioPinAvailable]
    ///
    fn to_bcm_gpio_pin_number(&self) -> u8 {
        match *self {
            //TODO map to the right bcm that we use
            GpioPinAvailable::Gpio0 => 0,
            GpioPinAvailable::Gpio1 => 1,
        }
    }

    /// Turn the gpio on, i.e. let the current pass
    ///
    /// ### Examples
    /// use crate::bindings::gpio;
    ///
    /// gpio::GpioPinAvailable::Gpio0.on()*
    pub fn on(&self) -> Result<(), Error> {
        GpioPinAvailable::turn_on(self.to_bcm_gpio_pin_number())
    }

    /// Turn on the gpio
    ///
    /// *'pinNumber': The BCM GPIO pin number of the gpio to turn on
    fn turn_on(pin_number: u8) -> Result<(), Error> {
        let mut pin = Gpio::new()?.get(pin_number)?.into_output();
        pin.set_high();
        pin.set_reset_on_drop(false);
        Ok(())
    }

    /// Turn the gpio off, i.e. do not let the current pass
    ///
    /// ### Examples
    /// use crate::bindings::gpio;
    ///
    /// gpio::GpioPinAvailable::Gpio0.off()
    fn off(&self) -> Result<(), Error> {
        GpioPinAvailable::turn_off(self.to_bcm_gpio_pin_number())
    }
    /// Turn off the gpio
    ///
    /// *'pinNumber': The BCM GPIO pin number of the gpio to turn off
    fn turn_off(pin_number: u8) -> Result<(), Error> {
        let mut pin = Gpio::new()?.get(pin_number)?.into_output();
        pin.set_high();
        pin.set_reset_on_drop(false);
        Ok(())
    }
}
