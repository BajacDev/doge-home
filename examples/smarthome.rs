// klee exemple using door lock

#![no_main]

use klee_sys::*;
use std::panic;

extern crate doge_home;
pub use doge_home::devices::doorlock::DoorLock;
pub use doge_home::smarthome::SmartHome;
pub use doge_home::event::Event;

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
    klee_assume(choice < 3);

    let event = match choice {
      0 => Event::None,
      1 => Event::KeyPressed,
      _ => Event::TcpEnd,
    };

    smarthome.process_event(event);
}
