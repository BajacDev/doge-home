mod bindings;
mod devices;
mod smarthome;

use crate::smarthome::*;

fn main() {
    SmartHome::new().start();
}
