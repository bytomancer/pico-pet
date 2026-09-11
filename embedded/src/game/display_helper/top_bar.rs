use core::cmp::min;

// use fixedstr::str_format;

use crate::game::color::Rgb332;
use crate::game::display::render;
use crate::game::display::render::fill_rect;
// use crate::game::display::render::h_dithered_line;
use crate::game::display::text_writer;
use crate::game::display::text_writer::FontStyle;
use crate::game::hardware::hardware::LCD_WIDTH;
// use crate::game::nvm::inventory::MAX_JUICE;
// use crate::game::nvm::inventory::MAX_RASPBERRIES;
use crate::game::nvm::inventory::MAX_TOMATOES;
use crate::game::nvm::settings::SettingType;

pub struct TopBar {
    frame_count: usize,
}
impl Default for TopBar {
    fn default() -> Self {
        Self { frame_count: 0 }
    }
}
impl TopBar {
    pub fn draw(&mut self) {
        self.frame_count = self.frame_count.wrapping_add(1);

        self.draw_top_bar_bg();
        self.draw_top_bar_clock();
        self.draw_top_bar_battery();

        if self.alert_is_necessary() {
            self.draw_top_bar_alert();
            self.draw_top_bar_inventory(10);
        } else {
            self.draw_top_bar_inventory(0);
        }
    }

    fn alert_is_necessary(&self) -> bool {
        // TODO: Add conditions (sick, hungry, etc...)
        let pet = &crate::game::globals::get_nvm().pet;
        pet.is_hungry
    }

    fn draw_top_bar_bg(&self) {
        render::fill_rect(0, 0, LCD_WIDTH, 8, Rgb332::DARK_GREY);
    }

    fn draw_top_bar_alert(&self) {
        text_writer::draw_text_left_aligned_nowrap(0, 0, FontStyle::Icon, Rgb332::YELLOW, "!\"");
    }

    fn draw_top_bar_battery(&self) {
        let x = LCD_WIDTH as i32 - FontStyle::Small.get_glyph_dimensions().0 as i32 * 3;
        let bat_charging = crate::game::globals::get_hardware().charge_indicator();
        if bat_charging {
            let anim_frame = (self.frame_count / 4) % 4;
            let battery_icon = match anim_frame {
                0 => "CDE",
                1 => "FGH",
                2 => "IJK",
                3 => "LMN",
                _ => "   ",
            };
            text_writer::draw_text_left_aligned_nowrap(
                x,
                0,
                FontStyle::Icon,
                Rgb332::GREEN,
                battery_icon,
            );
        } else {
            let bat_lvl_out_of_5 = crate::game::globals::get_battery().get_level().into_int();
            let bat_outline_color = match bat_lvl_out_of_5 {
                0 => Rgb332::RED,
                1 => Rgb332::YELLOW,
                _ => Rgb332::WHITE,
            };
            let bat_fill_color = match bat_lvl_out_of_5 {
                0 => Rgb332::RED, // no pixels show anyway
                1 => Rgb332::ORANGE,
                _ => Rgb332::GREEN,
            };
            let battery_icon = "@AB";
            text_writer::draw_text_left_aligned_nowrap(
                x,
                0,
                FontStyle::Icon,
                bat_outline_color,
                battery_icon,
            );
            let bar_x = x + 2;
            let bar_y = 2;
            let bar_w = bat_lvl_out_of_5 * 2;
            let bar_h = 4;
            fill_rect(bar_x, bar_y, bar_w as usize, bar_h, bat_fill_color);
        }
    }

    fn draw_top_bar_clock(&self) {
        let hardware = crate::game::globals::get_hardware();
        let nvm = crate::game::globals::get_nvm();
        let time = hardware.get_time();
        let time_str = time.hh_mm_str();

        // Battery is 3 chars wide, +1 for padding
        let battery_width = FontStyle::Icon.get_glyph_dimensions().0 as i32 * 4;
        let hh_mm_width = FontStyle::Small.get_glyph_dimensions().0 as i32 * 5;
        let use_meridians = nvm
            .settings
            .get_setting(SettingType::UseMeridian)
            .get_value()
            == 1;
        let meridian_width = if use_meridians {
            FontStyle::Small.get_glyph_dimensions().0 as i32 * 2
        } else {
            0
        };
        let x = LCD_WIDTH as i32 - hh_mm_width - battery_width - meridian_width;

        text_writer::draw_text_left_aligned_nowrap(
            x,
            0,
            FontStyle::Small,
            Rgb332::WHITE,
            &time_str,
        );
    }

    fn draw_top_bar_inventory(&self, offset: i32) {
        let nvm = crate::game::globals::get_nvm();
        let inventory = &nvm.inventory;

        let tomatoes = min(MAX_TOMATOES, inventory.get_tomatoes());
        // let raspberries = min(MAX_RASPBERRIES, inventory.get_raspberries());
        // let juice = min(MAX_JUICE, inventory.get_juice());

        // DEBUG: max values for testing
        // let tomatoes = MAX_TOMATOES;
        // let raspberries = MAX_RASPBERRIES;
        // let juice = MAX_JUICE;

        // let tomato_offset = if tomatoes > 9 { 5 } else { 0 };

        // let tomato_icon = "tu";
        // let x = offset;
        // text_writer::draw_text_left_aligned_nowrap(x, 0, FontStyle::Icon, Rgb332::RED, tomato_icon);
        // let display_tomatoes = str_format!(fixedstr::str4, "{}", tomatoes);
        // let x = 12 + offset;
        // let color = if tomatoes == MAX_TOMATOES {
        //     Rgb332::RED
        // } else {
        //     Rgb332::WHITE
        // };
        // text_writer::draw_text_left_aligned_nowrap(x, 0, FontStyle::Small, color, &display_tomatoes);

        // let rasp_icon = "rs";
        // let x = 17 + offset + tomato_offset;
        // text_writer::draw_text_left_aligned_nowrap(x, 0, FontStyle::Icon, Rgb332::RED, rasp_icon);
        // let display_raspberries = str_format!(fixedstr::str4, "{}", raspberries);
        // let x = 28 + offset + tomato_offset;
        // let color = if raspberries == MAX_RASPBERRIES {
        //     Rgb332::RED
        // } else {
        //     Rgb332::WHITE
        // };
        // text_writer::draw_text_left_aligned_nowrap(x, 0, FontStyle::Small, color, &display_raspberries);

        // let juice_icon = "w";
        // let x = 35 + offset + tomato_offset;
        // text_writer::draw_text_left_aligned_nowrap(x, 0, FontStyle::Icon, Rgb332::RED, juice_icon);
        // let display_juice = str_format!(fixedstr::str8, "{}ml", juice);
        // let x = 43 + offset + tomato_offset;
        // let color = if juice == MAX_JUICE {
        //     Rgb332::RED
        // } else {
        //     Rgb332::WHITE
        // };
        // text_writer::draw_text_left_aligned_nowrap(x, 0, FontStyle::Small, color, &display_juice);

        // for y in bar_y..(bar_y + bar_h) {
        // h_dithered_line(bar_x, y, bar_w as usize, Rgb332::YELLOW, true);
        // }
    }
}
