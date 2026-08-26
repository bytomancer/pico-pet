/// We use a W25Q32JVSSIQ in the RP2040-Plus
/// This sets up our boot loader via macros
#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;
