use crate::bindings::gpio::gpio_controller::GpioController;
use crate::bindings::gpio::GpioOutputPin;

pub struct DoorLock {
    is_open: bool,
}

impl DoorLock {
    /// Create a DoorLock with the associated GpioPin.
    ///
    /// ### Examples
    ///
    /// let mut door_lock = new();
    pub fn new() -> Self {
        DoorLock { is_open: false }
    }

    /// Open the DoorLock on which it is called.
    ///
    /// ### Examples
    ///
    /// new().open();
    pub fn open(
        &mut self,
        gpio_controller: &mut GpioController,
        gpio_output_pin: &mut GpioOutputPin,
    ) {
        self.is_open = true;
        gpio_controller.set_high(gpio_output_pin);
    }

    /// Close the DoorLock on which it is called.
    ///
    /// ### Examples
    ///
    ///
    /// new().close();
    pub fn close(
        &mut self,
        gpio_controller: &mut GpioController,
        gpio_output_pin: &mut GpioOutputPin,
    ) {
        self.is_open = false;
        gpio_controller.set_low(gpio_output_pin);
    }

    /// Toggle the DoorLock on which it is called
    ///
    /// If the DoorLock on which it is called is close then it open it.
    /// If the DoorLock on which it is called is open then it close it
    ///
    /// ### Examples
    ///
    /// new().toggle();
    pub fn toggle(
        &mut self,
        gpio_controller: &mut GpioController,
        gpio_output_pin: &mut GpioOutputPin,
    ) {
        if self.is_open {
            self.close(gpio_controller, gpio_output_pin);
        } else {
            self.open(gpio_controller, gpio_output_pin);
        }
    }
}
