use std::error::Error;

pub trait Runnable {
    fn run(self) -> Result<(), Box<dyn Error>>;
}
