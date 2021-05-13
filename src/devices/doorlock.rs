use crate::bindings::gpio::GpioPin;

pub struct DoorLock {
    is_open: bool,
    gpio: GpioPin,
}

impl DoorLock {
    /// Create a DoorLock with the associated GpioPin.
    ///
    /// ### Examples
    ///
    /// let mut door_lock = new(gpio_pin);
    pub fn new(gpio_pin: GpioPin) -> Self {
        DoorLock {
            is_open: false,
            gpio: gpio_pin,
        }
    }

    /// Open the DoorLock on which it is called.
    ///
    /// ### Examples
    ///
    /// new(gpio_pin).open();
    pub fn open(&mut self) {
        self.is_open = true;
        self.gpio.on();
    }

    /// Close the DoorLock on which it is called.
    ///
    /// ### Examples
    ///
    ///
    /// new(gpio_pin).close();
    pub fn close(&mut self) {
        self.is_open = false;
        self.gpio.off();
    }

    /// Toggle the DoorLock on which it is called
    ///
    /// If the DoorLock on which it is called is close then it open it.
    /// If the DoorLock on which it is called is open then it close it
    ///
    /// ### Examples
    ///
    /// new(gpio_pin).toggle();
    pub fn toggle(&mut self) {
        if self.is_open {
            self.close();
        } else {
            self.open();
        }
    }
}
