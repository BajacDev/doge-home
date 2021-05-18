#![no_std]
#![no_main]

use klee_sys::klee_make_symbolic;
use panic_klee as _;

extern crate doge_home;

pub use doge_home::devices::doorlock::DoorLock;

#[no_mangle]
fn main() {
    let mut a: i32 = 0;
    klee_make_symbolic!(&mut a, "a");
}
