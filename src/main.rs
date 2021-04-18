mod smarthome;
mod bindings;
mod devices;

// use bindings::CliState::*;
use crate::devices::doorlock::*;
use crate::bindings::cli::*;

use crate::smarthome::*;

fn main() {
    let mut smart_home = SmartHome::new();
    smart_home.start();
}