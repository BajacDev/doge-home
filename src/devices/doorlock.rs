pub struct DoorLock {is_open: bool}

impl DoorLock {

    pub fn new() -> Self {
        DoorLock {
            is_open: false,
        }
    }

    pub fn open(&mut self) {
        self.is_open = true;
        println!("DoorLock: open");
    }
    pub fn close(&mut self) {
        self.is_open = false;
        println!("DoorLock: closed");
    }
    pub fn toggle(&mut self) {
        if self.is_open {
            self.close();
        }else{
            self.open();
        }
    }
}