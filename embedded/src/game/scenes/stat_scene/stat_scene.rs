/// # Stat Screne
/// Intended to show detailed information about the pet.
/// Important to see the current hunger, happiness, health, etc.
use core::ops::Sub;

use crate::game::color::Rgb332;
use crate::game::display::render;
use crate::game::display::text_writer;
use crate::game::display::text_writer::FontStyle;
use crate::game::hardware::hardware::LCD_WIDTH;
use crate::game::hardware::input::KeyNames;
use crate::game::hardware::rtc::real_date_time::RealDateTime;
use crate::game::hardware::rtc::real_time::RealTime;
use crate::game::scenes::SceneBehavior;
use crate::game::scenes::SceneType;

pub struct StatScene {
    next_scene: Option<SceneType>,
}

impl Default for StatScene {
    fn default() -> Self {
        Self { next_scene: None }
    }
}

impl SceneBehavior for StatScene {
    fn input(&mut self) {
        let input = crate::game::globals::get_input();

        if input.get_state(&KeyNames::Back).just_released {
            self.next_scene = Some(SceneType::Main);
        }
    }

    fn tick(&mut self) {}

    fn sound(&mut self) {}

    fn draw(&mut self) {
        let nvm = crate::game::globals::get_nvm();
        // let hardware = crate::game::globals::get_hardware();

        text_writer::full_dialog_box("Stats", "");

        let curr_hp = nvm.pet.get_health();
        let max_hp = nvm.pet.get_max_health();

        let mut height_offset = 24;

        {
            let hp_text = fixedstr::str_format!(fixedstr::str12, "HP: {} / {}", curr_hp, max_hp);
            text_writer::draw_text_left_aligned_nowrap(
                8,
                height_offset,
                FontStyle::Small,
                Rgb332::BLACK,
                &hp_text,
            );
        }

        height_offset += 8;

        {
            let full_hp_bar_width = LCD_WIDTH - 16;
            let hp_bar_width = curr_hp as usize * full_hp_bar_width / max_hp as usize;
            render::solid_line_rect(8, height_offset, full_hp_bar_width, 8, Rgb332::BLACK);
            render::fill_rect(8, height_offset, hp_bar_width, 8, Rgb332::RED);
        }

        height_offset += 8;

        {
            let header = "Last fed";
            text_writer::draw_text_left_aligned_nowrap(
                8,
                height_offset,
                FontStyle::Small,
                Rgb332::BLACK,
                header,
            );
        }

        height_offset += 8;

        {
            // TODO: move to a helper
            let last_fed = nvm.pet.get_last_fed_date();
            let feed_deadline_raw = nvm.settings.get_feeding_deadline();
            let feed_deadline_time = RealTime::new(feed_deadline_raw.0, feed_deadline_raw.1, 0);
            let feed_deadline_datetime = RealDateTime::new(feed_deadline_time, last_fed);
            let data = feed_deadline_datetime.to_fixed_str();
            text_writer::draw_text_left_aligned_nowrap(
                8,
                height_offset,
                FontStyle::Small,
                Rgb332::BLACK,
                &data,
            );
        }

        height_offset += 8;

        {
            let header = "Next feeding";
            text_writer::draw_text_left_aligned_nowrap(
                8,
                height_offset,
                FontStyle::Small,
                Rgb332::BLACK,
                header,
            );
        }

        height_offset += 8;

        {
            let last_fed = nvm.pet.get_feeding_deadline();
            let data = last_fed.to_fixed_str();
            text_writer::draw_text_left_aligned_nowrap(
                8,
                height_offset,
                FontStyle::Small,
                Rgb332::BLACK,
                &data,
            );
        }

        height_offset += 8;

        {
            let header = "Next warning";
            text_writer::draw_text_left_aligned_nowrap(
                8,
                height_offset,
                FontStyle::Small,
                Rgb332::BLACK,
                header,
            );
        }

        height_offset += 8;

        {
            let mut feed_warning = nvm.pet.get_feeding_deadline();
            feed_warning.dec_by_1_hour();
            let data = feed_warning.to_fixed_str();
            text_writer::draw_text_left_aligned_nowrap(
                8,
                height_offset,
                FontStyle::Small,
                Rgb332::BLACK,
                &data,
            );
        }

        height_offset += 8;

        {
            let header = "Now";
            text_writer::draw_text_left_aligned_nowrap(
                8,
                height_offset,
                FontStyle::Small,
                Rgb332::BLACK,
                header,
            );
        }

        height_offset += 8;

        {
            let now = crate::game::globals::get_hardware().get_date_time();
            let data = now.to_fixed_str();
            text_writer::draw_text_left_aligned_nowrap(
                8,
                height_offset,
                FontStyle::Small,
                Rgb332::BLACK,
                &data,
            );
        }

        height_offset += 8;

        {
            let header = "Time until next feeding";
            text_writer::draw_text_left_aligned_nowrap(
                8,
                height_offset,
                FontStyle::Small,
                Rgb332::BLACK,
                header,
            );
        }

        height_offset += 8;

        {
            let now = crate::game::globals::get_hardware().get_date_time();
            let feeding_deadline = nvm.pet.get_feeding_deadline();
            let diff = feeding_deadline.sub(now);
            let data = diff.to_str();
            text_writer::draw_text_left_aligned_nowrap(
                8,
                height_offset,
                FontStyle::Small,
                Rgb332::BLACK,
                &data,
            );
        }
    }

    fn next_scene(&self) -> &Option<SceneType> {
        &self.next_scene
    }
}
