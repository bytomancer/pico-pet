use crate::game::scenes::eat_scene::EatScene;
use crate::game::scenes::intro_scene::IntroScene;
use crate::game::scenes::main_scene::MainScene;
use crate::game::scenes::nyi_scene::NyiScene;
use crate::game::scenes::pomo_scene::PomoScene;
use crate::game::scenes::settings_scene::SettingsScene;
use crate::game::scenes::stat_scene::stat_scene::StatScene;
use crate::game::scenes::SceneBehavior;
use crate::game::scenes::SceneType;

/// The scene manager tracks the current state of the game as a "scene".
/// Common scenes are the "main" scene or the "settings" scene.
/// The manager keeps exactly 1 Some scene, tied to the active_scene.
/// All other possible scenes should be None.
/// Upon boot, the IntroScene is always the first scene.
/// It is up to the IntroScene to then determine if it should be skipped.
pub struct SceneManager<'a> {
    pub intro_scene: Option<IntroScene>,

    pub game_play_scene: Option<MainScene<'a>>,

    pub pomo_scene: Option<PomoScene<'a>>,
    pub eat_scene: Option<EatScene<'a>>,
    pub stat_scene: Option<StatScene>,
    pub cosmetic_scene: Option<NyiScene>,
    pub settings_scene: Option<SettingsScene>,

    pub active_scene: SceneType,
}

impl<'a> Default for SceneManager<'a> {
    fn default() -> Self {
        Self {
            intro_scene: Some(IntroScene::default()),
            game_play_scene: Default::default(),
            pomo_scene: Default::default(),
            eat_scene: Default::default(),
            stat_scene: Default::default(),
            cosmetic_scene: Default::default(),
            settings_scene: Default::default(),
            active_scene: SceneType::Intro,
        }
    }
}
impl SceneManager<'static> {
    /// Provides a dynamic pointer to the current scene for dynamic dispatch.
    /// All scenes must impl the SceneBehavior trait.
    /// When the scene manager acts on the active scene,
    /// this trait represents what it needs to perform on it.
    fn get_scene(&mut self) -> &mut dyn SceneBehavior {
        match self.active_scene {
            SceneType::Intro => self.intro_scene.as_mut().unwrap(),
            SceneType::Main => self.game_play_scene.as_mut().unwrap(),
            SceneType::Settings => self.settings_scene.as_mut().unwrap(),
            SceneType::Pomo => self.pomo_scene.as_mut().unwrap(),
            SceneType::Eat => self.eat_scene.as_mut().unwrap(),
            SceneType::Stat => self.stat_scene.as_mut().unwrap(),
            SceneType::Cosmetics => self.cosmetic_scene.as_mut().unwrap(),
        }
    }

    /// This function is the game's full life cycle of 1 frame:
    /// - Parse input
    /// - Tick, performing any logic
    /// - Produce sounds
    /// - Draw a frame to the display buffer
    pub fn update_and_draw(&mut self) {
        let curr_scene = self.get_scene();
        curr_scene.input();
        curr_scene.tick();
        curr_scene.sound();
        curr_scene.draw();
    }

    /// Tests the current scene for what it declares as the next scene.
    /// If the current scene returns None from `get_scene`, nothing happens.
    /// If a Some(SceneType) is returned, then the current scene ends,
    /// and a new scene of the SceneType is instantiated as the new current scene.
    pub fn advance_scene(&mut self) {
        let curr_scene = self.get_scene();
        match curr_scene.next_scene().clone() {
            Some(next_scene) => {
                {
                    let hardware = crate::game::globals::get_hardware();
                    hardware.start_tone(&crate::game::hardware::audio::AudioFrequency::None);
                }
                self.active_scene = next_scene.clone();
                self.game_play_scene = None;
                self.pomo_scene = None;
                self.eat_scene = None;
                self.stat_scene = None;
                self.cosmetic_scene = None;
                self.settings_scene = None;

                match next_scene {
                    SceneType::Intro => self.intro_scene = Some(IntroScene::default()),
                    SceneType::Main => self.game_play_scene = Some(MainScene::default()),
                    SceneType::Pomo => self.pomo_scene = Some(PomoScene::default()),
                    SceneType::Eat => self.eat_scene = Some(EatScene::default()),
                    SceneType::Stat => self.stat_scene = Some(StatScene::default()),
                    SceneType::Cosmetics => self.cosmetic_scene = Some(NyiScene::default()),
                    SceneType::Settings => self.settings_scene = Some(SettingsScene::default()),
                }
            }
            None => {}
        }
    }

    /// This provides a list of scenes where the "idle" state cannot appear.
    pub fn is_current_scene_unidleable(&self) -> bool {
        match self.active_scene {
            SceneType::Pomo | SceneType::Intro => true,
            _ => false,
        }
    }
}
