use core::marker::PhantomData;

pub use embedded_hal::delay::DelayNs;
#[cfg(feature="async")]
pub use embedded_hal_async::delay::DelayNs as ADelay;

pub mod commands;
pub mod interfaces;
pub mod layout;

pub use interfaces::*;
pub use layout::*;

pub use commands::Commands::*;
pub use commands::*;

///enum of possible values ​​that can be written with the "write" function
pub enum SendType<'s> {
    Command(Commands),
    Text(&'s str),
    CustomChar(u8),
}

pub enum BusBits {
    Bus4Bits,
    Bus8Bits,
}

enum LCDEntryMode {
    LCDShiftMode = 0x01,
    LCDDirection = 0x02,
}

enum LCDDisplayControl {
    LCDBlink = 0x01,
    LCDCursor = 0x02,
    LCDDisplay = 0x04,
}

pub struct Blocking;
#[cfg(feature="async")]
pub struct Async;

pub struct LiquidCrystal<'interface, T: Interface, const COLS: u8, const LINES: usize, MODE = Blocking> {
    interface: &'interface mut T,
    corrent_enable: u8,
    bus: BusBits,
    layout: Layout<COLS, LINES>,
    entry_mode: u8,
    display_control: u8,
    _mode: PhantomData<MODE>,
}

