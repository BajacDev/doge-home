use rppal::gpio::Error;
use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;
use std::result::Result::*;
/// The enum type representing the avaiable Gpio
/// .
/// Each enum value is a Gpio that we can use in our project
pub enum GpioPinAvailable {
    Gpio0,
    Gpio1,
}

impl GpioPinAvailable {
    /// Create a GpioPin
    ///
    /// Create the GpioPin associated with the caller GpioPinAvailable, or throw a error.
    /// See enum type Error of rppal::gpio::Gpio
    ///
    /// ### Examples
    ///
    /// gpio_pin_avaialable.new()
    pub fn new(&self) -> Result<GpioPin, Error> {
        let mut output_pin = Gpio::new()?
            .get(self.to_bcm_gpio_pin_number())?
            .into_output();
        output_pin.set_reset_on_drop(false);
        Ok(GpioPin {
            gpio_pin: output_pin,
        })
    }

    /// Return the BCM GPIO pin number of [GpioPinAvailable]
    ///
    fn to_bcm_gpio_pin_number(&self) -> u8 {
        match *self {
            //TODO map to the right bcm that we use
            GpioPinAvailable::Gpio0 => 0,
            GpioPinAvailable::Gpio1 => 1,
        }
    }
}

pub struct GpioPin {
    gpio_pin: OutputPin,
}

impl GpioPin {
    /// Turn the gpio on, i.e. let the current pass
    ///
    /// ### Examples
    ///
    /// gpio_pin.on()
    pub fn on(&mut self) {
        self.gpio_pin.set_high();
    }

    /// Turn the gpio off, i.e. do not let the current pass
    ///
    /// ### Examples
    ///
    /// gpio_pin.off()
    pub fn off(&mut self) {
        self.gpio_pin.is_set_high();
    }
}
