//! Implementation of a memory map (dev/gpiomem) gpio controller, which is not thread safe.
//! I.e. the use of this should only come from on thread, if no care to make it thread safe while using it is taken.
//!
//! To ensure that the [GpioController] users remember or notice that this class is not thread safe,
//! all public method for the structure require a mutable reference.
//!
//! To not implement [GpioController] as thread safe, is a design choices made to use the minimum
//! amount of abstraction, even from rust.
//!
//! It is implemented for the rasbpery pi 4b (BCM2711).
//! See [https://datasheets.raspberrypi.org/bcm2711/bcm2711-peripherals.pdf] for more information on the chip
use crate::bindings::gpio::GpioOutputPin;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};

use libc::{self, c_void, size_t, MAP_FAILED, MAP_SHARED, O_SYNC, PROT_READ, PROT_WRITE};

const PATH_DEV_GPIOMEM: &str = "/dev/gpiomem";
// The rasbperry pi has as the max bcm number for a GPIO that it offer 27. This should not confuse with what the BCM2711 offer.
pub const GPIO_MAX_BCM_NUMBER : usize = 27;
const GPIO_MEM_SIZE_REQUIRED_FROM_GLOBAL_OFFSET: usize = GPCLR0_OFFSET
    + (GPIO_MAX_BCM_NUMBER / std::mem::size_of::<u32>()
        + if GPIO_MAX_BCM_NUMBER % std::mem::size_of::<u32>() != 0 {
            1
        } else {
            0
        });
/// GPIO Function Select 0 relative offset.
const GPFSEL0_OFFSET: usize = 0x00;
const GPFSEL_NUNBERS_GPIO_PER_REGISTER : usize = 10;
const GPSEL_NUMBERS_BITS_PER_GPIO : usize = 3;
// There is only 3 bit to set per GPIO
const GPSEL_OUTPUT_MODE_BITS_CONFIGURATION: u32 = 0b001;
/// GPIO Pin Output Set 0 relative offset.
const GPSET0_OFFSET: usize = 0x1c;
const GPSET_NUMBERS_GPIO_PER_REGISTER: usize = 32;
/// GPIO Pin Output Clear 0 relative offset.
const GPCLR0_OFFSET: usize = 0x28;
const GPCLR_NUMBERS_GPIO_PER_REGISTER: usize = 32;

// Use in combination with [get_the_gpio_controller] to make sure only one instance of this structure is created within the process
const GPIO_CONTROLLER_IS_TAKEN: AtomicBool = AtomicBool::new(false);

/// Implementation of a memory map (dev/gpiomem) gpio controller, which is not thread safe.
/// I.e. the use of this should only come from on thread, if no care to make it thread safe while using it is taken.
///
/// This structure allow you to controll a output gpio, i.e. the gpio let the current pass or not
///
/// ### Examples
///         
/// let mut gpio_controller = GpioController::get_the_gpio_controller();
///
/// let mut gpio_output_pin = GpioOutputPin::new(GpioPin::new(& GpioPinAvailable::Gpio0), & gpio_controller);
///
/// gpio_controller.set_high(& gpio_outputpin);
///
pub struct GpioController {
    /// A u32 pointer in C
    // as registers or of size x pointer should be ux
    mem_ptr: *mut u32,
}

impl GpioController {
    /// Give back the singleton [GpioController] or panic
    ///
    /// The function give you back the singleton [GpioController] if it is not present in the process that call it.
    ///
    /// ## Panics
    /// The function panic if you call it when there is already a instance of [GpioController] in the process you are in.
    /// When the lifecyle of the GpioController is finished, you can get a [GpioController] again.
    ///
    pub fn get_the_gpio_controller() -> GpioController {
        // Will panic if we try to get multiple instance of GpioController
        GPIO_CONTROLLER_IS_TAKEN
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .expect("Try to obtain multiple GpioController");
        GpioController::new()
    }

    /// Instantiate a [GpioController]
    ///
    /// ## Panics
    /// see [map_devgpiomem] panics
    ///
    fn new() -> GpioController {
        let mem_ptr = Self::map_devgpiomem();
        GpioController { mem_ptr: mem_ptr }
    }

