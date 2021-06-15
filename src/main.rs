mod bindings;
mod devices;
mod smarthome;

use crate::smarthome::SmartHome;

fn main() {
    SmartHome::new().start();
}
