/// Enum of possible commands for SendType::Command
#[repr(u8)]
pub enum Commands {
    Clear = 0x01,
    Reset = 0x02,
    ShiftCursotLeft = 0x10,
    ShiftCursotRight = 0x14,
    ShiftDisplayLeft = 0x18,
    ShiftDisplayRight = 0x1C,
    MoveLine1 = 0x80,
    MoveLine2 = 0xC0,
}
