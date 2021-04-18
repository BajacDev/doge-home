pub struct DoorLock {isOpen: bool}

impl DoorLock {

    pub fn new() -> Self {
        DoorLock {
            isOpen: false,
        }
    }

    pub fn open(&mut self) {
        self.isOpen = true;
        println!("DoorLock: open");
    }
    pub fn close(&mut self) {
        self.isOpen = false;
        println!("DoorLock: closed");
    }
    pub fn toggle(&mut self) {
        if self.isOpen {
            self.close();
        }else{
            self.open();
        }
    }
}