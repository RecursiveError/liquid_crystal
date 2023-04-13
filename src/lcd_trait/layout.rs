pub struct Layout<const COLS: u8, const LINES: usize>{
    pub addrs: [u8; LINES],
}

pub const LCD16X2: Layout<16,2> = Layout{
    addrs: [0x80, 0xC0],
};

pub const LCD20X4: Layout<20,4> = Layout{
    addrs: [0x80, 0xC0, 0x80+20, 0xC0+20],
};