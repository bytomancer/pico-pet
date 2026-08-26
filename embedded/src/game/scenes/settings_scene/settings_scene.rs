/// # Settings scene
/// Allows the user to modify all the settings.
/// For settings to persist, they must be written to NVM.
use crate::game::audio::audio_library::AudioId;
use crate::game::audio::audio_player::AudioPlayer;
use crate::game::audio::audio_player::AutoPlayMode;
use crate::game::audio::audio_player::RepeatMode;
use crate::game::color::Rgb332;
use crate::game::display::text_writer::FontStyle;
use crate::game::display::text_writer::{self};
use crate::game::hardware::hardware::LCD_WIDTH;
use crate::game::hardware::input::KeyNames;
use crate::game::scenes::SceneBehavior;
use crate::game::scenes::SceneType;

use super::setting_components::SettingComponent;
use super::setting_selected::SettingSelected;

/// Height to push the first UI element down from the top of the screen.
const SETTING_HEIGHT_OFFSET: i32 = 10;
/// How many settings can fit on the screen.
const SETTINGS_ON_SCREEN_COUNT: i32 = 5;

pub struct SettingsScene {
    frame_count: u32,
    next_scene: Option<SceneType>,
    song: AudioPlayer,
    pub setting_selected: SettingSelected,
    setting_highlighted: SettingSelected,
    input_enabled: bool,

    setting_components: [SettingComponent; SettingSelected::MAX_VALUE as usize + 1],

    scroll_offset: i32,
}
impl Default for SettingsScene {
    fn default() -> Self {
        Self {
            frame_count: 0,
            next_scene: None,
            song: AudioPlayer::new(AudioId::BallGame, RepeatMode::On, AutoPlayMode::On),
            setting_selected: SettingSelected::None,
            setting_highlighted: SettingSelected::None,
            input_enabled: false,
            setting_components: generate_setting_components(),

            scroll_offset: 0,
        }
    }
}

fn generate_setting_components() -> [SettingComponent; SettingSelected::MAX_VALUE as usize + 1] {
    [
        SettingComponent::Brightness(
            super::setting_components::BrightnessSettingComponent::default(),
        ),
        SettingComponent::Volume(super::setting_components::VolumeSettingComponent::default()),
        SettingComponent::Vibration(
            super::setting_components::VibrationSettingComponent::default(),
        ),
        SettingComponent::Time(super::setting_components::TimeSettingComponent::default()),
        SettingComponent::Date(super::setting_components::DateSettingComponent::default()),
        SettingComponent::Meridian(super::setting_components::MeridianSettingComponent::default()),
        SettingComponent::PomoTime(super::setting_components::PomoTimeSettingComponent::default()),
        SettingComponent::ShortRest(
            super::setting_components::ShortRestSettingComponent::default(),
        ),
        SettingComponent::LongRest(super::setting_components::LongRestSettingComponent::default()),
        SettingComponent::PomoCycle(
            super::setting_components::PomoCycleSettingComponent::default(),
        ),
        SettingComponent::FeedingDeadline(
            super::setting_components::FeedingDeadlineSettingComponent::default(),
        ),
        SettingComponent::Reset(super::setting_components::ResetSettingComponent::default()),
    ]
}

impl SceneBehavior for SettingsScene {
    fn tick(&mut self) {
        for component in self.setting_components.iter_mut() {
            component.tick();
        }
        self.frame_count += 1;
    }

    fn sound(&mut self) {
        self.song.tick();
    }

    fn draw(&mut self) {
        let title = "SETTINGS";
        let menu_body = "";
        text_writer::full_dialog_box(title, menu_body);

        self.display_cursor();

        self.display_more_arrows();

        for i in 0..SETTINGS_ON_SCREEN_COUNT {
            let sel_i = i + self.scroll_offset;
            let enabled = sel_i == self.setting_selected as i32;
            self.setting_components[sel_i as usize].draw(24 + 2 + 16 * i as i32, enabled);
        }
    }

