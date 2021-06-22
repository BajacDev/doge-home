use crate::bindings::gpio::gpio_controller::GpioController;
use crate::bindings::gpio::GpioOutputPin;

pub struct DoorLock {
    pub is_open: bool,
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
        gpio_controller: Option<&mut GpioController>,
        gpio_output_pin: Option<&mut GpioOutputPin>,
    ) {
        self.is_open = true;
        if let Some((gc, gop)) = gpio_controller.zip(gpio_output_pin) {
            gc.set_high(gop);
        }
    }

    /// Close the DoorLock on which it is called.
    ///
    /// ### Examples
    ///
    ///
    /// new().close();
    pub fn close(
        &mut self,
        gpio_controller: Option<&mut GpioController>,
        gpio_output_pin: Option<&mut GpioOutputPin>,
    ) {
        self.is_open = false;
        if let Some((gc, gop)) = gpio_controller.zip(gpio_output_pin) {
            gc.set_low(gop);
        }
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
        gpio_controller: Option<&mut GpioController>,
        gpio_output_pin: Option<&mut GpioOutputPin>,
    ) {
        if self.is_open {
            self.close(gpio_controller, gpio_output_pin);
        } else {
            self.open(gpio_controller, gpio_output_pin);
        }
    }
}