    /// Return the start user space address of the gpio interface registers.
    ///
    /// Map the gpio register physical adress to the user space virtual addresses,
    /// and return the user space virtual adress where it is map.
    ///
    /// ## Panics
    /// if /dev/gpiomem doesn't exist (< Raspbian Jessie), or /dev/gpiomem
    /// doesn't have the appropriate permissions, or the current user is
    /// not a member of the gpio group.
    ///
    fn map_devgpiomem() -> *mut u32 {
        // Open /dev/gpiomem with read/write/sync flags. This might fail if
        // /dev/gpiomem doesn't exist (< Raspbian Jessie), or /dev/gpiomem
        // doesn't have the appropriate permissions, or the current user is
        // not a member of the gpio group.
        let gpiomem_file = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(O_SYNC)
            .open(PATH_DEV_GPIOMEM)
            .unwrap_or_else(|_| panic!("Problem opening the file {}", PATH_DEV_GPIOMEM));
        // Memory-map /dev/gpiomem at offset 0
        let gpiomem_ptr = unsafe {
            libc::mmap(
                ptr::null_mut(),
                GPIO_MEM_SIZE_REQUIRED_FROM_GLOBAL_OFFSET,
                PROT_READ | PROT_WRITE,
                MAP_SHARED,
                gpiomem_file.as_raw_fd(),
                0,
            )
        };

        if gpiomem_ptr == MAP_FAILED {
            panic!("Could not map the file {} to user space", PATH_DEV_GPIOMEM);
        }

        gpiomem_ptr as *mut u32
    }

    /// Read 32 bits from the offset without reordering by the the compiler or cpu
    /// in respect to other volatile operation.
    #[inline(always)]
    fn read(&self, offset: usize) -> u32 {
        unsafe { ptr::read_volatile(self.mem_ptr.add(offset)) }
    }

    /// Write 32 bits from the offset without reordering by the the compiler or cpu
    /// in respect to other volatile operation.
    #[inline(always)]
    fn write(&self, offset: usize, value: u32) {
        unsafe {
            ptr::write_volatile(self.mem_ptr.add(offset), value);
        }
    }

    /// Set the passed [GpioOutputPin] to high.
    ///
    /// ### Examples
    ///
    /// let mut gpio_controller = GpioController::get_the_gpio_controller();
    ///
    /// let mut gpio_output_pin = GpioOutputPin::new(GpioPin::new(& GpioPinAvailable::Gpio0), & gpio_controller);
    ///
    /// gpio_controller.set_high(& gpio_outputpin);
    ///
    #[inline(always)]
    pub fn set_high(&mut self, gpio_output_pin: &mut GpioOutputPin) {
        //TODO remove those calculation since we only work with 32 gpio i.e. support 32
        let offset = GPSET0_OFFSET + gpio_output_pin.bcm_gpio_pin_number / GPSET_NUMBERS_GPIO_PER_REGISTER;
        let shift = gpio_output_pin.bcm_gpio_pin_number % GPSET_NUMBERS_GPIO_PER_REGISTER;
        self.write(offset, 1 << shift);
    }

    /// Set the passed [GpioOutputPin] pin to low.
    ///
    /// ### Examples
    ///
    /// let mut gpio_controller = GpioController::get_the_gpio_controller();
    ///
    /// let mut gpio_output_pin = GpioOutputPin::new(GpioPin::new(& GpioPinAvailable::Gpio0), & gpio_controller);
    ///
    /// gpio_controller.set_low(& gpio_outputpin);
    ///
    #[inline(always)]
    pub fn set_low(&mut self, gpio_output_pin: &mut GpioOutputPin) {
        let offset =
            (GPCLR0_OFFSET + gpio_output_pin.bcm_gpio_pin_number) / GPCLR_NUMBERS_GPIO_PER_REGISTER;
        let shift = gpio_output_pin.bcm_gpio_pin_number % GPCLR_NUMBERS_GPIO_PER_REGISTER;

        self.write(offset, 1 << shift);
    }

    /// Configure the passed [GpioOutputPin] to the correct mode, i.e. output mode.
    ///
    /// ### Examples
    ///
    /// let mut gpio_controller = GpioController::get_the_gpio_controller();
    ///
    /// let mut gpio_output_pin = GpioOutputPin::new(GpioPin::new(& GpioPinAvailable::Gpio0), & gpio_controller);
    ///
    /// gpio_controller.set_ouput_mode(& gpio_outputpin);
    ///
    /// **Note** : In this example the use of set_output_mode is uneccessary as the GpioOutputPin is already
    /// set to ouput mode from the GpioOutput::new() call.
    ///
    #[inline(always)]
    pub fn set_output_mode(&mut self, gpio_pin: &mut GpioOutputPin) {
        let offset = GPFSEL0_OFFSET + gpio_pin.bcm_gpio_pin_number / GPFSEL_NUNBERS_GPIO_PER_REGISTER;
        let shift = (gpio_pin.bcm_gpio_pin_number % GPFSEL_NUNBERS_GPIO_PER_REGISTER) * GPSEL_NUMBERS_BITS_PER_GPIO;
        let reg_value = self.read(offset);
        self.write(
            offset,
            (reg_value & !(0b111 << shift)) | ((GPSEL_OUTPUT_MODE_BITS_CONFIGURATION) << shift),
        );
    }
}

impl Drop for GpioController {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(
                self.mem_ptr as *mut c_void,
                GPIO_MEM_SIZE_REQUIRED_FROM_GLOBAL_OFFSET as size_t,
            )
        };
        GPIO_CONTROLLER_IS_TAKEN
            .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
            .expect("Try to release the current GpioController, but was already release");
    }
}
