pub use embedded_hal::blocking::delay::DelayUs;
pub use embedded_hal::blocking::delay::DelayMs;

pub mod interfaces;
pub mod commands;
pub mod layout;

pub use interfaces::*;
pub use layout::*;

pub use commands::*;
pub use commands::Commands::*;

///enum of possible values ​​that can be written with the "write" function
pub enum SendType<'s>{
    Command(Commands),
    Text(&'s str),
    CustomChar(u8),
}

pub enum BusBits{
    Bus4Bits,
    Bus8Bits
}

pub struct LiquidCrystal<'interface, T:Interface, const COLS: u8, const LINES: usize>{
    interface: &'interface mut T,
    corrent_enable: u8,
    bus: BusBits,
    layout: Layout<COLS, LINES>,
}

impl<'interface, T:Interface, const COLS: u8, const LINES: usize> LiquidCrystal<'interface, T, COLS, LINES>{

    pub fn new(interface: &'interface mut T, bus:BusBits, layout: Layout<COLS, LINES>) -> LiquidCrystal<'interface, T, COLS, LINES>{
        LiquidCrystal{
            interface,
            bus,
            layout,
            corrent_enable: 0b11,
        }
    }
    fn send8bits<D: DelayUs<u16>>(&mut self,delay: &mut D,  data:u8, rs_state: u8){
        self.interface.send(rs_state, data);
        self.interface.send(rs_state | (self.corrent_enable << 2), data);
        delay.delay_us(1);
        self.interface.send(rs_state, data);
    }

    fn send4bits<D: DelayUs<u16>>(&mut self,delay: &mut D,  data:u8, rs_state: u8){
        let high_nibble = data & 0xF0;
        let low_nibble = data << 4;
        self.send8bits(delay, high_nibble, rs_state);
        delay.delay_us(1);
        self.send8bits(delay, low_nibble, rs_state);
    }

    /// ### low level function to send data.
    /// processes the data before sending it to send4bits.
    /// `rs_state` represents the state of the RS pin of the display
    /// (0x01 write)
    /// (0x00 command)
    pub fn send<D: DelayUs<u16>>(&mut self,delay: &mut D,  data:u8, rs_state: u8){
        match self.bus {
            BusBits::Bus8Bits => self.send8bits(delay, data, rs_state),
            BusBits::Bus4Bits => self.send4bits(delay, data, rs_state),
        };

        if rs_state == 1 {
            delay.delay_us(2);
        }
        else{
            delay.delay_us(40);
        }
    }

    pub fn begin<D: DelayUs<u16> + DelayMs<u16>>(&mut self, delay: &mut D){
        delay.delay_ms(50);
        self.send8bits(delay,0x30, 0);
        delay.delay_us(4100);
        self.send8bits(delay, 0x30, 0);
        delay.delay_us(100);
        self.send8bits(delay, 0x30, 0);
        delay.delay_us(100);
        match self.bus {
            BusBits::Bus8Bits => self.send8bits(delay, 0x38,0),
            BusBits::Bus4Bits => {
                self.send8bits(delay, 0x20 , 0);
                self.send(delay, 0x28 , 0);
            },
        };
        self.write(delay, SendType::Command(Clear));
        self.write(delay, SendType::Command(Reset));
        self.send(delay, 0x06, 0);
        self.send(delay, 0x0C, 0);
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
                delay.delay_us(2000);
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
    pub fn set_cursor<D: DelayUs<u16>>(&mut self, delay: &mut D,line: usize, colum: u8) -> &mut Self{
        if(line < LINES) || (colum < COLS){
            self.send(delay, self.layout.addrs[line] + colum, 0);
        }
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

}
