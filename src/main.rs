mod bindings;
mod devices;
mod smarthome;

use crate::smarthome::*;

fn main() {
    let result_smart_home = SmartHome::new();
    match result_smart_home {
        Result::Ok(mut smart_home) => smart_home.start(),
        Err(_) => (),
    }
}
