use core::convert::Infallible;
use embedded_hal::digital::{ErrorType, OutputPin};

pub struct Dummy;

impl ErrorType for Dummy {
    type Error = Infallible;
}

impl OutputPin for Dummy {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
