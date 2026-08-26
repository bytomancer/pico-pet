use crate::game::display;
use crate::game::hardware::hardware::HardwareComponents;
use crate::game::hardware::input::InputHandler;
use crate::game::nvm::Nvm;

static mut HARDWARE: Option<HardwareComponents> = None;
static mut INPUT: Option<InputHandler> = None;
static mut NVM: Option<Nvm> = None;

fn init_hardware() {
    unsafe { self::HARDWARE = Some(HardwareComponents::new()) }
}

fn init_input() {
    unsafe { self::INPUT = Some(InputHandler::default()) }
}

fn init_nvm() {
    unsafe { self::NVM = Some(Nvm::load_or_create_default()) }
}

/// Initialize all the globals required by the game
/// Initialize the RP2040 hardware
/// Then either loads or initializes a fresh save file from NVM
/// Then sets up the input manager
/// Finally the fonts get loaded for text writing
pub fn init_globals() {
    self::init_hardware();
    self::init_nvm();
    self::init_input();

    display::text_writer::init_singleton_fonts();
}

/// Test if the hardware global is initialized.
#[allow(static_mut_refs)]
pub fn is_hardware_initialized() -> bool {
    unsafe { !self::HARDWARE.is_none() }
}
/// Mutably reference the hardware global.
#[allow(static_mut_refs)]
pub fn get_hardware() -> &'static mut HardwareComponents {
    unsafe { self::HARDWARE.as_mut().unwrap() }
}
/// Mutably reference the input global.
#[allow(static_mut_refs)]
pub fn get_input() -> &'static mut InputHandler {
    unsafe { self::INPUT.as_mut().unwrap() }
}
/// Mutably reference the nvm global.
#[allow(static_mut_refs)]
pub fn get_nvm() -> &'static mut Nvm {
    unsafe { self::NVM.as_mut().unwrap() }
}
