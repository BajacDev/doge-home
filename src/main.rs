mod smarthome;
mod bindings;
mod devices;

use crate::smarthome::*;

fn main() {
    let mut smart_home = SmartHome::new();
    smart_home.start();
}