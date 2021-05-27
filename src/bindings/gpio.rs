pub mod gpio_controller;
use gpio_controller::GpioController;
use std::sync::atomic::{AtomicBool, Ordering};

/// The enum type representing the avaiable Gpio
///
/// Each enum value is a Gpio that you can use
///
// If you change the enum variant make sure to assign a integer x that respect 0 <= x < [mem::GPIO_SUPPORTED_NBRS].
#[derive(Copy, Clone)]
pub enum GpioPinAvailable {
    Gpio0 = 0,
    Gpio1 = 1,
}

impl GpioPinAvailable {
    /// Return the BCM GPIO pin number of [GpioPinAvailable].
    fn to_bcm_gpio_pin_number(&self) -> usize {
        (*self) as usize
    }
}

/// Each instance represent a GPIO pin of the computer
///
/// ### Examples
///
/// let gpio_pin = GpioPin::new(& GpioPinAvailable::Gpio0)
///
pub struct GpioPin {
    bmc_gpio_pin_number: usize,
}
const GPIO_PINS_IS_TAKEN: [AtomicBool; gpio_controller::GPIO_SUPPORTED_NBRS] = [
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
];
impl GpioPin {
    /// Return the associated [GpioPin] of the passed [GpioPinAvailable].
    ///
    /// ### Examples
    ///
    /// let gpio_pin_0 = new(GpioPinAvailable::GPio0);
    ///
    /// ## Panics
    /// The function panic if you call it when there is already a instance of the [GpioPin] associated
    /// with the passed [GpioPinAvailable] in the process you are in.
    /// When the lifecyle of the associated [GpioPin] is finished, you can get the associated [GpioPin] again.
    ///
    pub fn new(gpio_pin_wanted: &GpioPinAvailable) -> GpioPin {
        let bcm_gpio_pin_number = gpio_pin_wanted.to_bcm_gpio_pin_number();
        GPIO_PINS_IS_TAKEN[bcm_gpio_pin_number]
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .expect("Try to obtain multiple time a GpioPin with BCM pin number");

        GpioPin {
            bmc_gpio_pin_number: bcm_gpio_pin_number,
        }
    }
}

impl Drop for GpioPin {
    fn drop(&mut self) {
        GPIO_PINS_IS_TAKEN[self.bmc_gpio_pin_number]
            .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
            .expect("Try to release the a GpioPin, but was already release");
    }
}

/// Each instance represent a GPIO pin of the computer configure in the output mode.
///
/// ### Examples
///
/// let mut gpio_controller = GpioController::get_the_gpio_controller();
///
/// let mut gpio_output_pin = GpioOutputPin::new(GpioPin::new(& GpioPinAvailable::Gpio0), & gpio_controller);
///
pub struct GpioOutputPin {
    bcm_gpio_pin_number: usize,
    // This field is necessary so that the gpio_pin is not dropped until
    // the instance of [GpioOutputPin] is dropped. This way the associated [gpio_pin] can
    // not be taken again while in use.
    gpio_pin: GpioPin,
}

impl GpioOutputPin {
    /// Return the associated [GpioOutputPin] of the passed [GpioPin].
    ///
    /// Create, setup and return the [GpioOuputPin] associated with the passed [GpioPin].
    ///
    /// ### Examples
    ///
    /// let mut gpio_controller = GpioController::get_the_gpio_controller();
    ///
    /// let mut gpio_output_pin = GpioOutputPin::new(GpioPin::new(& GpioPinAvailable::Gpio0), & gpio_controller);
    ///
    pub fn new(gpio_pin: GpioPin, gpio_controller: &mut GpioController) -> GpioOutputPin {
        let mut gpio_output_pin = GpioOutputPin {
            bcm_gpio_pin_number: gpio_pin.bmc_gpio_pin_number,
            gpio_pin: gpio_pin,
        };
        gpio_controller.set_output_mode(&mut gpio_output_pin);
        gpio_output_pin
    }
}
