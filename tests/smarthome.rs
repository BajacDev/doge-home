use proptest::prelude::*;

extern crate doge_home;
pub use doge_home::devices::doorlock::DoorLock;
pub use doge_home::smarthome::SmartHome;
pub use doge_home::event::Event;

proptest! {
  #[test]
  fn test(choice in 0..3i32, b: bool) {
    let mut smarthome = SmartHome::new_fake();

    smarthome.doorlock.is_open = b;

    let event = match choice {
      0 => Event::None,
      1 => Event::KeyPressed,
      _ => Event::TcpEnd,
    };

    smarthome.process_event(event);
  }
}
