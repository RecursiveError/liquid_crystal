//! # liquidCrystal
//! a library to work with alphanumeric lcd display compatible with the HD44780 controller

#![no_std]

pub mod interfaces;
pub mod fast_config;

pub use interfaces::*;
pub use fast_config::*;
use embedded_hal::blocking::delay::DelayUs;

/// Enum of possible commands for SendType::Command
#[repr(u8)]
pub enum Commands{
    Clear = 0x01,
    Reset = 0x02,
    LiquidCristalOff = 0x08,
    ShiftCursotLeft = 0x10,
    ShiftCursotRight = 0x14,
    ShiftDisplayLeft = 0x18,
    ShiftDisplayRight = 0x1C,
    CursorOn = 0x0E,
    CursorOff = 0x0C,
    CursorBlink = 0x0F,
    MoveLine1 = 0x80,
    MoveLine2 = 0xC0,

}
pub use Commands::*;
///enum of possible values ​​that can be written with the "write" function
pub enum SendType<'s>{
    Command(Commands),
    Text(&'s str),
    CustomChar(u8),
}



pub struct LiquidCristal<'interface, T> 
where 
    T: Interface,  
    {
    interface: &'interface mut T,
}

impl<'interface,T> LiquidCristal<'interface,T> 
    where 
        T: Interface, 
    {
    ///create a new display
    pub fn new(interface: &'interface mut T) -> LiquidCristal<'interface, T>{
        LiquidCristal{interface}
    }

    ///send data in 2 packages of 4bits
    fn send4bits<D: DelayUs<u16>>(&mut self, delay: &mut D, data:u8){
        self.interface.send(data);
        self.interface.send(data | EN);
        delay.delay_us(5); //pulse time need to be >450nS
        self.interface.send(data);
        delay.delay_us(5);
    }

    /// ### low level function to send data.
    /// processes the data before sending it to send4bits.
    /// `rs_state` represents the state of the RS pin of the display 
    /// (0x01 write) 
    /// (0x00 command)
    pub fn send<D: DelayUs<u16>>(&mut self,delay: &mut D,  data:u8, rs_state: u8){
        let high_bits = ((data) & 0xF0) | rs_state;
        let low_bits = ((data << 4) & 0xF0) | rs_state; 
        self.send4bits(delay, high_bits);
        self.send4bits(delay, low_bits);
        if rs_state != 0 {
            delay.delay_us(5); // Minimal time between consequent data writes is ~1 uS
        }else{
            delay.delay_us(2000); //generic command execution time is ~40 uS, but Clear or Reset command execution time is 1.5mS
        }
    }

    /// ### write on the display
    /// # Exemple
    /// to send Text
    /// ```
    /// write(&mut delay,Text("Text"))
    /// ```
    /// to send Command
    /// 
    /// ```
    ///  write(&mut delay,Command(Command))
    /// ```
    /// 
    /// to send custom char
    /// 
    /// ```
    ///  write(&mut delay, CustomChar(slot))
    /// ```
    /// 
    pub fn write<'s, D:DelayUs<u16>>(&mut self,delay: &mut D, data: SendType<'s>) -> &mut Self{
        match data {
            SendType::Command(x) =>{
                self.send(delay, x as u8, 0x00);
            }
            SendType::Text(x) => {
                for text in x.chars(){
                    self.send(delay, text as u8, RS);
                }
            }
            SendType::CustomChar(slot) => {
                if slot < 8 {
                    self.send(delay, slot, RS);
                }
            }
        };
        self
    }
    /// ### starts communication with the display
    pub fn init<D: DelayUs<u16>>(&mut self, delay: &mut D) -> &mut Self{
        
        delay.delay_us(150); //wait the delay init
        
        //start 4bits mode
        self.send4bits(delay, 0x03 << 4); 
        delay.delay_us(5000);
        self.send4bits(delay, 0x03 << 4);
        delay.delay_us(160);
        self.send4bits(delay, 0x03 << 4);
        delay.delay_us(160);
        self.send4bits(delay, 0x02 << 4);
        delay.delay_us(10000);
        self
    }

    /// ### moves the cursor to the indicated location.
    /// receives the line and column position and moves the cursor
    pub fn set_cursor<D: DelayUs<u16>>(&mut self, delay: &mut D,line: u8, colum: u8) -> &mut Self{
        let bits = if line == 1{
            (MoveLine1 as u8) + (0b0011_1111 & colum)
        }else{
            (MoveLine2 as u8) + (0b0011_1111 & colum)
        }; 

        self.send(delay,bits, 0x00);
        self
    }

    /// ### create custom characters
    /// attention: this function resets the internal variables of the display.
    pub fn custom_char<D: DelayUs<u16>>(&mut self, delay: &mut D, char_array: &[u8;8], slot: u8){
        if slot < 8{
            self.send(delay, 0x40 | (slot<<3) , 0x00);
            for c in 0..8{
                self.send(delay, char_array[c], RS);
            }
        }
        self.write(delay, SendType::Command(Reset));
    }
    /// ### loads the configuration struct to the display
    pub fn fast_config<D: DelayUs<u16>>(&mut self, delay: &mut D, config: FastConfig){
        self.send(delay,0b0000_0100 | (config.entry_mode.0 as u8) | (config.entry_mode.1 as u8), 0x00);
        self.send(delay,0b0000_1000 | (config.display.0 as u8) | (config.display.1 as u8)| (config.display.2 as u8), 0x00);
        self.send(delay,0b0010_0000 | (config.display_config.0 as u8) | (config.display_config.1 as u8) | (config.display_config.2 as u8), 0x00);
        self.write(delay, SendType::Command(Commands::Clear))
            .write(delay, SendType::Command(Commands::Reset));
    }
}