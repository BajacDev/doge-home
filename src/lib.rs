pub mod bindings;
pub mod devices;
pub mod event;
pub mod smarthome;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