    fn input(&mut self) {
        let input = crate::game::globals::get_input();

        if !self.input_enabled {
            // release all buttons to enable input
            if input.get_state(&KeyNames::Back).is_down
                || input.get_state(&KeyNames::Left).is_down
                || input.get_state(&KeyNames::Right).is_down
                || input.get_state(&KeyNames::Confirm).is_down
            {
                return;
            } else {
                self.input_enabled = true;
            }
        }

        match self.setting_selected {
            SettingSelected::None => {
                if input.get_state(&KeyNames::Back).just_released {
                    self.next_scene = Some(SceneType::Main);
                    let nvm = crate::game::globals::get_nvm();
                    nvm.settings.write();

                    return;
                }
                if self.check_for_new_setting_selected() {
                    return;
                }
                self.check_for_move_highlight();
            }

            setting => {
                self.setting_components[setting as usize].input();
                if self.setting_components[setting as usize].is_deselected() {
                    self.setting_components[setting as usize].reset();
                    self.setting_selected = SettingSelected::None;
                }
            }
        }
    }

    fn next_scene(&self) -> &Option<SceneType> {
        &self.next_scene
    }
}

impl SettingsScene {
    fn check_for_new_setting_selected(&mut self) -> bool {
        if self.setting_selected != SettingSelected::None {
            return false;
        }

        let input = crate::game::globals::get_input();

        if input.get_state(&KeyNames::Confirm).just_released {
            self.setting_selected = self.setting_highlighted.clone();
            true
        } else {
            false
        }
    }

    fn check_for_move_highlight(&mut self) {
        let input = crate::game::globals::get_input();

        if self.setting_selected != SettingSelected::None {
            return;
        }
        if input.get_state(&KeyNames::Left).is_down && input.get_state(&KeyNames::Right).is_down {
            return;
        }

        if input.get_state(&KeyNames::Right).just_pressed {
            self.setting_highlighted = self.setting_highlighted.next().clone();
            if self.setting_highlighted as usize == 0 {
                self.scroll_offset = 0;
            } else if (self.setting_highlighted as i32)
                >= self.scroll_offset + SETTINGS_ON_SCREEN_COUNT
            {
                self.scroll_offset += 1;
            }
        } else if input.get_state(&KeyNames::Left).just_pressed {
            self.setting_highlighted = self.setting_highlighted.prev().clone();
            if self.setting_highlighted as usize == SettingSelected::MAX_VALUE as usize {
                self.scroll_offset =
                    SettingSelected::MAX_VALUE as i32 - SETTINGS_ON_SCREEN_COUNT as i32 + 1;
            } else if (self.setting_highlighted as i32) < self.scroll_offset {
                self.scroll_offset -= 1;
            }
        }
    }

    fn setting_to_y_offset(&self, setting: &SettingSelected) -> i32 {
        match setting {
            SettingSelected::None => -4, // hide cursor far above screen
            _ => ((*setting as i32) - self.scroll_offset) * 2 + 1,
        }
    }

    fn display_cursor(&self) {
        let (icon, setting) = match self.setting_selected {
            SettingSelected::None => (" >", &self.setting_highlighted),
            _ => ("4}", &self.setting_selected),
        };

        let y_cursor_offset = self.setting_to_y_offset(setting);
        text_writer::draw_text_left_aligned_nowrap(
            10,
            SETTING_HEIGHT_OFFSET + 8 * 2 + 8 * y_cursor_offset + 0,
            FontStyle::Icon,
            Rgb332::RED,
            icon,
        );
    }

    fn display_more_arrows(&self) {
        let offset_cycle = [0, 1, 2, 3, 2, 1];
        let animation_frames = 5;
        let phase = self.frame_count as usize / animation_frames % offset_cycle.len();
        let cycle_position = offset_cycle[phase];
        let top_arrow_offset = SETTING_HEIGHT_OFFSET + 8 + 1 - cycle_position as i32;
        let bottom_arrow_offset = SETTING_HEIGHT_OFFSET
            + 8 * 2
            + 8 * 2 * SETTINGS_ON_SCREEN_COUNT
            + 1
            + cycle_position as i32;

        if self.scroll_offset != 0 {
            text_writer::draw_text_centered(
                LCD_WIDTH as i32 / 2,
                top_arrow_offset,
                FontStyle::Icon,
                Rgb332::BLUE,
                "kl",
            );
        }
        if self.scroll_offset
            != SettingSelected::MAX_VALUE as i32 - SETTINGS_ON_SCREEN_COUNT as i32 + 1
        {
            text_writer::draw_text_centered(
                LCD_WIDTH as i32 / 2,
                bottom_arrow_offset,
                FontStyle::Icon,
                Rgb332::BLUE,
                "ij",
            );
        }
    }
}
