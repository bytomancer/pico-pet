/// # Not Yet Implementd Scene
/// You can access this scene currently via the main menu,
/// as it is being used as a placeholder for scenes not yet implemented.
///
/// It is an excellent place to put any debugging and test code.
use crate::game::color::Rgb332;
use crate::game::display::text_writer;
use crate::game::display::text_writer::draw_text_left_aligned_nowrap;
use crate::game::display::text_writer::FontStyle;
use crate::game::hardware::input::KeyNames;
use crate::game::hardware::rtc::real_date_time::RealDateTime;
use crate::game::hardware::rtc::real_time::RealTime;
use crate::game::hardware::rtc::RealDate;
use crate::game::scenes::SceneBehavior;
use crate::game::scenes::SceneType;

const NUM_BAT_SAMPLES: usize = 32;

pub struct NyiScene {
    next_scene: Option<SceneType>,
    running_bat_readings: [u16; NUM_BAT_SAMPLES],
    bat_reading_i: usize,
    frame_count: usize,
    fps: usize,
}
impl Default for NyiScene {
    fn default() -> Self {
        Self {
            next_scene: None,
            running_bat_readings: [0; NUM_BAT_SAMPLES],
            bat_reading_i: 0,
            frame_count: 0,
            fps: 0,
        }
    }
    frame_count: usize,
    fps: usize,
}
impl Default for NyiScene {
    fn default() -> Self {
        Self {
            next_scene: None,
            running_bat_readings: [0; NUM_BAT_SAMPLES],
            bat_reading_i: 0,
            frame_count: 0,
            fps: 0,
        }
    }
}

impl SceneBehavior for NyiScene {
    fn input(&mut self) {
        let input = crate::game::globals::get_input();
        if input.get_state(&KeyNames::Back).just_released {
            self.next_scene = Some(SceneType::Main);
        }
        if input.get_state(&KeyNames::Clock).just_released {
            self.fps = self.frame_count;
            self.frame_count = 0;
        }
        if input.get_state(&KeyNames::Clock).just_released {
            self.fps = self.frame_count;
            self.frame_count = 0;
        }
    }

    fn tick(&mut self) {
        self.frame_count += 1;
        self.frame_count += 1;
    }

    fn sound(&mut self) {
        let hardware = crate::game::globals::get_hardware();
        hardware.end_tone();
    }

