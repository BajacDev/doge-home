// klee exemple using door lock

#![no_main]

use klee_sys::*;
use std::panic;

// the crate is not used here
// this is a test to proves it compiles
extern crate doge_home;
pub use doge_home::devices::doorlock::DoorLock;

#[no_mangle]
fn main() {
    panic::set_hook(Box::new(|_| {
        klee_abort!();
    }));

    let mut a: i32 = 1;
    klee_make_symbolic!(&mut a, "a");

    if a == 0 {
        panic!("{:?}", a);
    }
}
