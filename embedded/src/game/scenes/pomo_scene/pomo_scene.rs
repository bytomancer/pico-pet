/// Pomodoro Scene
/// This is the scene the user interacts with to complete pomodoros.
/// It's another excellent place for the pets to show simple animations,
/// without being overly distracting,
/// but ultimately builds character for the pet.
///
/// This is one of the most fleshed out scenes currently,
/// but the animation in the center of the screen is a placeholder.
/// Mechanically, the scene works exactly as I invisioned it.
use crate::game::audio::audio_library::AudioId;
use crate::game::audio::audio_player::AudioPlayer;
use crate::game::audio::audio_player::AutoPlayMode;
use crate::game::audio::audio_player::RepeatMode;
use crate::game::color::Rgb332;
use crate::game::display::sprite::Sprite;
use crate::game::display::sprite_factory;
use crate::game::display::text_writer;
use crate::game::display::text_writer::FontStyle;
use crate::game::hardware::hardware::LCD_HEIGHT;
use crate::game::hardware::hardware::LCD_WIDTH;
use crate::game::hardware::input::KeyNames;
use crate::game::nvm::settings::SettingType;
use crate::game::scenes::SceneBehavior;
use crate::game::scenes::SceneType;

use super::timer::TimerEvent;

use super::timer::PomoTimer;

const ANIM_ON_RATE: usize = 3;

pub struct PomoScene<'a> {
    menu_sprite: Sprite<'a>,
    lofi_sprite: Sprite<'a>,
    next_scene: Option<SceneType>,
    timer: PomoTimer,
    pomo_finished_track: AudioPlayer,
    break_finished_track: AudioPlayer,
    countdown_321_track: AudioPlayer,
    countdown_go_track: AudioPlayer,
    frame_count: usize,
}
impl Default for PomoScene<'static> {
    fn default() -> Self {
        Self {
            next_scene: None,
            menu_sprite: sprite_factory::new_pomo_menu_sprite(0, 0),
            lofi_sprite: sprite_factory::new_lofi_sprite(0, 8),
            timer: PomoTimer::default(),
            pomo_finished_track: AudioPlayer::new(
                AudioId::PomodoroFinished,
                RepeatMode::Off,
                AutoPlayMode::Off,
            ),
            break_finished_track: AudioPlayer::new(
                AudioId::BreakFinished,
                RepeatMode::Off,
                AutoPlayMode::Off,
            ),
            countdown_321_track: AudioPlayer::new(
                AudioId::Countdown321,
                RepeatMode::Off,
                AutoPlayMode::Off,
            ),
            countdown_go_track: AudioPlayer::new(
                AudioId::CountdownGo,
                RepeatMode::Off,
                AutoPlayMode::Off,
            ),
            frame_count: 0,
        }
    }
}

pub enum PomoMenuFrame {
    Play = 0,
    Pause = 1,
    Exit = 2,
    Stop = 3,
}

impl PomoScene<'_> {
    pub fn start_pomo_sound(&mut self) {
        self.pomo_finished_track.play()
    }
    pub fn start_break_sound(&mut self) {
        self.break_finished_track.play()
    }
    pub fn is_playing_alert(&self) -> bool {
        self.pomo_finished_track.is_playing() || self.break_finished_track.is_playing()
    }
}

impl SceneBehavior for PomoScene<'_> {
    fn input(&mut self) {
        if self.is_playing_alert() {
            return;
        }
        let input = crate::game::globals::get_input();
        if input.get_state(&KeyNames::Back).just_released {
            self.timer.back_pressed();
            if self.timer.is_exiting {
                self.next_scene = Some(SceneType::Main);
            }
        } else if input.get_state(&KeyNames::Confirm).just_released {
            self.timer.confirm_pressed();
        }
    }

    fn tick(&mut self) {
        self.frame_count += 1;
        if self.frame_count > sprite_factory::LOFI_DIMENSIONS.frames * ANIM_ON_RATE {
            self.frame_count = 0;
        }
        let input = crate::game::globals::get_input();
        if input.get_state(&KeyNames::Clock).just_pressed {
            self.timer
                .timer_interrupt(&mut self.countdown_321_track, &mut self.countdown_go_track);
        }
        match self.timer.pop_event() {
            TimerEvent::Paused => {}
            TimerEvent::PomoFinished => {
                self.start_pomo_sound();
                let nvm = crate::game::globals::get_nvm();
                let cycles = nvm
                    .settings
                    .get_setting(SettingType::PomodoroCycles)
                    .get_value();
                let inventory = &mut nvm.inventory;
                inventory.inc_tomatoes();
                if self.timer.cycles_elapsed + 1 == cycles {
                    inventory.inc_raspberries();
                    inventory.inc_juice();
                } else if self.timer.cycles_elapsed > 0 {
                    inventory.inc_juice();
                }
                inventory.write();
            }
            TimerEvent::ShortBreakFinished | TimerEvent::LongBreakFinished => {
                self.start_break_sound();
            }
            TimerEvent::None => {}
        }
    }

    fn sound(&mut self) {
        let hardware = crate::game::globals::get_hardware();
        self.pomo_finished_track.tick();
        self.break_finished_track.tick();
        self.countdown_321_track.tick();
        self.countdown_go_track.tick();
        if self.is_playing_alert() {
            hardware.start_vibrating();
        } else {
            hardware.stop_vibrating();
        }
    }

    fn draw(&mut self) {
        if self.is_playing_alert() {
            text_writer::full_dialog_box("", "");
            let x = LCD_WIDTH as i32 / 2;
            let y = LCD_HEIGHT as i32 / 2 - 7;
            let style = FontStyle::BigBold;
            let color = Rgb332::RED;
            let text = "TIME'S UP!!!";
            text_writer::draw_text_centered(x, y, style, color, text);
            return;
        }

        self.lofi_sprite
            .draw((self.frame_count / ANIM_ON_RATE) % 180);

        let back_sprite_frame = self.timer.get_back_sprite_frame();
        self.menu_sprite.x = 0;
        self.menu_sprite.y = (LCD_HEIGHT - self.menu_sprite.h) as i32;
        self.menu_sprite.draw(back_sprite_frame as usize);

        let confirm_sprite_frame = self.timer.get_confirm_sprite_frame();
        self.menu_sprite.x = (LCD_WIDTH - self.menu_sprite.w) as i32;
        self.menu_sprite.y = (LCD_HEIGHT - self.menu_sprite.h) as i32;
        self.menu_sprite.draw(confirm_sprite_frame as usize);

        let message = self.timer.get_message();
        text_writer::bottom_dialog_box(&message);

        let (timer_text, color) = self.timer.get_timer_text_and_color();
        text_writer::bottom_big_dialog_box_custom_color(&timer_text, color);
    }

    fn next_scene(&self) -> &Option<SceneType> {
        &self.next_scene
    }
}
