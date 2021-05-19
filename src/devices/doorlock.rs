use crate::bindings::gpio::GpioPinAvailable;
use crate::bindings::gpio::mem::GpioController;

pub struct DoorLock {
    is_open: bool,
    gpio: GpioPinAvailable,
}

impl DoorLock {
    /// Create a DoorLock with the associated GpioPin.
    ///
    /// ### Examples
    ///
    /// let mut door_lock = new(gpio_pin);
    pub fn new(gpio_pin_available: GpioPinAvailable) -> Self {
        DoorLock {
            is_open: false,
            gpio: gpio_pin_available,
        }
    }

    /// Open the DoorLock on which it is called.
    ///
    /// ### Examples
    ///
    /// new(gpio_pin).open();
    pub fn open(&mut self, gpio_controller : &mut GpioController) {
        self.is_open = true;
        gpio_controller.set_high(self.gpio)
    }

    /// Close the DoorLock on which it is called.
    ///
    /// ### Examples
    ///
    ///
    /// new(gpio_pin).close();
    pub fn close(&mut self, gpio_controller : &mut GpioController) {
        self.is_open = false;
        gpio_controller.set_low(self.gpio)
    }

    /// Toggle the DoorLock on which it is called
    ///
    /// If the DoorLock on which it is called is close then it open it.
    /// If the DoorLock on which it is called is open then it close it
    ///
    /// ### Examples
    ///
    /// new(gpio_pin).toggle();
    pub fn toggle(&mut self, gpio_controller : &mut GpioController) {
        if self.is_open {
            self.close(gpio_controller);
        } else {
            self.open(gpio_controller);
        }
    }
}