    fn draw(&mut self) {
        text_writer::full_dialog_box("NOT IMPL", "");

        let input = crate::game::globals::get_input();
        if !input.get_state(&KeyNames::Clock).just_released {
            // limit checks to 1hz
            // return;
        }
        let hardware = crate::game::globals::get_hardware();
        let nvm = crate::game::globals::get_nvm();
        nvm.pet.is_hungry = false;
        let now = crate::game::globals::get_hardware().get_date_time();
        // let time_hr = now.time.hr;
        // let time_min = now.time.min;
        let (feeding_deadline_hr, feeding_deadline_min) = nvm.settings.get_feeding_deadline();
        let mut last_fed = nvm.pet.get_last_fed_date();
        last_fed.inc_by_1_day();
        // let next_feed_day = fed_day + 1; // TODO: wrap around.....
        // let next_feed_mon = fed_mon;
        // let next_feed_yr = fed_yr;

        let last_fed = RealDateTime {
            date: last_fed,
            time: RealTime {
                // Pet feeding time is advanced to the deadline
                hr: feeding_deadline_hr,
                min: feeding_deadline_min,
                sec: 0,
            },
        };
        let mut next_feed = last_fed.clone();
        next_feed.date.inc_by_1_day();

        // if now.date.year_since_2k > next_feed_yr {
        //     nvm.pet.is_hungry = true;
        // } else if now.date.month > next_feed_mon {
        //     nvm.pet.is_hungry = true;
        // } else if now.date.day_of_month > next_feed_day {
        //     nvm.pet.is_hungry = true;
        // } else if now.time.hr > feeding_deadline_hr {
        //     nvm.pet.is_hungry = true;
        // } else if now.time.min > feeding_deadline_min {
        //     nvm.pet.is_hungry = true;
        // } else if now.date.year_since_2k == next_feed_yr {
        //     if now.date.month == next_feed_mon {
        //         if now.date.day_of_month == next_feed_day {
        //             if now.time.hr == feeding_deadline_hr {
        //                 if now.time.min == feeding_deadline_min {
        //                     // It's time to feed
        //                     // TODO: this is a test to confirm the logic
        //                     nvm.pet.is_hungry = true;
        //                 }
        //             }
        //         }
        //     }
        // }

        nvm.pet.is_hungry = now > next_feed;

        let mut y = 16;
        draw_text_left_aligned_nowrap(8, y, FontStyle::Small, Rgb332::BLACK, "NOW:");
        y += 8;
        draw_text_left_aligned_nowrap(
            8,
            y,
            FontStyle::Small,
            Rgb332::BLACK,
            &fixedstr::str_format!(
                fixedstr::str32,
                "{}-{:02}-{:02} {:02}:{:02}:{:02}",
                now.date.year_since_2k as u16 + RealDate::ZERO_YEAR,
                now.date.month,
                now.date.day_of_month,
                now.time.hr,
                now.time.min,
                now.time.sec,
            ),
        );
        y += 8;
        draw_text_left_aligned_nowrap(
            8,
            y,
            FontStyle::Small,
            Rgb332::BLACK,
            &fixedstr::str_format!(fixedstr::str32, "{}", now.to_y2k_epoch()),
        );

        // y += 8;
        // draw_text(8, y, FontStyle::Small, Rgb332::BLACK, "LAST FED:");
        // y += 8;
        // draw_text(
        //     8,
        //     y,
        //     FontStyle::Small,
        //     Rgb332::BLACK,
        //     &fixedstr::str_format!(
        //         fixedstr::str32,
        //         "{}-{:02}-{:02} {:02}:{:02}:{:02}",
        //         fed_yr as u16 + RealDate::ZERO_YEAR,
        //         fed_mon,
        //         fed_day,
        //         feeding_deadline_hr,
        //         feeding_deadline_min,
        //         0
        //     ),
        // );
        // y += 8;
        // draw_text(
        //     8,
        //     y,
        //     FontStyle::Small,
        //     Rgb332::BLACK,
        //     &fixedstr::str_format!(fixedstr::str32, "{}", last_fed.to_y2k_epoch()),
        // );

        y += 8;
        draw_text_left_aligned_nowrap(8, y, FontStyle::Small, Rgb332::BLACK, "NEXT FEED DEADLINE:");
        y += 8;
        draw_text_left_aligned_nowrap(
            8,
            y,
            FontStyle::Small,
            Rgb332::BLACK,
            &fixedstr::str_format!(
                fixedstr::str32,
                "{}-{:02}-{:02} {:02}:{:02}:{:02}",
                next_feed.date.year_since_2k as u16 + RealDate::ZERO_YEAR,
                next_feed.date.month,
                next_feed.date.day_of_month,
                feeding_deadline_hr,
                feeding_deadline_min,
                0
            ),
        );
        y += 8;
        draw_text_left_aligned_nowrap(
            8,
            y,
            FontStyle::Small,
            Rgb332::BLACK,
            &fixedstr::str_format!(fixedstr::str32, "{}", next_feed.to_y2k_epoch()),
        );

        y += 8;
        draw_text_left_aligned_nowrap(8, y, FontStyle::Small, Rgb332::BLACK, "HUNGRY?:");
        y += 8;
        draw_text_left_aligned_nowrap(
            8,
            y,
            FontStyle::Small,
            Rgb332::BLACK,
            &fixedstr::str_format!(
                fixedstr::str32,
                "{}",
                if nvm.pet.is_hungry { "YES" } else { "NO" }
            ),
        );

        self.bat_reading_i += 1;
        if self.bat_reading_i >= self.running_bat_readings.len() {
            self.bat_reading_i = 0;
        }
        let vsense = hardware.get_vsense();
        self.running_bat_readings[self.bat_reading_i] = vsense;

        let mut vsense_sum: u32 = 0;
        for vs in self.running_bat_readings.iter() {
            vsense_sum += *vs as u32;
        }
        let vsense_avg = vsense_sum / NUM_BAT_SAMPLES as u32;

        y += 8;
        draw_text_left_aligned_nowrap(8, y, FontStyle::Small, Rgb332::BLACK, "BATTERY LEVEL:");
        y += 8;
        draw_text_left_aligned_nowrap(
            8,
            y,
            FontStyle::Small,
            Rgb332::BLACK,
            &fixedstr::str_format!(fixedstr::str32, "{}", vsense_avg),
        );

        y += 8;
        draw_text(
            8,
            y,
            FontStyle::Small,
            Rgb332::BLACK,
            &fixedstr::str_format!(fixedstr::str32, "FPS: {}", self.fps),
        );
    }

    fn next_scene(&self) -> &Option<SceneType> {
        &self.next_scene
    }
}
