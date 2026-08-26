/// #Intro Scene
/// Intended to introduce the user to the vpet,
/// teach them about pomodoros,
/// and setup some basic settings.
///
/// Importantly, this scene is our boot scene.
/// If the nvm is in a "fresh" state,
/// we stay on this scene,
/// otherwise we skip straight to main.
///
/// Completing this scene writes a new save file to the NVM.
use core::usize;

use crate::game::color::Rgb332;
use crate::game::display::render;
use crate::game::display::text_writer;
use crate::game::hardware::hardware::LCD_HEIGHT;
use crate::game::hardware::hardware::LCD_WIDTH;
use crate::game::hardware::input::KeyNames;
use crate::game::scenes::SceneBehavior;
use crate::game::scenes::SceneType;

pub struct IntroScene {
    frame: usize,
    next_scene: Option<SceneType>,
}
impl Default for IntroScene {
    fn default() -> Self {
        let nvm = crate::game::globals::get_nvm();
        let next_scene = if nvm.fresh {
            None
        } else {
            Some(SceneType::Main)
        };
        Self {
            next_scene,
            frame: 0,
        }
    }
}
impl SceneBehavior for IntroScene {
    fn input(&mut self) {
        let input = crate::game::globals::get_input();
        if input.get_state(&KeyNames::Confirm).just_released {
            let nvm = crate::game::globals::get_nvm();
            nvm.fresh = false;
            nvm.write_all();
            self.next_scene = Some(SceneType::Main);
        }
    }

    fn tick(&mut self) {
        self.frame += 1;
    }

    fn sound(&mut self) {
        ()
    }

    fn draw(&mut self) {
        let frame_decrease = 3;
        let anim_frame = self.frame / frame_decrease;
        let anim_end = (frame_decrease * 17) + 1;
        if self.frame > anim_end {
            text_writer::full_dialog_box("Setup", "DEBUGGING\npress ok to skip");
        } else {
            render::fill_rect(
                0,
                0,
                LCD_WIDTH,
                LCD_HEIGHT,
                Self::fade_function(anim_frame as u8),
            )
        }
    }

    fn next_scene(&self) -> &Option<SceneType> {
        &self.next_scene
    }
}
impl IntroScene {
    /// Probably a placeholder unless we keep it...
    /// Fades the screen through some rainbow colours,
    /// based on the current frame of the animation.
    fn fade_function(f: u8) -> Rgb332 {
        // frame 1: red = 1
        // frame 2: green = 1
        // frame 3: red = 2
        // frame 4: green = 2
        // frame 5: blue = 1

        // frame 6: red = 3
        // frame 7: green = 3
        // frame 8: red = 4
        // frame 9: green = 4
        // frame 10: blue = 2

        // frame 11: red = 5
        // frame 12: green = 5
        // frame 13: red = 6
        // frame 14: green = 6
        // frame 15: blue = 3

        // frame 16: red = 7
        // frame 17: green = 7

        // on frame 17, color is 7, 7, 3, which is 0b111_111_11, aka white

        let cycles = f / 5;
        let sub_cycle = f % 5;
        let mut r = cycles * 2;
        if sub_cycle >= 3 {
            r += 2;
        } else if sub_cycle >= 1 {
            r += 1;
        }
        let mut g = cycles * 2;
        if sub_cycle >= 4 {
            g += 2;
        } else if sub_cycle >= 2 {
            g += 1;
        }
        let b = cycles;

        Rgb332::from_components(r, g, b)
    }
}
