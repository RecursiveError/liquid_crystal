#[repr(u8)]
pub enum ShiftConfig {
    Increment = 0b0000_0010,
    Decrement = 0,
}

#[repr(u8)]
pub enum ShiftState {
    On = 0b0000_0001,
    Off = 0,
}

#[repr(u8)]
pub enum Display {
    On = 0b0000_0100,
    Off = 0,
}

#[repr(u8)]
pub enum Cursor {
    On = 0b0000_0010,
    Off = 0,
}

#[repr(u8)]
pub enum Blink {
    On = 0b0000_0001,
    Off = 0,
}

#[repr(u8)]
pub enum ShiftMode {
    ShiftCursor = 0,
    ShiftDisplay= 0b0000_1000,
}

#[repr(u8)]
pub enum ShiftDirection{
    Left = 0,
    Right = 0b0000_0100,
}

#[repr(u8)]
pub enum Bits{
    Four = 0,
}

#[repr(u8)]
pub enum DisplayLines {
    One = 0,
    Two = 0b0000_1000,
}

#[repr(u8)]
pub enum CharSize {
    C5X10 = 0b0000_0100,
    C5X8 = 0,
}

pub struct FastConfig{
    pub entry_mode: (ShiftConfig, ShiftState),
    pub display: (Display, Cursor, Blink),
    pub display_config: (Bits,DisplayLines,CharSize),
    pub write_config: (ShiftMode, ShiftDirection)
}

pub const DEFALT_CONFIG:FastConfig = FastConfig{
    entry_mode: (ShiftConfig::Decrement, ShiftState::Off),
    display: (Display::On, Cursor::On, Blink::On),
    display_config: (Bits::Four, DisplayLines::Two, CharSize::C5X8),
    write_config: (ShiftMode::ShiftCursor, ShiftDirection::Left),
};
