pub mod audio;
pub mod boot;
pub mod color;
pub mod cores;
pub mod display;
pub mod display_helper;
pub mod exit;
pub mod globals;
pub mod hardware;
pub mod irq;
pub mod nvm;
pub mod scenes;
pub mod setting_value;

pub const FRAME_RATE: u32 = 30;
pub const APPROXIMATE_FRAME_RATE: u32 = 31;
