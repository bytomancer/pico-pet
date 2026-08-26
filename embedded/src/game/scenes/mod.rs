pub mod eat_scene;
pub mod intro_scene;
pub mod main_scene;
pub mod nyi_scene;
pub mod pomo_scene;
pub mod settings_scene;
pub mod stat_scene;

/// Enumeration for each type of scene.
/// Used by the [`SceneManager`] to identify the active scene.
#[derive(Clone)]
pub enum SceneType {
    Intro,

    Main,

    Pomo,
    Eat,
    Stat,
    Cosmetics,
    Settings,
}
impl Default for SceneType {
    fn default() -> Self {
        Self::Intro
    }
}

/// Every scene must implement SceneBehavior.
/// On every frame:
/// - `input()` is called.
/// - `tick()` is called.
/// - `sound()` is called.
/// - `draw()` is called.
/// - The MCU primary core blocks while the display buffer pushes to the LCD.
/// - `next_scene()` is called, and if it returns Some, the scene changes.
pub trait SceneBehavior {
    fn input(&mut self);
    fn tick(&mut self);
    fn sound(&mut self);
    fn draw(&mut self);

    fn next_scene(&self) -> &Option<SceneType>;
}
