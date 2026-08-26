use core::ptr::addr_of;

use crate::game::color::Rgb332;
use crate::game::hardware::hardware::LCD_HEIGHT;
use crate::game::hardware::hardware::LCD_WIDTH;

use super::font::Font;
use super::font::FontFactory;
use super::render;

static mut SMALL_FONT: Option<Font> = None;
static mut ICON_FONT: Option<Font> = None;
static mut BIG_REGULAR_FONT: Option<Font> = None;
static mut BIG_BOLD_FONT: Option<Font> = None;
static mut BIG_ITALIC_FONT: Option<Font> = None;

// TODO (Release): Remove any unused fonts
#[allow(dead_code)]
pub enum FontStyle {
    Small,
    Big,
    BigBold,
    BigItalic,
    Icon,
}
impl FontStyle {
    pub fn get_glyph_dimensions(&self) -> (u8, u8) {
        match self {
            FontStyle::Small => (5, 8),
            FontStyle::Icon => (5, 8),
            FontStyle::Big => (8, 13),
            FontStyle::BigBold => (8, 13),
            FontStyle::BigItalic => (8, 13),
        }
    }
}

/// This is considered a "global" store of all fonts,
/// which get pre-loaded into RAM.
pub fn init_singleton_fonts() {
    unsafe {
        SMALL_FONT = Some(FontFactory::new_small_font());
        ICON_FONT = Some(FontFactory::new_small_icon_font());
        BIG_REGULAR_FONT = Some(FontFactory::new_big_regular_font());
        BIG_BOLD_FONT = Some(FontFactory::new_big_bold_font());
        BIG_ITALIC_FONT = Some(FontFactory::new_big_italic_font());
    }
}

/// Draws a specified string to the display buffer.
/// **NOTE: Do not use \n with centered text!**
/// TODO: Add \n support through recursion over draw_text_centered()
pub fn draw_text_centered(x: i32, y: i32, style: FontStyle, color: Rgb332, text: &str) {
    let font = get_font(style);
    let (width, _) = font.size.get_glyph_dimensions();
    let x = x - ((width * text.len()) as i32 / 2);
    font.draw_text(x, y, color, text, false);
}

/// Loads the specified font and draws the text to the display buffer.
/// If the string goes off screen there will be no wrapping,
/// the overflow characters will not be shown.
pub fn draw_text_left_aligned_nowrap(x: i32, y: i32, style: FontStyle, color: Rgb332, text: &str) {
    let font = get_font(style);
    font.draw_text(x, y, color, text, false)
}

/// Loads the specified font and draws the text to the display buffer.
/// Text will wrap if it goes off screen.
pub fn draw_text_left_aligned_wrapped(x: i32, y: i32, style: FontStyle, color: Rgb332, text: &str) {
    let font = get_font(style);
    font.draw_text(x, y, color, text, true)
}

fn get_font(style: FontStyle) -> &'static Font<'static> {
    unsafe {
        let font_opt = match style {
            FontStyle::Small => addr_of!(SMALL_FONT),
            FontStyle::Big => addr_of!(BIG_REGULAR_FONT),
            FontStyle::BigBold => addr_of!(BIG_BOLD_FONT),
            FontStyle::BigItalic => addr_of!(BIG_ITALIC_FONT),
            FontStyle::Icon => addr_of!(ICON_FONT),
        };
        &font_opt.as_ref().unwrap().as_ref().unwrap()
    }
}

/// Our standard 1-line dialog box at the bottom of the screen.
pub fn bottom_dialog_box(text: &str) {
    let height = 18usize;
    let box_x: i32 = 0;
    let box_y = LCD_HEIGHT as i32 - 24 - height as i32;
    let text_x = LCD_WIDTH as i32 / 2;
    let text_y = LCD_HEIGHT as i32 - 24 + 4 - height as i32;

    render::fill_rect(box_x, box_y, LCD_WIDTH, height, Rgb332::WHITE);
    render::fancy_border(box_x, box_y, LCD_WIDTH, height);

    draw_text_centered(text_x, text_y, FontStyle::Small, Rgb332::BLUE, text)
}

/// A larger dialog box at the bottom of the screen.
pub fn bottom_big_dialog_box_custom_color(text: &str, color: Rgb332) {
    let height = 24usize;
    let box_x: i32 = 24;
    let box_y = LCD_HEIGHT as i32 - height as i32;
    let text_x = LCD_WIDTH as i32 / 2;
    let text_y = LCD_HEIGHT as i32 + 4 + 2 - height as i32;

    render::fill_rect(box_x, box_y, LCD_WIDTH - (24 * 2), height, Rgb332::WHITE);
    render::fancy_border(box_x, box_y, LCD_WIDTH - (24 * 2), height);

    draw_text_centered(text_x, text_y, FontStyle::BigBold, color, text)
}

/// Draws a dialog box over the entire screen.
/// Useful for menus or info dumps.
pub fn full_dialog_box(title: &str, text: &str) {
    let title_width = 8 * title.len() as i32;
    let title_x = 64 - (title_width / 2);

    render::flood(Rgb332::WHITE);
    render::fancy_border(0, 0, LCD_WIDTH, LCD_HEIGHT);

    draw_text_left_aligned_nowrap(title_x, 5, FontStyle::BigBold, Rgb332::BLACK, title);
    draw_text_left_aligned_nowrap(5, 18, FontStyle::Small, Rgb332::BLUE, text);
}
