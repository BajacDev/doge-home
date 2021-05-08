mod bindings;
mod devices;
mod smarthome;

use crate::smarthome::*;

fn main() {
    let mut smart_home = SmartHome::new();
    smart_home.start();
}
