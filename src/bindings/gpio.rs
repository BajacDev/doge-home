pub mod mem;

use mem::GpioController;

/// The enum type representing the avaiable Gpio
/// .
/// Each enum value is a Gpio that we can use in our project
pub enum GpioPinAvailable {
    Gpio0,
    Gpio1,
}
const GPIO_PIN_AVAILABLE_NUMBER : usize = 2;
impl GpioPinAvailable {
    /// Return the BCM GPIO pin number of [GpioPinAvailable].
    fn to_bcm_gpio_pin_number(&self) -> usize {
        match *self {
            //TODO map to the right bcm that we use
            GpioPinAvailable::Gpio0 => 0,
            GpioPinAvailable::Gpio1 => 1,
        }
    }
}

pub struct GpioPin{
    BCM_GPIO_pin_number : usize
}
static mut GPIO_PIN_ACCESS_CONTROLLER : [Option<GpioPin>; GPIO_PIN_AVAILABLE_NUMBER] = [Some(GpioPin{BCM_GPIO_pin_number : 0}); GPIO_PIN_AVAILABLE_NUMBER];
impl GpioPin{

    //TODO documentation with panic field 
    pub fn new(gpio_pin_wanted: &GpioPinAvailable) -> GpioPin{
        let BCM_GPIO_pin_number = gpio_pin_wanted.to_bcm_gpio_pin_number();
        let gpio_pin = unsafe{GPIO_PIN_ACCESS_CONTROLLER[BCM_GPIO_pin_number as usize].expect("Try to obtain multiple GpioController")};
        unsafe{GPIO_PIN_ACCESS_CONTROLLER[BCM_GPIO_pin_number as usize] = None};
        gpio_pin.BCM_GPIO_pin_number = BCM_GPIO_pin_number;
        gpio_pin
    }
}

pub struct GpioOutputPin {
    BCM_GPIO_pin_number : usize
}

impl GpioOutputPin {
    //TODO good documentation
    /// Create a GpioPin.
    ///
    /// Create the GpioPin associated with the passed GpioPinAvailable, or throw a error.
    /// See enum type Error of rppal::gpio::Gpio.
    ///
    /// ### Examples
    ///
    /// let gpio_pin_0 = new(GpioPinAvailable::GPio0);
    pub fn new(gpio_pin: GpioPin,  gpio_controller : &mut GpioController) -> GpioOutputPin {
        let mut gpio_output_pin = GpioOutputPin{BCM_GPIO_pin_number : gpio_pin.BCM_GPIO_pin_number};
        gpio_controller.setOutputMode(& mut gpio_output_pin);
        gpio_output_pin
    }
}