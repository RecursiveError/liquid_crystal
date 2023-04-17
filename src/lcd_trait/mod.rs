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
    Bus8Bits,
}

enum LCDEntryMode{
    LCDShiftMode = 0x01,
    LCDDirection = 0x02,
}

enum LCDDisplayControl{
    LCDBlink = 0x01,
    LCDCursor = 0x02,
    LCDDisplay = 0x04,

}

pub struct LiquidCrystal<'interface, T:Interface, const COLS: u8, const LINES: usize>{
    interface: &'interface mut T,
    corrent_enable: u8,
    bus: BusBits,
    layout: Layout<COLS, LINES>,
    entry_mode: u8,
    display_control:u8,
}

impl<'interface, T:Interface, const COLS: u8, const LINES: usize> LiquidCrystal<'interface, T, COLS, LINES>{

    pub fn new(interface: &'interface mut T, bus:BusBits, layout: Layout<COLS, LINES>) -> LiquidCrystal<'interface, T, COLS, LINES>{
        LiquidCrystal{
            interface,
            bus,
            layout,
            corrent_enable: 0b11,
            entry_mode: 0x06, //shift Off, written from left to right
            display_control: 0x0C //display on, cursor off, cursor blinking off
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
        self.update_config(delay);
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
    pub fn custom_char<D: DelayUs<u16>>(&mut self, delay: &mut D, char_array: &[u8;8], slot: u8)->&mut Self{
        if slot < 8{
            self.send(delay, 0x40 | (slot<<3) , 0x00);
            for c in 0..8{
                self.send(delay, char_array[c], RS);
            }
        }
        self.write(delay, SendType::Command(Reset));
        self
    }
    /// ### enable all displays
    pub fn echo(&mut self) -> &mut Self{
        self.corrent_enable = 0b11;
        self
    }

    /// ### select a display
    ///0 = EN1 | 1 = EN2
    pub fn select_lcd(&mut self, en: u8)-> &mut Self{
        if en < 2 {
        self.corrent_enable = 1<<en;
        }
        self
    }

    /// ### enable blinking cursor
    /// use update_config after configuration to apply changes!
    #[inline]
    pub fn enable_blink(&mut self)-> &mut Self{
        self.display_control |= LCDDisplayControl::LCDBlink as u8;
        self
    }

    /// ### enable cursor
    /// use update_config after configuration to apply changes!
    #[inline]
    pub fn enable_cursor(&mut self)-> &mut Self{
        self.display_control |= LCDDisplayControl::LCDCursor as u8;
        self
    }

    /// ### enable display
    /// use update_config after configuration to apply changes!
    #[inline]
    pub fn enable_display(&mut self)-> &mut Self{
        self.display_control |= LCDDisplayControl::LCDDisplay as u8;
        self
    }

    /// ### enable autoscroll
    /// use update_config after configuration to apply changes!
    #[inline]
    pub fn enable_autoscroll(&mut self)-> &mut Self{
        self.entry_mode |= LCDEntryMode::LCDShiftMode as u8;
        self
    }

    /// ### disable blinking cursor
    /// use update_config after configuration to apply changes!
    #[inline]
    pub fn disable_blink(&mut self)-> &mut Self{
        self.display_control &= !(LCDDisplayControl::LCDBlink as u8);
        self
    }

    /// ### disable cursor
    /// use update_config after configuration to apply changes!
    #[inline]
    pub fn disable_cursor(&mut self)-> &mut Self{
        self.display_control &= !(LCDDisplayControl::LCDCursor as u8);
        self
    }

    /// ### disable display
    /// use update_config after configuration to apply changes!
    #[inline]
    pub fn disable_display(&mut self)-> &mut Self{
        self.display_control &= !(LCDDisplayControl::LCDDisplay as u8);
        self
    }

    /// ### disable autoscroll
    /// use update_config after configuration to apply changes!
    #[inline]
    pub fn disable_autoscroll(&mut self)-> &mut Self{
        self.entry_mode &= !(LCDEntryMode::LCDShiftMode as u8);
        self
    }

    /// ### autoscroll increments position
    /// use update_config after configuration to apply changes!
    #[inline]
    pub fn set_autoscroll_increment(&mut self)-> &mut Self{
        self.entry_mode |= LCDEntryMode::LCDDirection as u8;
        self
    }

    /// ### autoscroll decrements position
    /// use update_config after configuration to apply changes!
    #[inline]
    pub fn set_autoscroll_decrement(&mut self)-> &mut Self{
        self.entry_mode &= !(LCDEntryMode::LCDDirection as u8);
        self
    }

    /// ### send the configs to the display
    pub fn update_config<D: DelayUs<u16>>(&mut self, delay: &mut D)-> &mut Self{
        self.send(delay, self.display_control, 0);
        self.send(delay, self.entry_mode, 0);
        self
    }


}
