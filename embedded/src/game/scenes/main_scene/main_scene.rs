/// # Main Scene
/// When the user first wakes the vpet for the day,
/// this is where they will be.
///
/// Contains a menu to reach many of the other scenes.
///
/// We should have little animations on this screen,
/// where the pet expresses their personality.
use crate::game::audio::audio_library::AudioId;
use crate::game::audio::audio_player::AudioPlayer;
use crate::game::audio::audio_player::AutoPlayMode;
use crate::game::audio::audio_player::RepeatMode;
use crate::game::display::sprite::Sprite;
use crate::game::display::sprite_factory;
use crate::game::display::sprite_factory::MENU_DIMENSIONS;
use crate::game::hardware::hardware::LCD_HEIGHT;
use crate::game::hardware::hardware::LCD_WIDTH;
use crate::game::hardware::input::KeyNames;
use crate::game::scenes::SceneBehavior;
use crate::game::scenes::SceneType;
use crate::game::APPROXIMATE_FRAME_RATE;

use super::menu_selection::MenuSelection;

pub struct MainScene<'a> {
    ferris: Sprite<'a>,
    menu_sprite: Sprite<'a>,
    frame_count: u32,
    next_scene: Option<SceneType>,
    menu_item_selected: MenuSelection,
    ferris_cry: AudioPlayer,
    button_beep: AudioPlayer,
}
impl Default for MainScene<'static> {
    fn default() -> Self {
        let ferris = sprite_factory::new_ferris_sprite(
            (LCD_WIDTH as i32 - sprite_factory::FERRIS_DIMENSIONS.w as i32) / 2,
            LCD_HEIGHT as i32 - 64,
        );

        let menu_sprite = sprite_factory::new_menu_sprite(0, 0);

        Self {
            ferris,
            menu_sprite,
            frame_count: 0,
            next_scene: None,
            menu_item_selected: MenuSelection::Pomo,
            ferris_cry: AudioPlayer::new(AudioId::FerrisCry, RepeatMode::Off, AutoPlayMode::Off),
            button_beep: AudioPlayer::new(AudioId::ButtonBeep, RepeatMode::Off, AutoPlayMode::Off),
        }
    }
}
impl SceneBehavior for MainScene<'static> {
    fn tick(&mut self) {
        self.frame_count += 1;
        if self.frame_count % (4 * APPROXIMATE_FRAME_RATE) == (1 * APPROXIMATE_FRAME_RATE)
            || self.frame_count % (4 * APPROXIMATE_FRAME_RATE) == 0
        {
            self.ferris.x -= 8;
        } else if self.frame_count % (4 * APPROXIMATE_FRAME_RATE) == (2 * APPROXIMATE_FRAME_RATE)
            || self.frame_count % (4 * APPROXIMATE_FRAME_RATE) == (3 * APPROXIMATE_FRAME_RATE)
        {
            self.ferris.x += 8;
        }
    }

    fn sound(&mut self) {
        self.ferris_cry.tick();
        self.button_beep.tick();

        if self.frame_count % (2 * APPROXIMATE_FRAME_RATE) == APPROXIMATE_FRAME_RATE {
            self.ferris_cry.play();
        }
    }

    fn draw(&mut self) {
        self.ferris
            .draw(((self.frame_count / APPROXIMATE_FRAME_RATE) % 2) as usize);

        let sel_item = self.menu_item_selected as usize;
        let prev_item = self.menu_item_selected.prev() as usize;
        let next_item = self.menu_item_selected.next() as usize;

        self.menu_sprite.x = MENU_DIMENSIONS.w as i32 + 4;
        self.menu_sprite.y = (LCD_HEIGHT - MENU_DIMENSIONS.h) as i32 + 4;
        self.menu_sprite.draw(5);
        self.menu_sprite.draw(prev_item);

        self.menu_sprite.x = (MENU_DIMENSIONS.w * 3) as i32 + 4;
        self.menu_sprite.y = (LCD_HEIGHT - MENU_DIMENSIONS.h) as i32 + 4;
        self.menu_sprite.draw(5);
        self.menu_sprite.draw(next_item);

        self.menu_sprite.x = (MENU_DIMENSIONS.w * 2) as i32 + 4;
        self.menu_sprite.y = (LCD_HEIGHT - MENU_DIMENSIONS.h) as i32;
        self.menu_sprite.draw(6);
        self.menu_sprite.draw(sel_item);
    }

    fn input(&mut self) {
        let input = crate::game::globals::get_input();

        if input.get_state(&KeyNames::Left).just_pressed {
            self.menu_item_selected = self.menu_item_selected.prev();
            self.button_beep.play();
        }
        if input.get_state(&KeyNames::Right).just_pressed {
            self.menu_item_selected = self.menu_item_selected.next();
            self.button_beep.play();
        }

        if input.get_state(&KeyNames::Confirm).just_released {
            self.menu_button_confirmed();
        }
    }

    fn next_scene(&self) -> &Option<SceneType> {
        &self.next_scene
    }
}

impl MainScene<'static> {
    fn menu_button_confirmed(&mut self) {
        match self.menu_item_selected {
            MenuSelection::Pomo => self.next_scene = Some(SceneType::Pomo),
            MenuSelection::Eat => self.next_scene = Some(SceneType::Eat),
            MenuSelection::Stat => self.next_scene = Some(SceneType::Stat),
            MenuSelection::Cosmetic => self.next_scene = Some(SceneType::Cosmetics),
            MenuSelection::Settings => self.next_scene = Some(SceneType::Settings),
            MenuSelection::None => {}
        }
    }
}
