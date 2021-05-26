/// TOOD just to start working on outputpin withotut error
#[derive(Debug)]
pub struct Pin {
    pub(crate) pin: u8,
}

#[derive(Debug)]
pub struct OutputPin {
    pin: Pin,
    pud_mode: PullUpDown,
}