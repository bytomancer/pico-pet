// We will not use the standard library
// Many of its features are available in core::*
//
// In place of a standard main(),
// we use a special ROM bootstrap that invokes our #[entry]
#![no_std]
#![no_main]

mod game;

/// The second core is currently entirely disabled.
/// This keeps it in an extremely deep sleep.
/// My amperage testing shows that enabling it,
/// then causing it to wfi() costs around 5mA (very rough estimate).
/// Should we need to enable it, uncomment this function call in main.
#[allow(unused_imports)]
use game::cores::spawn_secondary_core_worker;

use game::cores::run_primary_main_loop;
use game::globals::init_globals;

use rp2040_hal::entry;

/// Entry point
/// Invoked by a bootstrap setup in game::cpu_setup::boot
#[entry]
fn main() -> ! {
    init_globals();
    // Second core is disabled for power savings
    // spawn_secondary_core_worker();
    run_primary_main_loop()
}
