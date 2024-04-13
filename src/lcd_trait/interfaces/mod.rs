#![allow(unused)]

pub mod dummy;
use embedded_hal::i2c::I2c;
use embedded_hal::digital::OutputPin;

pub const EN: u8 = 0b00000100;
pub const _RW: u8 = 0b00000010; // NO READ FUNCTION
pub const RS: u8 = 0b00000001;

#[deprecated(since="0.2.0", note="This address is only valid in the LCM1602 IIC module, just type 0x27 instead")]
pub const I2C_ADDRESS: u8 = 0x27;

pub trait Interface {
    fn send(&mut self, config: u8, data: u8);
}

pub struct Parallel<D1, D2, D3, D4, RS, EN, EN2>
where
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    RS: OutputPin,
    EN: OutputPin,
    EN2: OutputPin,
{
    d1: D1,
    d2: D2,
    d3: D3,
    d4: D4,
    rs: RS,
    en: EN,
    en2: EN2,
}

impl<D1, D2, D3, D4, RS, EN, EN2> Parallel<D1, D2, D3, D4, RS, EN, EN2>
where
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    RS: OutputPin,
    EN: OutputPin,
    EN2: OutputPin,
{
    pub fn new(
        d1: D1,
        d2: D2,
        d3: D3,
        d4: D4,
        rs: RS,
        en: EN,
        en2: EN2,
    ) -> Parallel<D1, D2, D3, D4, RS, EN, EN2> {
        Parallel {
            d1,
            d2,
            d3,
            d4,
            rs,
            en,
            en2,
        }
    }
}

impl<D1, D2, D3, D4, RS, EN, EN2> Interface for Parallel<D1, D2, D3, D4, RS, EN, EN2>
where
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    RS: OutputPin,
    EN: OutputPin,
    EN2: OutputPin,
{
    fn send(&mut self, config: u8, data: u8) {
        if (data & 0b0001_0000) != 0 {
            self.d1.set_high();
        } else {
            self.d1.set_low();
        }

        if (data & 0b0010_0000) != 0 {
            self.d2.set_high();
        } else {
            self.d2.set_low();
        }

        if (data & 0b0100_0000) != 0 {
            self.d3.set_high();
        } else {
            self.d3.set_low();
        }

        if (data & 0b1000_0000) != 0 {
            self.d4.set_high();
        } else {
            self.d4.set_low();
        }

        if (config & 0b0000_0001) != 0 {
            self.rs.set_high();
        } else {
            self.rs.set_low();
        }

        if (config & 0b0000_0100) != 0 {
            self.en.set_high();
        } else {
            self.en.set_low();
        }

        if (config & 0b0000_1000) != 0 {
            self.en2.set_high();
        } else {
            self.en2.set_low();
        }
    }
}

pub struct I2C<T: I2c> {
    i2c_bus: T,
    addr: u8,
}

impl<T: I2c> I2C<T> {
    pub fn new(i2c_bus: T, addr: u8) -> I2C<T> {
        I2C { i2c_bus, addr }
    }
}

impl<T: I2c> Interface for I2C<T> {
    fn send(&mut self, config: u8, data: u8) {
        let byte = (config & 0b00000111) | (data & 0xF0) | 0x08; //ignores possible additional Enables, i2C Module does not support multiple displays
        self.i2c_bus.write(self.addr, &[byte]); //0x08 (0b0000_1000) corresponds to the display backlight in the I2C module
    }
}
