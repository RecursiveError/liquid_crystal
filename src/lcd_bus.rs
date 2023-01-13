pub use crate::lcd_trait::*;

pub struct LiquidCrystal4Bits<'interface, T:Interface>{
    interface: &'interface mut T,
    corrent_enable: u8,
}

pub struct LiquidCrystal8Bits<'interface, T:Interface>{
    interface: &'interface mut T,
    corrent_enable: u8,
    multiplex_level: u8,
}

impl<'interface, T:Interface> LiquidCrystal4Bits<'interface, T> {
   pub fn new(interface: &'interface mut T) -> LiquidCrystal4Bits<T>{
        LiquidCrystal4Bits{interface, corrent_enable: 1}
    }

    fn send4bits<D: DelayUs<u16>>(&mut self, delay: &mut D, data:u8, rs_state: u8){
        let config  = rs_state | (self.corrent_enable << 2);
        self.interface.send(config, data);
        self.interface.send(rs_state, data);
        delay.delay_us(5); //pulse time need to be >450nS
        self.interface.send(config, data);
        delay.delay_us(5);
    }

    pub fn init<D: DelayUs<u16>>(&mut self, delay: &mut D) -> &mut Self{
        
        delay.delay_us(150); //wait the delay init
        
        //start 4bits mode
        self.send4bits(delay, 0x03 << 4, 0x00); 
        delay.delay_us(5000);
        self.send4bits(delay, 0x03 << 4, 0x00);
        delay.delay_us(160);
        self.send4bits(delay, 0x03 << 4, 0x00);
        delay.delay_us(160);
        self.send4bits(delay, 0x02 << 4, 0x00);
        delay.delay_us(10000);
        self
    }

    pub fn select_display(&mut self, enable:u8){
        self.corrent_enable = 1 << (enable & 0b0000_0111); 
    }

    pub fn echo(&mut self){
        self.corrent_enable = 0x0F;
    }

}

impl<'interface, T:Interface> LiquidCrystal8Bits<'interface, T> {
    pub fn new(interface: &'interface mut T, multiplex_level: u8) -> LiquidCrystal8Bits<T>{
        LiquidCrystal8Bits{interface, corrent_enable: 1, multiplex_level}
    }
}

impl<'interface, T:Interface> LiquidCrystal for LiquidCrystal4Bits<'interface, T>{
    
    fn send<D: DelayUs<u16>>(&mut self,delay: &mut D,  data:u8, rs_state: u8){
        let high_bits = (data) & 0xF0;
        let low_bits = (data << 4) & 0xF0; 
        self.send4bits(delay, high_bits, rs_state);
        self.send4bits(delay, low_bits, rs_state); 
        if rs_state != 0 {
            delay.delay_us(5); // Minimal time between consequent data writes is ~1 uS
        }else{
            delay.delay_us(2000); //generic command execution time is ~40 uS, but Clear or Reset command execution time is 1.5mS
        }
    }
}