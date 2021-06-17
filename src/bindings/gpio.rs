//! Implementation of gpio pins, which is not thread safe.
//! I.e. the use of this should only come from on thread, if no care to make it thread safe while using it is taken.
//! We choose to not make it thread safe so that the minimum numbers of things has to be put in the trusted comupting base.

pub mod gpio_controller;
use gpio_controller::GpioController;
/// The enum type representing the avaiable Gpio for the rasbperry pi 4b.
///
/// Each enum value is a Gpio that you can use in the rasbperry pi 4b.
///
// You should not change the enum variant as it resepct the bcm number convention of the board, so it is meaningull and well established.
// Otherwise it might confuse future user and reader of this binding.
// If you want to change the number assigned to each enum variant, then you should change the to_bcm_pin_number as it rely on that.
// We did not put Gpio0 and Gpio1 as it is stated that these pins ares reserverd for HAT ID EEPROM see https://www.raspberrypi.org/documentation/hardware/raspberrypi/bcm2711/rpi_DATA_2711_1p0_preliminary.pdf last seen on 17 june 2021.
#[derive(Copy, Clone)]
pub enum GpioPinAvailable {
    Gpio2 = 2,
    Gpio3 = 3,
    Gpio4 = 4,
    Gpio17 = 17,
    Gpio27 = 27,
    Gpio22 = 22,
    Gpio10 = 10,
    Gpio9 = 9,
    Gpio11 = 11,
    Gpio5 = 5,
    Gpio6 = 6,
    Gpio13 = 13,
    Gpio19 = 19,
    Gpio26 = 26,
    Gpio14 = 14,
    Gpio15 = 15,
    Gpio18 = 18,
    Gpio23 = 23,
    Gpio24 = 24,
    Gpio25 = 25,
    Gpio8 = 8,
    Gpio7 = 7,
    Gpio12 = 12,
    Gpio16 = 16,
    Gpio20 = 20,
    Gpio21 = 21,
}

impl GpioPinAvailable {
    /// Return the BCM GPIO pin number of [GpioPinAvailable].
    fn to_bcm_gpio_pin_number(&self) -> usize {
        let bcm_pin_number: usize = (*self) as usize;
        if bcm_pin_number <= gpio_controller::GPIO_MAX_BCM_NUMBER_SUPPORTED {
            panic!("There is a implementation error a bcm pin number is bigger than the max bcm pin number supported in the implemenation")
        }
        bcm_pin_number
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

// We don't need that much atomic boolean as their is less GPIO avaiable on the rasbpery pi but
// it is for simplicity. With this number of atomic boolean we can simply assign a GPIO the atomic boolean
// at the position corresponding to his bcm number
static mut GPIO_PINS_IS_TAKEN: [bool; gpio_controller::GPIO_MAX_BCM_NUMBER_SUPPORTED + 1] =
    [false; gpio_controller::GPIO_MAX_BCM_NUMBER_SUPPORTED + 1];

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
        unsafe {
            if GPIO_PINS_IS_TAKEN[bcm_gpio_pin_number] {
                panic!("Try to obtain multiple time a GpioPin with BCM pin number")
            } else {
                GPIO_PINS_IS_TAKEN[bcm_gpio_pin_number] = true;
            }
        }
        GpioPin {
            bmc_gpio_pin_number: bcm_gpio_pin_number,
        }
    }
}

impl Drop for GpioPin {
    fn drop(&mut self) {
        unsafe {
            if GPIO_PINS_IS_TAKEN[self.bmc_gpio_pin_number] {
                GPIO_PINS_IS_TAKEN[self.bmc_gpio_pin_number] = false;
            } else {
                panic!("Try to release the a GpioPin, but was already release")
            }
        }
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
