use crate::game::color::Rgb332;
use crate::game::display::render;
use crate::game::display::text_writer;
use crate::game::display::text_writer::FontStyle;
use crate::game::display_helper::top_bar::draw_top_bar;
use crate::game::globals;
use crate::game::hardware::hardware::BRIGHTNESS_LUT;
use crate::game::hardware::hardware::LCD_HEIGHT;
use crate::game::hardware::hardware::LCD_WIDTH;
use crate::game::hardware::input::KeyNames;
use crate::game::scenes::main_scene::MainScene;
use crate::game::APPROXIMATE_FRAME_RATE;

use super::scene_manager::SceneManager;

const SECONDS_UNTIL_IDLE: usize = 60;
const FRAMES_UNTIL_IDLE: usize = SECONDS_UNTIL_IDLE * APPROXIMATE_FRAME_RATE as usize;

/// The core game loop
/// Does some game setup first
/// Draws a blue frame to the screen
///
/// On every frame,
/// idle checking is done,
/// then the screen floods w/ black,
/// the scene manager performs the full tick on the active scene,
/// the screen is swapped,
/// and lastly the scene manager checks for and performs any state transitions.
pub fn primary_main_loop() -> ! {
    let mut scene_manager = SceneManager::default();
    scene_manager.game_play_scene = Some(MainScene::default());

    let mut idle_frame_counter: usize = 0;

    // Draw the first frame to initialize the screen
    // Black fails to initialize the display
    render::flood(Rgb332::DARKEST_BLUE);
    swap();

    loop {
        check_feeding_deadline_is_passed();

        // TODO: move idle to its own state on top of the current state
        {
            let input = crate::game::globals::get_input();
            let hardware = crate::game::globals::get_hardware();

            if scene_manager.is_current_scene_unidleable() {
                idle_frame_counter = 0
            }

            let any_key_pressed = hardware.key0_pressed()
                || hardware.key1_pressed()
                || hardware.key2_pressed()
                || hardware.key3_pressed();
            if idle_frame_counter >= FRAMES_UNTIL_IDLE {
                input.force_reset();
                if any_key_pressed {
                    // next frame will return from idle state
                    idle_frame_counter = 0;
                    // wait for the user to release all keys
                    while hardware.key0_pressed()
                        || hardware.key1_pressed()
                        || hardware.key2_pressed()
                        || hardware.key3_pressed()
                    {}
                }
                let lowest_brightness = BRIGHTNESS_LUT[0];
                // Can't draw a fully black frame
                render::flood(Rgb332::DARKEST_BLUE);

                let x = LCD_WIDTH as i32 / 2;
                let y = LCD_HEIGHT as i32 / 2 - FontStyle::Big.get_glyph_dimensions().1 as i32 / 2;
                let time = hardware.get_time();

                let time_str = time.hh_mm_str();

                text_writer::draw_text_centered(
                    x,
                    y,
                    text_writer::FontStyle::Big,
                    Rgb332::WHITE,
                    time_str.as_str(),
                );
                hardware.set_backlight_raw(lowest_brightness);
                hardware.end_tone();
                crate::game::display::render::draw_buffer_to_screen(&mut hardware.display);

                // hardware.wfi();

                continue;
            } else if any_key_pressed {
                idle_frame_counter = 0;
            } else {
                idle_frame_counter += 1;
            }
        }

        globals::get_input().update();

        render::flood(Rgb332::BLACK);

        // TODO: some scenes don't have this, don't bother drawing it
        draw_top_bar();
        scene_manager.update_and_draw();

        swap();

        scene_manager.advance_scene();
    }
}

fn check_feeding_deadline_is_passed() -> () {
    let input = crate::game::globals::get_input();
    if !input.get_state(&KeyNames::Clock).just_released {
        // limit checks to 1hz to save CPU cycles
        return;
    }
    let nvm = crate::game::globals::get_nvm();
    // DEBUG: force hungry
    // nvm.pet.is_hungry = true;
    let now = crate::game::globals::get_hardware().get_date_time();
    let feeding_deadline = nvm.pet.get_feeding_deadline();
    let mut feeding_warning = feeding_deadline.clone();
    feeding_warning.dec_by_1_hour();

    if now > feeding_deadline {
        perform_starve();
        // TODO: reconsider this?
        // nvm.pet.is_hungry = true;
        // nvm.pet.is_starved = true;
        // } else if now > feeding_warning {
        //     nvm.pet.is_hungry = true;
    }
}

fn perform_starve() {
    let nvm = crate::game::globals::get_nvm();
    nvm.pet.is_hungry = false;
    nvm.pet.is_starved = false;
    let current_feeding_deadline = nvm.pet.get_feeding_deadline();
    nvm.pet.set_last_fed_date(current_feeding_deadline.date);
    let hp = nvm.pet.get_health();
    if hp > 0 {
        nvm.pet.set_health(hp - 1);
    }
    // current_feeding_deadline.inc_by_1_day();
    // nvm.pet.set
}

/// Performs a double-buffered frame swap
fn swap() {
    let hardware = crate::game::globals::get_hardware();
    hardware.set_backlight_from_lut();
    crate::game::display::render::draw_buffer_to_screen(&mut hardware.display);
}
