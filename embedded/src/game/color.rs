/// Rgb332 is an 8-bit color palette.
/// 3 bits of red, 3 bits of green, 2 bits of blue.
/// This gives us 256 available colors.
/// We express these colors as 0bRRR_GGG_BB.
/// ex: Red is 0b111_000_000 and yellow is 0b111_111_00.
///
/// One of the 256 available colors is reserved as invisible.
/// Whenever this color is used, it will not be drawn.
/// This color is defined as 0b111_000_11, a hot magenta.
///
/// Feel free to add constants for any color you will use in code.
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Rgb332(u8);
impl Rgb332 {
    pub const INVISIBLE: Rgb332 = Rgb332(0b111_000_11);

    pub const BLACK: Rgb332 = Rgb332(0b000_000_00);
    pub const DARK_GREY: Rgb332 = Rgb332(0b010_010_01);
    pub const LIGHT_GREY: Rgb332 = Rgb332(0b100_100_10);
    pub const WHITE: Rgb332 = Rgb332(0b111_111_11);

    pub const RED: Rgb332 = Rgb332(0b111_000_00);

    pub const GREEN: Rgb332 = Rgb332(0b000_111_00);

    pub const BLUE: Rgb332 = Rgb332(0b000_000_11);
    pub const DARKEST_BLUE: Rgb332 = Rgb332(0b000_000_01);

    pub const YELLOW: Rgb332 = Rgb332(0b111_111_00);

    pub const FANCY_BORDER_CORNER_COLOR: Rgb332 = Rgb332(0b000_000_10);
    pub const FANCY_BORDER_EDGE_COLOR: Rgb332 = Rgb332(0b000_000_11);
    pub const FANCY_BORDER_EDGE_FILL_COLOR: Rgb332 = Rgb332(0b101_101_11);

    pub fn into_usize(self) -> usize {
        self.0 as usize
    }
    pub fn into_u8(self) -> u8 {
        self.0 as u8
    }
    pub fn from_u8(value: u8) -> Self {
        Rgb332(value)
    }
    pub fn from_components(r: u8, g: u8, b: u8) -> Self {
        Rgb332((r << 5) | (g << 2) | b)
    }
    #[allow(dead_code)]
    pub fn into_components(self) -> (u8, u8, u8) {
        ((self.0 >> 5) & 0b111, (self.0 >> 2) & 0b111, self.0 & 0b11)
    }
}
