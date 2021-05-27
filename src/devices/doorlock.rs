use crate::bindings::gpio::gpio_controller::GpioController;
use crate::bindings::gpio::GpioOutputPin;

pub struct DoorLock {
    is_open: bool,
    gpio_output_pin: GpioOutputPin,
}

impl DoorLock {
    /// Create a DoorLock with the associated GpioPin.
    ///
    /// ### Examples
    ///
    /// let mut door_lock = new(gpio_pin);
    pub fn new(gpio_pin_ouput: GpioOutputPin) -> Self {
        DoorLock {
            is_open: false,
            gpio_output_pin: gpio_pin_ouput,
        }
    }

    /// Open the DoorLock on which it is called.
    ///
    /// ### Examples
    ///
    /// new(gpio_pin).open();
    pub fn open(&mut self, gpio_controller: &mut GpioController) {
        self.is_open = true;
        gpio_controller.set_high(&mut self.gpio_output_pin)
    }

    /// Close the DoorLock on which it is called.
    ///
    /// ### Examples
    ///
    ///
    /// new(gpio_pin).close();
    pub fn close(&mut self, gpio_controller: &mut GpioController) {
        self.is_open = false;
        gpio_controller.set_low(&mut self.gpio_output_pin)
    }

    /// Toggle the DoorLock on which it is called
    ///
    /// If the DoorLock on which it is called is close then it open it.
    /// If the DoorLock on which it is called is open then it close it
    ///
    /// ### Examples
    ///
    /// new(gpio_pin).toggle();
    pub fn toggle(&mut self, gpio_controller: &mut GpioController) {
        if self.is_open {
            self.close(gpio_controller);
        } else {
            self.open(gpio_controller);
        }
    }
}
