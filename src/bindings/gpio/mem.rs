//! Implementation of a memory map, which is not multiple thread safe. i.e. the use of this should only come from on thread
//! It is implemented for the rasbpery pi 4b (BCM2711).
//! See https://datasheets.raspberrypi.org/bcm2711/bcm2711-peripherals.pdf for more information on the chip
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::ptr;
use crate::bindings::gpio::GpioPinAvailable;

use libc::{self, c_void, size_t, MAP_FAILED, MAP_SHARED, O_SYNC, PROT_READ, PROT_WRITE};


const PATH_DEV_GPIOMEM: &str = "/dev/gpiomem";
/// The BCM2711 (RPi4) has 58 32-bit registers related to the GPIO. However for simpliciy I will only implement 32.

const GPIO_MEM_REGISTERS: usize = 32;
const GPIO_MEM_SIZE: usize = GPCLR0 + (GPIO_MEM_REGISTERS/std::mem::size_of::<u32>() + if (GPIO_MEM_REGISTERS%std::mem::size_of::<u32>()!=0) {1} else {0} );
const GPFSEL0: usize = 0x00;
const GPSET0: usize = 0x1c;
const GPCLR0: usize = 0x28;

pub struct GpioController {
    mem_ptr: *mut u32,
}

impl GpioController {
    pub fn open() -> GpioController {
        // Try /dev/gpiomem first. If that fails, try /dev/mem instead. If neither works,
        // report back the error that's the most relevant.
        //TODO write it so that it is a singleton panic if ask for one but already exist one, 
        // indead there should be only one so that we are sure there is no concurency, we force it
        let mem_ptr = Self::map_devgpiomem();
        GpioController{mem_ptr}
    }

    /// map the gpio register to the user space virtual addresses
    fn map_devgpiomem() -> *mut u32 {
        // Open /dev/gpiomem with read/write/sync flags. This might fail if
        // /dev/gpiomem doesn't exist (< Raspbian Jessie), or /dev/gpiomem
        // doesn't have the appropriate permissions, or the current user is
        // not a member of the gpio group.
        //TODO maybe print to the console to tell the user what he can do to fix this bug ask the others
        let gpiomem_file = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(O_SYNC)
            .open(PATH_DEV_GPIOMEM).expect("Problem opening the file /dev/gpiomem");
        
        // Memory-map /dev/gpiomem at offset 0
        let gpiomem_ptr = unsafe {
            libc::mmap(
                ptr::null_mut(),
                GPIO_MEM_SIZE,
                PROT_READ | PROT_WRITE,
                MAP_SHARED,
                gpiomem_file.as_raw_fd(),
                0,
            )
        };

        if gpiomem_ptr == MAP_FAILED {
            panic!("Could not map the file /dev/gpiomem to user space");
        }

        gpiomem_ptr as *mut u32
    }

    /// Read without reordering by the compiler in respect to other volatile operation
    #[inline(always)]
    fn read(&self, offset: usize) -> u32 {
        unsafe { ptr::read_volatile(self.mem_ptr.add(offset)) }
    }

    /// Write without reordering by the the compiler or cpu in respect to other volatile operation
    #[inline(always)]
    fn write(&self, offset: usize, value: u32) {
        unsafe {
            ptr::write_volatile(self.mem_ptr.add(offset), value);
        }
    }


    /// Set the pin to high
    #[inline(always)]
    pub fn set_high(& mut self, gpio_pin_available : GpioPinAvailable) {
        let offset = (GPSET0 + gpio_pin_available.to_bcm_gpio_pin_number() as usize) / 32;
        let shift = gpio_pin_available.to_bcm_gpio_pin_number() % 32;

        self.write(offset, 1 << shift);
    }


    /// Set the pin to low
    #[inline(always)]
    pub fn set_low(& mut self, gpio_pin_available : GpioPinAvailable) {
        let offset = (GPCLR0 + gpio_pin_available.to_bcm_gpio_pin_number() as usize)/ 32;
        let shift = gpio_pin_available.to_bcm_gpio_pin_number() % 32;

        self.write(offset, 1 << shift);
    }


    //TODO check that gpio pull up pull down is not needed, maybe check rppal to see if they use it for pin
}

impl Drop for GpioController {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.mem_ptr as *mut c_void, GPIO_MEM_SIZE as size_t);
        }
    }
}
