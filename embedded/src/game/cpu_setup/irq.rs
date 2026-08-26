use rp2040_hal::pac::interrupt;

/// Enable hardware interrupt to wake the CPU from wfi() calls
#[interrupt]
fn IO_IRQ_BANK0() {
    unsafe {
        let p = rp2040_hal::pac::Peripherals::steal();

        // Clear all GPIO interrupt sources
        p.IO_BANK0.intr[0].write(|w| w.bits(0xffffffff));
        p.IO_BANK0.intr[1].write(|w| w.bits(0xffffffff));
    }
}
