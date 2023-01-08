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