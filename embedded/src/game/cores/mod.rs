pub mod primary_core;
pub mod scene_manager;
pub mod secondary_core;

use self::primary_core::primary_main_loop;
use self::secondary_core::secondary_main_loop;

use rp2040_hal::multicore::Multicore;

/// Sets up and starts the secondary core.
/// The second core runs `secondary_main_loop` indefinitely.
/// There is a limited size stack for the secondary core.
/// See CORE1_STACK for more information.
#[allow(unused)]
pub fn spawn_secondary_core_worker() {
    // TODO (Release): Remove entire function if unused
    unsafe {
        let hardware = crate::game::globals::get_hardware();
        let mut mc = Multicore::new(
            &mut *hardware.psm_ptr,
            &mut *hardware.ppb_ptr,
            &mut *hardware.fifo_ptr,
        );
        let cores = &mut mc.cores();
        let core1 = &mut cores[1];
        let sys_freq = hardware.sys_freq;
        #[allow(static_mut_refs)]
        let stack_ref = &mut secondary_core::CORE1_STACK.mem;
        let _test = core1.spawn(stack_ref, move || secondary_main_loop(sys_freq));
    }
}

pub fn run_primary_main_loop() -> ! {
    primary_main_loop()
}
