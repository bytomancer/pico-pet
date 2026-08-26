/// This enum specifies the notes available in the BEAT audio format.
/// Notes use regular temperment 440hz,
/// with all notes off of C-major represented as "sharp".
/// Each of these tones are represented by a square wave,
/// powered entirely by the PWM module.
#[allow(unused)]
#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum AudioFrequency {
    C4 = 0x77,
    Cs4 = 0x78,
    D4 = 0x79,
    Ds4 = 0x7A,
    E4 = 0x7B,
    F4 = 0x7C,
    Fs4 = 0x7D,
    G4 = 0x7E,
    Gs4 = 0x7F,
    A4 = 0x80,
    As4 = 0x81,
    B4 = 0x82,
    C5 = 0x83,
    Cs5 = 0x84,
    D5 = 0x85,
    Ds5 = 0x86,
    E5 = 0x87,
    F5 = 0x88,
    Fs5 = 0x89,
    G5 = 0x8A,
    Gs5 = 0x8B,
    A5 = 0x8C,
    As5 = 0x8D,
    B5 = 0x8E,
    C6 = 0x8F,
    Cs6 = 0x90,
    D6 = 0x91,
    Ds6 = 0x92,
    E6 = 0x93,
    F6 = 0x94,
    Fs6 = 0x95,
    G6 = 0x96,
    Gs6 = 0x97,
    A6 = 0x98,
    As6 = 0x99,
    B6 = 0x9A,
    C7 = 0x9B,

    None = 0x00,
}
impl AudioFrequency {
    /// Casts a raw u8 to its equivalent BEAT
    /// **UB: unrecognized byte in BEAT file.**
    pub fn from_byte(byte: u8) -> Self {
        unsafe { core::mem::transmute(byte) }
    }

    /// A programatically generated map of PWM register configs,
    /// which produces a tone approximate to the expected frequency of the note.
    /// For more info, see our associated helper crate:
    /// `helpers/frequency_finder`
    pub fn get_registers(&self) -> AudioRegisterConfig {
        match &self {
            Self::C4 => AudioRegisterConfig::new(65335, 7, 5),
            Self::Cs4 => AudioRegisterConfig::new(65003, 6, 15),
            Self::D4 => AudioRegisterConfig::new(65485, 6, 8),
            Self::Ds4 => AudioRegisterConfig::new(64930, 6, 3),
            Self::E4 => AudioRegisterConfig::new(65239, 5, 13),
            Self::F4 => AudioRegisterConfig::new(65077, 5, 8),
            Self::Fs4 => AudioRegisterConfig::new(65126, 5, 3),
            Self::G4 => AudioRegisterConfig::new(65409, 4, 14),
            Self::Gs4 => AudioRegisterConfig::new(65077, 4, 10),
            Self::A4 => AudioRegisterConfig::new(64934, 4, 6),
            Self::As4 => AudioRegisterConfig::new(65004, 4, 2),
            Self::B4 => AudioRegisterConfig::new(65314, 3, 14),
            Self::C5 => AudioRegisterConfig::new(64783, 3, 11),
            Self::Cs5 => AudioRegisterConfig::new(64422, 3, 8),
            Self::D5 => AudioRegisterConfig::new(65484, 3, 4),
            Self::Ds5 => AudioRegisterConfig::new(64281, 3, 2),
            Self::E5 => AudioRegisterConfig::new(64546, 2, 15),
            Self::F5 => AudioRegisterConfig::new(65077, 2, 12),
            Self::Fs5 => AudioRegisterConfig::new(64349, 2, 10),
            Self::G5 => AudioRegisterConfig::new(65410, 2, 7),
            Self::Gs5 => AudioRegisterConfig::new(65076, 2, 5),
            Self::A5 => AudioRegisterConfig::new(64934, 2, 3),
            Self::As5 => AudioRegisterConfig::new(65003, 2, 1),
            Self::B5 => AudioRegisterConfig::new(65313, 1, 15),
            Self::C6 => AudioRegisterConfig::new(63703, 1, 14),
            Self::Cs6 => AudioRegisterConfig::new(64422, 1, 12),
            Self::D6 => AudioRegisterConfig::new(65484, 1, 10),
            Self::Ds6 => AudioRegisterConfig::new(64281, 1, 9),
            Self::E6 => AudioRegisterConfig::new(63201, 1, 8),
            Self::F6 => AudioRegisterConfig::new(65077, 1, 6),
            Self::Fs6 => AudioRegisterConfig::new(64349, 1, 5),
            Self::G6 => AudioRegisterConfig::new(63775, 1, 4),
            Self::Gs6 => AudioRegisterConfig::new(63363, 1, 3),
            Self::A6 => AudioRegisterConfig::new(63130, 1, 2),
            Self::As6 => AudioRegisterConfig::new(63092, 1, 1),
            Self::B6 => AudioRegisterConfig::new(63273, 1, 0),
            Self::C7 => AudioRegisterConfig::new(59721, 1, 0),

            Self::None => AudioRegisterConfig::new(0, 0, 0),
        }
    }
}

/// PWM register configuration settings.
/// Producing an exact frequency isn't as simple as putting in a goal in hz.
/// Instead, you must follow the formula:
/// PWM(hz) = SYSTEM_FREQUENCY(hz) / ( (TOP+1) * (CSR_PH_CORRECT+1) * (DIV_INT + (DIV_FRAC/16)) )
/// SYSTEM_FREQUENCY = 125_000_000 HZ
/// CSR_PH_CORRECT = 1
/// The TOP, DIV_INT, and DIV_FRAC represent the 3 configurable registers.
/// See page 528 of the RP2040 PDF for more information.
/// [https://pip-assets.raspberrypi.com/categories/814-rp2040/documents/RP-008371-DS-1-rp2040-datasheet.pdf]
pub struct AudioRegisterConfig {
    top: u16,
    div_int: u8,
    div_frac: u8,
}
impl AudioRegisterConfig {
    fn new(top: u16, div_int: u8, div_frac: u8) -> Self {
        Self {
            top,
            div_int,
            div_frac,
        }
    }
    pub fn get_top(&self) -> u16 {
        self.top
    }
    pub fn get_div_int(&self) -> u8 {
        self.div_int
    }
    pub fn get_div_frac(&self) -> u8 {
        self.div_frac
    }
}
