use embedded_hal::digital::v2::OutputPin;

pub struct Dummy;

impl OutputPin for Dummy{
    type Error = ();

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}