impl<'interface, T: Interface, const COLS: u8, const LINES: usize>
    LiquidCrystal<'interface, T, COLS, LINES>
{
    pub fn new(
        interface: &'interface mut T,
        bus: BusBits,
        layout: Layout<COLS, LINES>,
    ) -> LiquidCrystal<'interface, T, COLS, LINES> {
        LiquidCrystal {
            interface,
            bus,
            layout,
            corrent_enable: 0b11,
            entry_mode: 0x06,      //shift Off, written from left to right
            display_control: 0x0C, //display on, cursor off, cursor blinking off
            _mode: PhantomData,
        }
    }

    #[cfg(feature="async")]
    pub fn asynch(self) -> LiquidCrystal<'interface, T, COLS, LINES, Async> {
        LiquidCrystal {
            interface: self.interface,
            bus: self.bus,
            layout: self.layout,
            corrent_enable: self.corrent_enable,
            entry_mode: self.entry_mode,
            display_control: self.display_control,
            _mode: PhantomData,
        }
    }

    fn send8bits(&mut self, delay: &mut impl DelayNs, data: u8, rs_state: u8) {
        self.interface.send(rs_state, data);
        self.interface
            .send(rs_state | (self.corrent_enable << 2), data);
        delay.delay_us(1);
        self.interface.send(rs_state, data);
    }

    fn send4bits(&mut self, delay: &mut impl DelayNs, data: u8, rs_state: u8) {
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
    pub fn send(&mut self, delay: &mut impl DelayNs, data: u8, rs_state: u8) {
        match self.bus {
            BusBits::Bus8Bits => self.send8bits(delay, data, rs_state),
            BusBits::Bus4Bits => self.send4bits(delay, data, rs_state),
        };

        if rs_state == 1 {
            delay.delay_us(2);
        } else {
            delay.delay_us(40);
        }
    }

    pub fn begin(&mut self, delay: &mut impl DelayNs) {
        delay.delay_ms(50);
        self.send8bits(delay, 0x30, 0);
        delay.delay_us(4100);
        self.send8bits(delay, 0x30, 0);
        delay.delay_us(100);
        self.send8bits(delay, 0x30, 0);
        delay.delay_us(100);
        match self.bus {
            BusBits::Bus8Bits => self.send8bits(delay, 0x38, 0),
            BusBits::Bus4Bits => {
                self.send8bits(delay, 0x20, 0);
                self.send(delay, 0x28, 0);
            }
        };
        self.write(delay, SendType::Command(Clear));
        self.write(delay, SendType::Command(Reset));
        self.update_config(delay);
    }

    /// ### write on the display
    /// # Exemple
    /// to send Text
    /// ```ignore
    /// write(&mut delay,Text("Text"))
    /// ```
    /// to send Command
    ///
    /// ```ignore
    ///  write(&mut delay,Command(Command))
    /// ```
    ///
    /// to send custom char
    ///
    /// ```ignore
    ///  write(&mut delay, CustomChar(slot))
    /// ```
    ///
    pub fn write<'s>(&mut self, delay: &mut impl DelayNs, data: SendType<'s>) -> &mut Self {
        match data {
            SendType::Command(x) => {
                self.send(delay, x as u8, 0x00);
                delay.delay_us(2000);
            }
            SendType::Text(x) => {
                for text in x.chars() {
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
    pub fn set_cursor(&mut self, delay: &mut impl DelayNs, line: usize, colum: u8) -> &mut Self {
        if (line < LINES) || (colum < COLS) {
            self.send(delay, self.layout.addrs[line] + colum, 0);
        }
        self
    }

    /// ### create custom characters
    /// attention: this function resets the internal variables of the display.
    pub fn custom_char(
        &mut self,
        delay: &mut impl DelayNs,
        char_array: &[u8; 8],
        slot: u8,
    ) -> &mut Self {
        if slot < 8 {
            self.send(delay, 0x40 | (slot << 3), 0x00);
            for c in 0..8 {
                self.send(delay, char_array[c], RS);
            }
        }
        self.write(delay, SendType::Command(Reset));
        self
    }

    /// ### send the configs to the display
    pub fn update_config(&mut self, delay: &mut impl DelayNs) -> &mut Self {
        self.send(delay, self.display_control, 0);
        self.send(delay, self.entry_mode, 0);
        self
    }
}

#[cfg(feature="async")]
impl<'interface, T: Interface, const COLS: u8, const LINES: usize>
    LiquidCrystal<'interface, T, COLS, LINES, Async>
{
    pub fn blocking(self) -> LiquidCrystal<'interface, T, COLS, LINES, Blocking> {
        LiquidCrystal {
            interface: self.interface,
            bus: self.bus,
            layout: self.layout,
            corrent_enable: self.corrent_enable,
            entry_mode: self.entry_mode,
            display_control: self.display_control,
            _mode: PhantomData,
        }
    }

    async fn send8bits(&mut self, delay: &mut impl ADelay, data: u8, rs_state: u8) {
        self.interface.send(rs_state, data);
        self.interface
            .send(rs_state | (self.corrent_enable << 2), data);
        delay.delay_us(1).await;
        self.interface.send(rs_state, data);
    }

    async fn send4bits(&mut self, delay: &mut impl ADelay, data: u8, rs_state: u8) {
        let high_nibble = data & 0xF0;
        let low_nibble = data << 4;
        self.send8bits(delay, high_nibble, rs_state).await;
        delay.delay_us(1).await;
        self.send8bits(delay, low_nibble, rs_state).await;
    }

    /// ### low level function to send data.
    /// processes the data before sending it to send4bits.
    /// `rs_state` represents the state of the RS pin of the display
    /// (0x01 write)
    /// (0x00 command)
    pub async fn send(&mut self, delay: &mut impl ADelay, data: u8, rs_state: u8) {
        match self.bus {
            BusBits::Bus8Bits => self.send8bits(delay, data, rs_state).await,
            BusBits::Bus4Bits => self.send4bits(delay, data, rs_state).await,
        };

        if rs_state == 1 {
            delay.delay_us(2).await;
        } else {
            delay.delay_us(40).await;
        }
    }

    pub async fn begin(&mut self, delay: &mut impl ADelay) {
        delay.delay_ms(50).await;
        self.send8bits(delay, 0x30, 0).await;
        delay.delay_us(4100).await;
        self.send8bits(delay, 0x30, 0).await;
        delay.delay_us(100).await;
        self.send8bits(delay, 0x30, 0).await;
        delay.delay_us(100).await;
        match self.bus {
            BusBits::Bus8Bits => self.send8bits(delay, 0x38, 0).await,
            BusBits::Bus4Bits => {
                self.send8bits(delay, 0x20, 0).await;
                self.send(delay, 0x28, 0).await;
            }
        };
        self.write(delay, SendType::Command(Clear)).await;
        self.write(delay, SendType::Command(Reset)).await;
        self.update_config(delay).await;
    }

    /// ### write on the display
    /// # Exemple
    /// to send Text
    /// ```ignore
    /// write(&mut delay,Text("Text"))
    /// ```
    /// to send Command
    ///
    /// ```ignore
    ///  write(&mut delay,Command(Command))
    /// ```
    ///
    /// to send custom char
    ///
    /// ```ignore
    ///  write(&mut delay, CustomChar(slot))
    /// ```
    ///
    pub async fn write<'s>(&mut self, delay: &mut impl ADelay, data: SendType<'s>) -> &mut Self {
        match data {
            SendType::Command(x) => {
                self.send(delay, x as u8, 0x00).await;
                delay.delay_us(2000).await;
            }
            SendType::Text(x) => {
                for text in x.chars() {
                    self.send(delay, text as u8, RS).await;
                }
            }
            SendType::CustomChar(slot) => {
                if slot < 8 {
                    self.send(delay, slot, RS).await;
                }
            }
        };
        self
    }

    /// ### moves the cursor to the indicated location.
    /// receives the line and column position and moves the cursor
    pub async fn set_cursor(&mut self, delay: &mut impl ADelay, line: usize, colum: u8) -> &mut Self {
        if (line < LINES) || (colum < COLS) {
            self.send(delay, self.layout.addrs[line] + colum, 0).await;
        }
        self
    }

    /// ### create custom characters
    /// attention: this function resets the internal variables of the display.
    pub async fn custom_char(
        &mut self,
        delay: &mut impl ADelay,
        char_array: &[u8; 8],
        slot: u8,
    ) -> &mut Self {
        if slot < 8 {
            self.send(delay, 0x40 | (slot << 3), 0x00).await;
            for c in 0..8 {
                self.send(delay, char_array[c], RS).await;
            }
        }
        self.write(delay, SendType::Command(Reset)).await;
        self
    }

    /// ### send the configs to the display
    pub async fn update_config(&mut self, delay: &mut impl ADelay) -> &mut Self {
        self.send(delay, self.display_control, 0).await;
        self.send(delay, self.entry_mode, 0).await;
        self
    }
}

impl<'interface, T: Interface, const COLS: u8, const LINES: usize, MODE>
    LiquidCrystal<'interface, T, COLS, LINES, MODE>
{
    /// ### enable all displays
    pub fn echo(&mut self) -> &mut Self {
        self.corrent_enable = 0b11;
        self
    }

    /// ### select a display
    ///0 = EN1 | 1 = EN2
    pub fn select_lcd(&mut self, en: u8) -> &mut Self {
        if en < 2 {
            self.corrent_enable = 1 << en;
        }
        self
    }

    /// ### enable blinking cursor
    /// use update_config after configuration to apply changes!
    #[inline]
    pub fn enable_blink(&mut self) -> &mut Self {
        self.display_control |= LCDDisplayControl::LCDBlink as u8;
        self
    }

    /// ### enable cursor
    /// use update_config after configuration to apply changes!
    #[inline]
    pub fn enable_cursor(&mut self) -> &mut Self {
        self.display_control |= LCDDisplayControl::LCDCursor as u8;
        self
    }

    /// ### enable display
    /// use update_config after configuration to apply changes!
    #[inline]
    pub fn enable_display(&mut self) -> &mut Self {
        self.display_control |= LCDDisplayControl::LCDDisplay as u8;
        self
    }

    /// ### enable autoscroll
    /// use update_config after configuration to apply changes!
    #[inline]
    pub fn enable_autoscroll(&mut self) -> &mut Self {
        self.entry_mode |= LCDEntryMode::LCDShiftMode as u8;
        self
    }

    /// ### disable blinking cursor
    /// use update_config after configuration to apply changes!
    #[inline]
    pub fn disable_blink(&mut self) -> &mut Self {
        self.display_control &= !(LCDDisplayControl::LCDBlink as u8);
        self
    }

    /// ### disable cursor
    /// use update_config after configuration to apply changes!
    #[inline]
    pub fn disable_cursor(&mut self) -> &mut Self {
        self.display_control &= !(LCDDisplayControl::LCDCursor as u8);
        self
    }

    /// ### disable display
    /// use update_config after configuration to apply changes!
    #[inline]
    pub fn disable_display(&mut self) -> &mut Self {
        self.display_control &= !(LCDDisplayControl::LCDDisplay as u8);
        self
    }

    /// ### disable autoscroll
    /// use update_config after configuration to apply changes!
    #[inline]
    pub fn disable_autoscroll(&mut self) -> &mut Self {
        self.entry_mode &= !(LCDEntryMode::LCDShiftMode as u8);
        self
    }

    /// ### autoscroll increments position
    /// use update_config after configuration to apply changes!
    #[inline]
    pub fn set_autoscroll_increment(&mut self) -> &mut Self {
        self.entry_mode |= LCDEntryMode::LCDDirection as u8;
        self
    }

    /// ### autoscroll decrements position
    /// use update_config after configuration to apply changes!
    #[inline]
    pub fn set_autoscroll_decrement(&mut self) -> &mut Self {
        self.entry_mode &= !(LCDEntryMode::LCDDirection as u8);
        self
    }
}
