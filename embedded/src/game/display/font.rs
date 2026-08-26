use core::str::Chars;

use crate::game::color::Rgb332;
use crate::game::display::render;
use crate::game::hardware::hardware::LCD_WIDTH;

/// We currently support the following font sizes,
/// specified as SizeWxH.
/// Fonts must be black & white images with a fixed grid of the specified size.
pub enum FontSize {
    Size5x8,
    Size8x13,
}

type GlyphDimensions = (usize, usize);

impl FontSize {
    pub const SMALL_FONT_DIMENSIONS: GlyphDimensions = (5, 8);
    pub const BIG_FONT_DIMENSIONS: GlyphDimensions = (8, 13);

    pub fn get_glyph_dimensions(&self) -> GlyphDimensions {
        match *self {
            FontSize::Size5x8 => Self::SMALL_FONT_DIMENSIONS,
            FontSize::Size8x13 => Self::BIG_FONT_DIMENSIONS,
        }
    }
}

/// An object representing a font
/// Has a FontSize metadata,
/// then the data itself.
///
/// When drawing a font, glyphs must be aligned per ISO_8859_13,
/// starting from ASCII 0x20, ending at ASCII 0x7F.
/// ex: https://docs.rs/embedded-graphics/latest/embedded_graphics/mono_font/iso_8859_13/index.html
pub struct Font<'a> {
    pub size: FontSize,
    data: &'a [u8],
}

impl<'a> Font<'a> {
    pub fn new(size: FontSize, data: &'a [u8]) -> Self {
        Font { size, data }
    }

    fn extract_color(chars: &mut Chars) -> u8 {
        let r = chars.next().unwrap_or('0') as u8 - 48;
        let g = chars.next().unwrap_or('0') as u8 - 48;
        let b = chars.next().unwrap_or('0') as u8 - 48;
        (r << 5) ^ (g << 2) ^ b
    }

    /// Displays text to the screen
    /// The color argument provides a starting color,
    /// however you can actually change the color mid string using:
    /// r#"This text is \c800red\c000 but now it's black."#;
    /// Additionally, the \n newline will wrap the text.
    /// There is a `wrap` flag on this function,
    /// which when true will wrap if a glyph goes off screen.
    pub fn draw_text(&self, x0: i32, y0: i32, color: Rgb332, text: &str, wrap: bool) {
        let mut x = x0;
        let mut y = y0;
        let size = &self.size;
        let (glyph_w, glyph_h) = size.get_glyph_dimensions();
        let mut fg_color = color;
        let mut bg_color = Rgb332::INVISIBLE;
        let mut chars = text.chars();

        while let Some(c) = chars.next() {
            match c {
                '\\' => match chars.next() {
                    Some('c') => fg_color = Rgb332::from_u8(Self::extract_color(&mut chars)),
                    Some('b') => bg_color = Rgb332::from_u8(Self::extract_color(&mut chars)),
                    Some(other_char) => {
                        self.blit_glyph(x, y, other_char, bg_color, fg_color);
                        x += glyph_w as i32;
                        if wrap && x > (LCD_WIDTH - glyph_w) as i32 {
                            y += glyph_h as i32;
                            x = x0;
                        }
                    }
                    None => {}
                },
                '\n' => {
                    y += glyph_h as i32;
                    x = x0;
                }
                _ => {
                    self.blit_glyph(x, y, c, bg_color, fg_color);
                    x += glyph_w as i32;
                    if wrap && x > (LCD_WIDTH - glyph_w) as i32 {
                        y += glyph_h as i32;
                        x = x0;
                    }
                }
            }
        }
    }

    fn blit_glyph(&self, x: i32, y: i32, c: char, bg_color: Rgb332, fg_color: Rgb332) {
        match self.size {
            FontSize::Size5x8 => self.build_5x8_glyph(x, y, c, bg_color, fg_color),
            FontSize::Size8x13 => self.build_8x13_glyph(x, y, c, bg_color, fg_color),
        }
    }

    /// Returns the x & y glyph coordinates of any char.
    /// These are the glyph glyph coordinates,
    /// so you may want to multiply them by glyph size!
    fn font_lookup(c: char) -> (usize, usize) {
        let x = (c as usize - 32) % 16;
        let y = (c as usize - 32) / 16;
        (x, y)
    }

    /// Rasterizes a 5x8 glyph into a temporary RGB332 5 wide 8 tall texture,
    /// then blits the texture into the display buffer.
    fn build_5x8_glyph(&self, x: i32, y: i32, c: char, bg_color: Rgb332, fg_color: Rgb332) {
        let mut glyph = [bg_color.into_u8(); 5 * 8];
        let (glyph_x0, glyph_y0) = Self::font_lookup(c);
        let (glyph_bit_x0, glyph_bit_y0) = (glyph_x0 * 5, glyph_y0 * 8);
        for glyph_y in 0..8 {
            for glyph_x in 0..5 {
                let glyph_bit_loc = (glyph_bit_y0 + glyph_y) * 80 + (glyph_bit_x0 + glyph_x);
                let glyph_byte_loc = glyph_bit_loc / 8;
                let glyph_byte_bit_offset = glyph_bit_loc % 8;
                let src = self.data[glyph_byte_loc];
                if (0b1000_0000 >> glyph_byte_bit_offset) & src != 0b0 {
                    glyph[(glyph_y * 5) + glyph_x] = fg_color.into_u8();
                }
            }
        }
        render::blit(x, y, 5, 8, &glyph)
    }

    /// Rasterizes an 8x13 glyph into a temporary RGB332 5 wide 8 tall texture,
    /// then blits the texture into the display buffer.
    fn build_8x13_glyph(&self, x: i32, y: i32, c: char, bg_color: Rgb332, fg_color: Rgb332) {
        let mut glyph = [bg_color.into_u8(); 8 * 13];
        let (glyph_x0, glyph_y0) = Self::font_lookup(c);

        for glyph_y in 0..13 {
            let data_row = self.data[(glyph_y + glyph_y0 * 13) * 16 + glyph_x0];
            for bit_idx in 0..8 {
                if (data_row & (0b1000_0000 >> bit_idx)) != 0 {
                    glyph[glyph_y * 8 + bit_idx] = fg_color.into_u8();
                }
            }
        }

        render::blit(x, y, 8, 13, &glyph)
    }
}

/// The FontFactory is where you can load new fonts for the game.
pub struct FontFactory;
impl FontFactory {
    pub fn new_small_font() -> Font<'static> {
        Font::new(
            FontSize::Size5x8,
            include_bytes!("../../../assets/font_raw/font_5x8.data"),
        )
    }

    pub fn new_small_icon_font() -> Font<'static> {
        Font::new(
            FontSize::Size5x8,
            include_bytes!("../../../assets/font_raw/icon_font.data"),
        )
    }

    pub fn new_big_regular_font() -> Font<'static> {
        Font::new(
            FontSize::Size8x13,
            include_bytes!("../../../assets/font_raw/font_8x13.data"),
        )
    }

    pub fn new_big_bold_font() -> Font<'static> {
        Font::new(
            FontSize::Size8x13,
            include_bytes!("../../../assets/font_raw/font_8x13_bold.data"),
        )
    }

    pub fn new_big_italic_font() -> Font<'static> {
        Font::new(
            FontSize::Size8x13,
            include_bytes!("../../../assets/font_raw/font_8x13_italic.data"),
        )
    }
}
