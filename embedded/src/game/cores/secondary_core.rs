use rp2040_hal::multicore::Stack;

#[allow(unused)]
pub static mut CORE1_STACK: Stack<4096> = Stack::new();

/// Gameloop running on the secondary core.
/// Currently unused, but present in case we do start doing multicore.
#[allow(unused)]
pub fn secondary_main_loop(sys_freq: u32) -> ! {
    // TODO (Release): Remove function if unused

    //     let core = unsafe { pac::CorePeripherals::steal() };
    //     let mut delay = cortex_m::delay::Delay::new(core.SYST, sys_freq);
    //     loop {
    //         cortex_m::asm::wfi();
    //         delay.delay_ms(1000);
    //     }
    loop {}
}
