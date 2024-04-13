// liquid_crystal ESP32 blocking example
//author: Guilherme Silva Schultz (RecursiveError)
//this example was generated with "esp-idf-template" via "cargo generate"

use esp_idf_svc::hal::delay::Delay;
use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::units::*;
use esp_idf_svc::sys::EspError;

use liquid_crystal::prelude::*;
use liquid_crystal::LiquidCrystal;
use liquid_crystal::I2C;

fn main() -> Result<(), EspError> {
    let mut delay = Delay::new(1);
    let p = Peripherals::take()?;

    let i2c_conf = I2cConfig::new().baudrate(100.kHz().into());
    let i2c_drive = I2cDriver::new(p.i2c0, p.pins.gpio21, p.pins.gpio22, &i2c_conf)?;
    let mut i2c_bus = I2C::new(i2c_drive, 0x27);
    let mut lcd = LiquidCrystal::new(&mut i2c_bus, Bus4Bits, LCD16X2);
    lcd.begin(&mut delay);
    lcd.write(&mut delay, Text("hello World!"));
    Ok(())
}
