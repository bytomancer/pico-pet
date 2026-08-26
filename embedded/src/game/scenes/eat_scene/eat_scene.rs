/// # Feeding Scene
/// This scene is where the user feeds the vpet.
/// Currently a placeholder.
use crate::game::color::Rgb332;
use crate::game::display::sprite::Sprite;
use crate::game::display::sprite_factory;
use crate::game::display::text_writer;
use crate::game::hardware::input::KeyNames;
use crate::game::scenes::SceneBehavior;
use crate::game::scenes::SceneType;

/*

Thoughts:
Probably need subscenes...
1. You have already fed the pet and it cannot be fed again today.
2. You have insufficient tomatoes+juice, go do more pomodoros.
3. You have no tomatoes, would you like to use your reserved juice?
4. You have enough tomatoes, proceed?

*/

pub struct EatScene<'a> {
    next_scene: Option<SceneType>,
    inv_sprite: Sprite<'a>,
}
impl Default for EatScene<'_> {
    fn default() -> Self {
        Self {
            next_scene: None,
            inv_sprite: sprite_factory::new_inventory_sprite(0, 0),
        }
    }
}

impl SceneBehavior for EatScene<'_> {
    fn input(&mut self) {
        let input = crate::game::globals::get_input();
        if input.get_state(&KeyNames::Back).just_released {
            self.next_scene = Some(SceneType::Main);
        }
    }

    fn tick(&mut self) {
        ()
    }

    fn sound(&mut self) {
        let hardware = crate::game::globals::get_hardware();
        hardware.end_tone();
    }

    fn draw(&mut self) {
        text_writer::draw_text_left_aligned_nowrap(
            0,
            64,
            text_writer::FontStyle::Small,
            Rgb332::WHITE,
            "Feed?",
        );

        self.inv_sprite.x = 0;
        self.inv_sprite.y = 0;
        self.inv_sprite.draw(0);

        self.inv_sprite.y += self.inv_sprite.h as i32;
        self.inv_sprite.draw(1);

        self.inv_sprite.y += self.inv_sprite.h as i32;
        self.inv_sprite.draw(2);

        self.inv_sprite.y += self.inv_sprite.h as i32;
        self.inv_sprite.draw(3);

        self.inv_sprite.y += self.inv_sprite.h as i32;
        self.inv_sprite.draw(4);
    }

    fn next_scene(&self) -> &Option<SceneType> {
        &self.next_scene
    }
}
