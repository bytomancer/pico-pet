use rp2040_hal::rom_data;

use crate::game::color::Rgb332;
use crate::game::display::render;
use crate::game::display::text_writer;
use crate::game::hardware::hardware::LCD_HEIGHT;

/// Custom panic handler
/// When we have access to an initialized global hardware struct,
/// The error is displayed to the screen with the line number of the error.
/// Any active audio is cancelled.
/// The display brightness is temporarily maximized.
/// Then you can press a button to reboot.
///
/// If the hardware isn't yet initialized,
/// we just reboot.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if !crate::game::globals::is_hardware_initialized() {
        loop {
            // TODO: change this to regular reboot for release
            // cortex_m::peripheral::SCB::sys_reset();
            rom_data::reset_to_usb_boot(0, 0);
            // if reset fails, just sleep
            cortex_m::asm::wfi();
        }
    }
    let hardware = crate::game::globals::get_hardware();
    render::flood(Rgb332::RED);
    unsafe {
        embedded_hal::PwmPin::set_duty(&mut (*hardware.backlight_channel_ptr), 32767);
        hardware.end_tone();
    }

    let err_str = fixedstr::str_format!(fixedstr::str256, "error:\n\\b000{:?}", info);

    text_writer::draw_text_centered(
        64,
        4,
        text_writer::FontStyle::BigBold,
        Rgb332::WHITE,
        "PANIC!",
    );

    text_writer::draw_text_left_aligned_wrapped(
        0,
        16,
        text_writer::FontStyle::Small,
        Rgb332::WHITE,
        &err_str,
    );

    text_writer::draw_text_centered(
        64,
        LCD_HEIGHT as i32 - 15,
        text_writer::FontStyle::Small,
        Rgb332::WHITE,
        "press any key to reboot",
    );

    render::draw_buffer_to_screen(&mut hardware.display);

    while !hardware.key0_pressed()
        && !hardware.key1_pressed()
        && !hardware.key2_pressed()
        && !hardware.key3_pressed()
    {}
    // TODO (Release): don't reset to USB
    rom_data::reset_to_usb_boot(0, 0);
    // if reset fails, just reboot
    cortex_m::peripheral::SCB::sys_reset()
}

/// A small scene for the reboot operation.
/// The screen informs the user a reboot is in progress.
/// Any active audio is cancelled.
/// The display brightness is temporarily maximized.
pub fn reboot() -> ! {
    if !crate::game::globals::is_hardware_initialized() {
        loop {
            // TODO: change this to regular reboot for release
            // cortex_m::peripheral::SCB::sys_reset();
            rom_data::reset_to_usb_boot(0, 0);
            // if reset fails, just sleep
            cortex_m::asm::wfi();
        }
    }
    let hardware = crate::game::globals::get_hardware();
    render::flood(Rgb332::from_u8(0b010_010_11));
    unsafe {
        embedded_hal::PwmPin::set_duty(&mut (*hardware.backlight_channel_ptr), 32767);
        hardware.end_tone();
    }
    text_writer::draw_text_centered(
        64,
        LCD_HEIGHT as i32 - 15,
        text_writer::FontStyle::Small,
        Rgb332::WHITE,
        "Rebooting...",
    );

    render::draw_buffer_to_screen(&mut hardware.display);

    hardware.delay.delay_ms(1_000);
    cortex_m::peripheral::SCB::sys_reset()
}
