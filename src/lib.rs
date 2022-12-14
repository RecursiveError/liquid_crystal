#![no_std]

pub mod interfaces;

pub use interfaces::*;
use embedded_hal::blocking::delay::DelayUs;

//TODO: set all functions code
#[repr(u8)]
pub enum Commands{
    Clear = 0x01,
    EntryMode = 0x06,
    LiquidCristalOff = 0x08,
    Fun4bits1line = 0x20,
    Fun4bits2line = 0x28,
    CursorOn = 0x0E,
    CursorOff = 0x0C,
    MoveLine1 = 0x80,
    MoveLine2 = 0xC0,

}
pub use Commands::*;

pub enum SendType<'s>{
    Command(Commands),
    Text(&'s str),
}


/*TODO
CREATE 8BITS MODE
set cursor function
*/
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
    pub fn new(interface: &'interface mut T) -> LiquidCristal<'interface, T>{
        LiquidCristal{interface}
    }

    //envia os dados em 2 pacotes de 4bits
    fn send4bits<D: DelayUs<u16>>(&mut self, delay: &mut D, data:u8){
        self.interface.send(data);
        self.interface.send(data | EN);
        delay.delay_us(5);
        self.interface.send(data);
        delay.delay_us(5);
    }

    //trata os bits antes de enviar para send4bits
    fn send<D: DelayUs<u16>>(&mut self,delay: &mut D,  data:u8, rs_state: u8){
        let high_bits = ((data) & 0xF0) | rs_state;
        let low_bits = ((data << 4) & 0xF0) | rs_state; 
        self.send4bits(delay, high_bits);
        self.send4bits(delay, low_bits);
    }

    //envia dados para o LiquidCristal
    pub fn write<'s, D:DelayUs<u16>>(&mut self,delay: &mut D, data: SendType<'s>) -> &mut Self{
        match data {
            SendType::Command(x) =>{
                self.send(delay, x as u8, 0x00);
                 delay.delay_us(2000);
            }
            SendType::Text(x) => {
                for text in x.chars(){
                    self.send(delay, text as u8, RS);
                    delay.delay_us(80);
                }
            }
        };
        self
    }

    //inicia o LiquidCristal
    pub fn init<D: DelayUs<u16>>(&mut self, delay: &mut D) -> &mut Self{
        //inicia o modo 4bits
        self.send4bits(delay, 0x03 << 4); 
        delay.delay_us(5000);
        self.send4bits(delay, 0x03 << 4);
        delay.delay_us(160);
        self.send4bits(delay, 0x03 << 4);
        delay.delay_us(160);
        self.send4bits(delay, 0x02 << 4);
        delay.delay_us(10000);

        //configura o LiquidCristal
        self.write(delay, SendType::Command(Fun4bits2line))
            .write(delay, SendType::Command(CursorOff))
            .write(delay, SendType::Command(Clear))
    }

    pub fn set_cursor<D: DelayUs<u16>>(&mut self, delay: &mut D,line: u8, colum: u8) -> &mut Self{
        let bits = if line == 1{
            (MoveLine1 as u8) & (0b0011_1111 & colum)
        }else{
            (MoveLine1 as u8) & (0b0011_1111 & colum)
        }; 

        self.send(delay,bits, 0x00);
        self
    }


}