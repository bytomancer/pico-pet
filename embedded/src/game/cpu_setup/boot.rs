/// Sets up boot2 to use the W25Q080 style bootloader.
/// This bootloader is optimized for the flash chip on the RP2040-Plus.
#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;
