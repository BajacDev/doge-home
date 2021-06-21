// klee exemple using door lock

#![no_main]

use klee_sys::*;
use std::panic;

extern crate doge_home;
pub use doge_home::devices::doorlock::DoorLock;
pub use doge_home::smarthome::SmartHome;
pub use doge_home::event::Event;

const MAX_TCP_BUFFER_SIZE: usize = 4;

use std::os::raw;

// inspired by https://gitlab.henriktjader.com/pln/klee-sys/-/blob/master/src/lib_klee_analysis.rs
extern "C" {
  fn klee_make_symbolic(data: *mut raw::c_void, length: usize, name: *const raw::c_char);
}


#[no_mangle]
fn main() {
    panic::set_hook(Box::new(|_| {
        klee_abort!();
    }));

    let mut smarthome = SmartHome::new_fake();

    let mut is_open: bool = false;
    klee_make_symbolic!(&mut is_open, "doorlock");
    smarthome.doorlock.is_open = is_open;

    let mut choice: u32 = 0;
    klee_make_symbolic!(&mut choice, "choice");
    klee_assume(choice < 4);

    let event = match choice {
      0 => Event::None,
      1 => Event::KeyPressed,
      2 => Event::TcpEnd,
      _ => {
        let mut size: usize = 0;
        klee_make_symbolic!(&mut size, "size");
        klee_assume(size < MAX_TCP_BUFFER_SIZE);
        let v: [u8; MAX_TCP_BUFFER_SIZE] = [0; MAX_TCP_BUFFER_SIZE];
        unsafe {
          klee_make_symbolic(
            v.as_ptr() as *mut u32 as *mut raw::c_void,
            MAX_TCP_BUFFER_SIZE, 
            "vec".as_ptr() as *const raw::c_char
          );
        }
        //klee_make_symbolic!(&mut v, "vec");
        Event::TcpRead(size, v.to_vec())
      }
    };

    smarthome.process_event(event);
}
