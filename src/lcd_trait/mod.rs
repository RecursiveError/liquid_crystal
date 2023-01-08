pub use embedded_hal::blocking::delay::DelayUs;

pub mod interfaces;
pub mod fast_config;
pub mod commands;

pub use interfaces::*;
pub use fast_config::*;

pub use commands::*;
pub use commands::Commands::*;

///enum of possible values ​​that can be written with the "write" function
pub enum SendType<'s>{
    Command(Commands),
    Text(&'s str),
    CustomChar(u8),
}

pub trait LiquidCrystal{

    /// ### low level function to send data.
    /// processes the data before sending it to send4bits.
    /// `rs_state` represents the state of the RS pin of the display 
    /// (0x01 write) 
    /// (0x00 command)
   fn send<D: DelayUs<u16>>(&mut self,delay: &mut D,  data:u8, rs_state: u8);

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
    fn write<'s, D:DelayUs<u16>>(&mut self,delay: &mut D, data: SendType<'s>) -> &mut Self{
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

    /// ### moves the cursor to the indicated location.
    /// receives the line and column position and moves the cursor
    fn set_cursor<D: DelayUs<u16>>(&mut self, delay: &mut D,line: u8, colum: u8) -> &mut Self{
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
    fn custom_char<D: DelayUs<u16>>(&mut self, delay: &mut D, char_array: &[u8;8], slot: u8){
        if slot < 8{
            self.send(delay, 0x40 | (slot<<3) , 0x00);
            for c in 0..8{
                self.send(delay, char_array[c], RS);
            }
        }
        self.write(delay, SendType::Command(Reset));
    }
    /// ### loads the configuration struct to the display
    fn fast_config<D: DelayUs<u16>>(&mut self, delay: &mut D, config: FastConfig){
        self.send(delay,0b0000_0100 | (config.entry_mode.0 as u8) | (config.entry_mode.1 as u8), 0x00);
        self.send(delay,0b0000_1000 | (config.display.0 as u8) | (config.display.1 as u8)| (config.display.2 as u8), 0x00);
        self.send(delay,0b0010_0000 | (config.display_config.0 as u8) | (config.display_config.1 as u8) | (config.display_config.2 as u8), 0x00);
        self.write(delay, SendType::Command(Commands::Clear))
            .write(delay, SendType::Command(Commands::Reset));
    }
}