/// Our NVM is broken into pages,
/// each page containing only 8 bytes of data.
/// The PageCanon assigns meaning to each page we utilize.
#[repr(u16)]
pub enum PageCanon {
    Header1 = 0x000,

    Settings1 = 0x001,
    Settings2 = 0x002,

    Inventory1 = 0x003,

    Pet1 = 0x004,
    Pet2 = 0x005,

    /// Evaluates to the count of enum varaints, useful for array sizing
    Count,
}
impl Into<u16> for PageCanon {
    fn into(self) -> u16 {
        self as u16
    }
}
