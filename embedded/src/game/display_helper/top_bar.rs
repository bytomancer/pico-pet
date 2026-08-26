use core::cmp::min;

use fixedstr::str_format;

use crate::game::color::Rgb332;
use crate::game::display::render;
use crate::game::display::text_writer;
use crate::game::display::text_writer::FontStyle;
use crate::game::hardware::hardware::LCD_WIDTH;
use crate::game::nvm::inventory::MAX_JUICE;
use crate::game::nvm::inventory::MAX_RASPBERRIES;
use crate::game::nvm::inventory::MAX_TOMATOES;
use crate::game::nvm::settings::SettingType;

/// This draws our standard "top bar" for pet status.
/// At the moment, the info contained in the bar is up for change.
pub fn draw_top_bar() {
    draw_top_bar_bg();
    draw_top_bar_clock();

    if alert_is_necessary() {
        draw_top_bar_alert();
        draw_top_bar_inventory(10);
    } else {
        draw_top_bar_inventory(0);
    }
}

fn alert_is_necessary() -> bool {
    // TODO: Add conditions (sick, hungry, etc...)
    let pet = &crate::game::globals::get_nvm().pet;
    pet.is_hungry
}

fn draw_top_bar_bg() {
    render::fill_rect(0, 0, LCD_WIDTH, 8, Rgb332::DARK_GREY);
}

fn draw_top_bar_alert() {
    text_writer::draw_text_left_aligned_nowrap(0, 0, FontStyle::Icon, Rgb332::YELLOW, "!\"");
}

fn draw_top_bar_clock() {
    let hardware = crate::game::globals::get_hardware();
    let nvm = crate::game::globals::get_nvm();
    let time = hardware.get_time();
    let time_str = time.hh_mm_str();
    let mut x = LCD_WIDTH as i32 - FontStyle::Small.get_glyph_dimensions().0 as i32 * (5 + 2);
    if nvm
        .settings
        .get_setting(SettingType::UseMeridian)
        .get_value()
        == 0
    {
        x += 2 * FontStyle::Small.get_glyph_dimensions().0 as i32;
    }

    text_writer::draw_text_left_aligned_nowrap(x, 0, FontStyle::Small, Rgb332::WHITE, &time_str);
}

fn draw_top_bar_inventory(offset: i32) {
    let nvm = crate::game::globals::get_nvm();
    let inventory = &nvm.inventory;

    let tomatoes = min(MAX_TOMATOES, inventory.get_tomatoes());
    let raspberries = min(MAX_RASPBERRIES, inventory.get_raspberries());
    let juice = min(MAX_JUICE, inventory.get_juice());

    // DEBUG: max values for testing
    // let tomatoes = MAX_TOMATOES;
    // let raspberries = MAX_RASPBERRIES;
    // let juice = MAX_JUICE;

    let tomato_offset = if tomatoes > 9 { 5 } else { 0 };

    let tomato_icon = "tu";
    let x = offset;
    text_writer::draw_text_left_aligned_nowrap(x, 0, FontStyle::Icon, Rgb332::RED, tomato_icon);
    let display_tomatoes = str_format!(fixedstr::str4, "{}", tomatoes);
    let x = 12 + offset;
    let color = if tomatoes == MAX_TOMATOES {
        Rgb332::RED
    } else {
        Rgb332::WHITE
    };
    text_writer::draw_text_left_aligned_nowrap(x, 0, FontStyle::Small, color, &display_tomatoes);

    let rasp_icon = "rs";
    let x = 17 + offset + tomato_offset;
    text_writer::draw_text_left_aligned_nowrap(x, 0, FontStyle::Icon, Rgb332::RED, rasp_icon);
    let display_raspberries = str_format!(fixedstr::str4, "{}", raspberries);
    let x = 28 + offset + tomato_offset;
    let color = if raspberries == MAX_RASPBERRIES {
        Rgb332::RED
    } else {
        Rgb332::WHITE
    };
    text_writer::draw_text_left_aligned_nowrap(x, 0, FontStyle::Small, color, &display_raspberries);

    let juice_icon = "w";
    let x = 35 + offset + tomato_offset;
    text_writer::draw_text_left_aligned_nowrap(x, 0, FontStyle::Icon, Rgb332::RED, juice_icon);
    let display_juice = str_format!(fixedstr::str8, "{}ml", juice);
    let x = 43 + offset + tomato_offset;
    let color = if juice == MAX_JUICE {
        Rgb332::RED
    } else {
        Rgb332::WHITE
    };
    text_writer::draw_text_left_aligned_nowrap(x, 0, FontStyle::Small, color, &display_juice);
}
