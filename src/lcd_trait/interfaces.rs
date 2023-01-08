#![allow(unused)]
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::blocking::i2c::Write;

pub const EN:u8 = 0b00000100;
pub const _RW:u8 = 0b00000010;// NO READ FUNCTION
pub const RS:u8 = 0b00000001;
pub const I2C_ADDRESS:u8 = 0x27;

pub trait Interface{
    fn send(&mut self, config: u8, data:u8);
}

/*
pub struct Parallel<D1,D2,D3,D4,RS,EN>
    where
        D1: OutputPin,
        D2: OutputPin,
        D3: OutputPin,
        D4: OutputPin,
        RS: OutputPin,
        EN: OutputPin,
    {
    d1: D1,
    d2: D2,
    d3: D3,
    d4: D4,
    rs: RS,
    en: EN,
}

impl<D1,D2,D3,D4,RS,EN> Parallel<D1,D2,D3,D4,RS,EN>
    where
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    RS: OutputPin,
    EN: OutputPin,
    {
    pub fn new(d1: D1, d2: D2,d3: D3, d4:D4, rs:RS, en: EN)-> Parallel<D1,D2,D3,D4,RS,EN>{
        Parallel{
            d1,
            d2,
            d3,
            d4,
            rs,
            en,
        }
    }
}

impl<D1,D2,D3,D4,RS,EN> Interface for Parallel<D1,D2,D3,D4,RS,EN>
where
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    RS: OutputPin,
    EN: OutputPin,
    {
        fn send(&mut self, data:u8) {
            let data_value = (data & 0xF0) >> 4;
            let en_std = data & EN;
            let rs_std = data & RS;

            //check Regista Select pin
            if rs_std != 0{
                self.rs.set_high();
            }else{
                self.rs.set_low();
            }
            
            //check data pins
            if (data_value & (0b0001)) != 0{
                self.d1.set_high();
            }else{
                self.d1.set_low();
            }

            if (data_value & (0b0010)) != 0{
                self.d2.set_high();
            }else{
                self.d2.set_low();
            }

            if (data_value & (0b0100)) != 0{
                self.d3.set_high();
            }else{
                self.d3.set_low();
            }

            if (data_value & (0b1000)) != 0{
                self.d4.set_high();
            }else{
                self.d4.set_low();
            }

            //check enable pin
            if en_std != 0{
                self.en.set_high();
            }else{
                self.en.set_low();
            }
        }

    }*/

pub struct I2C<T: Write>{
    i2c_bus: T,
    addr: u8,
}

impl<T: Write> I2C<T> {
    pub fn new(i2c_bus:T, addr: u8) -> I2C<T>{
        I2C {i2c_bus, addr}
    }
}

impl<T:Write> Interface for I2C<T> {
    fn send(&mut self, config: u8, data:u8) {
        let byte = (config & 0b0000_0111) | data; //ignores possible additional Enables, i2C Module does not support multiple displays
        self.i2c_bus.write(self.addr, &[byte | 0x08]); //0x08 (0b0000_1000) corresponds to the display backlight in the I2C module
    }
}