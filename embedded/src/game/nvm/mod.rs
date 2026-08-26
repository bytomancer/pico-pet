/// Everything related to our NVM utilization.
/// NVM, or Non-Volatile Memory, is just persistant storage.
/// You can think of it like a save file.
pub mod header;
pub mod inventory;
pub mod page_canon;
mod pet;
pub mod settings;

pub use self::header::NvmHeader;
use self::inventory::NvmInventory;
use self::page_canon::PageCanon;
pub use self::pet::NvmPet;
pub use self::settings::NvmSettings;

/// Used to represent an unused byte.
/// The use of 0xff matches the default state of the AT24C32,
/// which ships filled with 0xff from the factory.
pub const NVM_BLANK: u8 = 0xff;

/// Our global NVM struct.
/// This represents our entire "save file".
///
/// Contains one bit of metadata, "fresh".
/// Fresh is true if the NVM is assumed to not contain a save file yet.
///
/// In addition, it contains several meaningful NVM containers,
/// each representing potentially many pages of memory,
/// all relating to a similar concept for the save file.
///
/// For a in-depth breakdown of the NVM page usage,
/// view the project root's [`README.md`]
pub struct Nvm {
    pub fresh: bool,
    pub parity: NvmHeader,
    pub settings: NvmSettings,
    pub inventory: NvmInventory,
    pub pet: NvmPet,
}
impl Default for Nvm {
    fn default() -> Self {
        Self {
            fresh: true,
            parity: NvmHeader::default(),
            settings: NvmSettings::default(),
            inventory: NvmInventory::default(),
            pet: NvmPet::default(),
        }
    }
}
impl Nvm {
    /// Attempts to load the NVM's data as a valid save file.
    /// If the NVM_SENTINEL is not detected in the first byte of the first page,
    /// a blank save state is created.
    pub fn load_or_create_default() -> Self {
        match NvmHeader::try_load() {
            Some(parity) => {
                let new_nvm = Self {
                    fresh: false,
                    parity,
                    settings: NvmSettings::load(),
                    inventory: NvmInventory::load(),
                    pet: NvmPet::load(),
                };

                new_nvm
            }
            None => {
                let new_nvm = Self::default();

                // Used to be here, but now part of the IntroScene.
                // new_nvm.write_all();

                new_nvm
            }
        }
    }

    /// Backs up the current save state data into NVM.
    ///
    /// Without calling this, the save state is lost when power is lost.
    /// However, try to limit the usage of this function.
    /// Ideally we only need to write after performing an operation,
    /// even better if we have to leave the scene to confirm it.
    /// We should inform the user that exiting is "confirming" and "saving",
    /// otherwise they could assume that any change is automatically saved,
    /// especially because we can update the global state without writing,
    /// making it appear the changes are reflected.
    ///
    /// For example, if you change the volume,
    /// and update the volume stored in the "Nvm" global,
    /// the volume will be updated and reflect across all audio playbacks.
    /// However, if `write_all` is not called,
    /// when the user reboots the volume will re-load from the NVM.
    ///
    /// The AT24C32 has a write tolerance of about 1 million writes per cell.
    /// I believe each cell is 4096 bytes, or 512 pages,
    /// but I haven't looked into it to confirm.
    ///
    /// TODO: As I document this,
    /// I realized `Nvm` is not a good name for this struct.
    /// It's really closer to an ActiveRecord.
    /// All the values live in RAM.
    /// It is only by calling `write_all` or `load` that the NVM is used.
    /// It's important to view this as the active but ephemeral storage of data,
    /// and respect that the memory is fleeting until "backed up"
    /// by calling `write_all`.
    ///
    /// TODO:
    /// We can also improve write tolerance by tagging pages as "dirty".
    /// Then, we only rewrite any page that is dirty.
    /// Dirty is set whenever a value on the page is updated,
    /// and that update differs from its previous value.
    pub fn write_all(&mut self) {
        self.parity.write();
        self.settings.write();
        self.inventory.write();
        self.pet.write();
    }

    /// A factory reset option.
    /// All data in the save file will be lost when this is called,
    /// then the device will reboot.
    pub fn erase_all_then_reboot(&mut self) {
        let hardware = crate::game::globals::get_hardware();

        let blank_data_buffer = [NVM_BLANK; 8];
        for page in 0..PageCanon::Count.into() {
            hardware.write_nvm_page(page, &blank_data_buffer);
        }

        crate::game::exit::reboot();
    }
}
