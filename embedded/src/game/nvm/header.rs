/// At boot, the system reads the first page as the "header".
/// If the first byte of the header matches NVM_SENTINEL,
/// we assume the save file is legit and load it.
/// If that byte does not match NVM_SENTINEL,
/// we assume the chip is in its default state (all 0xFF),
/// the save file has been corrupted,
/// or a developer needs to force their NVM to be overwritten.
use super::page_canon::PageCanon;
use super::NVM_BLANK;

const NVM_SENTINEL: u8 = 0x68;

pub struct NvmHeader {
    pub data: [u8; 8],
}
impl Default for NvmHeader {
    fn default() -> Self {
        Self {
            data: [
                NVM_SENTINEL,
                NVM_BLANK,
                NVM_BLANK,
                NVM_BLANK,
                NVM_BLANK,
                NVM_BLANK,
                NVM_BLANK,
                NVM_BLANK,
            ],
        }
    }
}
impl NvmHeader {
    pub fn try_load() -> Option<Self> {
        let hardware = crate::game::globals::get_hardware();
        let data = hardware.get_nvm_page(PageCanon::Header1.into());
        if data[0] == NVM_SENTINEL {
            Some(Self { data })
        } else {
            None
        }
    }

    pub fn write(&self) {
        let hardware = crate::game::globals::get_hardware();
        hardware.write_nvm_page(PageCanon::Header1.into(), &self.data);
    }
}
