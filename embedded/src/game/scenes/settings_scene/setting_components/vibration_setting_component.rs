use crate::game::color::Rgb332;
use crate::game::display::text_writer;
use crate::game::display::text_writer::FontStyle;
use crate::game::hardware::hardware::LCD_WIDTH;
use crate::game::nvm::settings::SettingType;
use crate::game::setting_value::Setting;

use super::adjust_setting;
use super::check_if_confirming;
use super::check_if_exiting;
use super::SettingComponentTrait;

pub struct VibrationSettingComponent {
    pub will_be_deselected: bool,
    pub setting: Setting,
    pub initial_value: u8,
    preview_timer: u8,
}

impl Default for VibrationSettingComponent {
    fn default() -> Self {
        let nvm = crate::game::globals::get_nvm();
        let setting = nvm.settings.get_setting(SettingType::Vibration);
        let initial_value = setting.get_value();
        Self {
            will_be_deselected: false,
            setting,
            initial_value,
            preview_timer: 0,
        }
    }
}

impl SettingComponentTrait for VibrationSettingComponent {
    fn draw(&mut self, y_offset: i32, _selected: bool) {
        text_writer::draw_text_centered(
            LCD_WIDTH as i32 / 2,
            y_offset,
            FontStyle::Small,
            Rgb332::BLACK,
            "VIBRATION",
        );
        let value = self.setting.get_value();
        if value == 0 {
            text_writer::draw_text_left_aligned_nowrap(
                24,
                y_offset + 8,
                FontStyle::Icon,
                Rgb332::BLUE,
                "44444444        ",
            );
            text_writer::draw_text_left_aligned_nowrap(
                24,
                y_offset + 8,
                FontStyle::Small,
                Rgb332::WHITE,
                "  off           ",
            );
            text_writer::draw_text_left_aligned_nowrap(
                24,
                y_offset + 8,
                FontStyle::Small,
                Rgb332::LIGHT_GREY,
                "            on  ",
            );
        } else {
            text_writer::draw_text_left_aligned_nowrap(
                24,
                y_offset + 8,
                FontStyle::Icon,
                Rgb332::BLUE,
                "        44444444",
            );
            text_writer::draw_text_left_aligned_nowrap(
                24,
                y_offset + 8,
                FontStyle::Small,
                Rgb332::LIGHT_GREY,
                "  off           ",
            );
            text_writer::draw_text_left_aligned_nowrap(
                24,
                y_offset + 8,
                FontStyle::Small,
                Rgb332::WHITE,
                "            on  ",
            );
        }
    }

    fn tick(&mut self) {
        if self.preview_timer > 0 {
            crate::game::globals::get_hardware().start_vibrating();
            self.preview_timer -= 1;
        } else {
            crate::game::globals::get_hardware().stop_vibrating();
        }
    }

    fn input(&mut self) {
        let nvm = crate::game::globals::get_nvm();
        if check_if_confirming() {
            self.will_be_deselected = true;
        } else if check_if_exiting() {
            nvm.settings
                .set_value(SettingType::Vibration, self.initial_value);
            self.will_be_deselected = true;
        } else {
            adjust_setting(&mut self.setting);
            nvm.settings
                .set_value(SettingType::Vibration, self.setting.get_value());
            let hardware = crate::game::globals::get_hardware();
            if hardware.key0_pressed()
                || hardware.key1_pressed()
                || hardware.key2_pressed()
                || hardware.key3_pressed()
            {
                if self.setting.get_value() == 1 {
                    self.preview_timer = 22; // 1 second
                }
            }
        }
    }

    fn is_deselected(&mut self) -> bool {
        self.will_be_deselected
    }

    fn reset_internal_state(&mut self) {
        *self = Self::default();
    }
}
