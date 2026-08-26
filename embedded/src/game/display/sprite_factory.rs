/// To add a sprite to the game,
/// append its data into these structures here.
/// You must specify the dimensions of each sprite sheet.
/// **Sprite sheets MUST be exported as "Vertical Strip"!**
use super::sprite::Sprite;

pub struct Dimensions {
    pub w: usize,
    pub h: usize,
    pub frames: usize,
}

pub const FERRIS_DIMENSIONS: Dimensions = Dimensions {
    w: 40,
    h: 24,
    frames: 2,
};
pub const MENU_DIMENSIONS: Dimensions = Dimensions {
    w: 24,
    h: 24,
    frames: 5,
};
pub const POMO_MENU_DIMENSIONS: Dimensions = Dimensions {
    w: 24,
    h: 24,
    frames: 4,
};
pub const INVENTORY_DIMENSIONS: Dimensions = Dimensions {
    w: 24,
    h: 24,
    frames: 5,
};
pub const LOFI_DIMENSIONS: Dimensions = Dimensions {
    w: 128,
    h: 80,
    frames: 58,
};

pub fn new_ferris_sprite(x: i32, y: i32) -> Sprite<'static> {
    Sprite::new(
        x,
        y,
        FERRIS_DIMENSIONS.w,
        FERRIS_DIMENSIONS.h,
        include_bytes!("../../../assets/sprite_raw/ferris.data"),
    )
}

pub fn new_menu_sprite(x: i32, y: i32) -> Sprite<'static> {
    Sprite::new(
        x,
        y,
        MENU_DIMENSIONS.w,
        MENU_DIMENSIONS.h,
        include_bytes!("../../../assets/sprite_raw/menu.data"),
    )
}

pub fn new_pomo_menu_sprite(x: i32, y: i32) -> Sprite<'static> {
    Sprite::new(
        x,
        y,
        POMO_MENU_DIMENSIONS.w,
        POMO_MENU_DIMENSIONS.h,
        include_bytes!("../../../assets/sprite_raw/pomo_menu.data"),
    )
}

pub fn new_inventory_sprite(x: i32, y: i32) -> Sprite<'static> {
    Sprite::new(
        x,
        y,
        INVENTORY_DIMENSIONS.w,
        INVENTORY_DIMENSIONS.h,
        include_bytes!("../../../assets/sprite_raw/inventory.data"),
    )
}

pub fn new_lofi_sprite(x: i32, y: i32) -> Sprite<'static> {
    Sprite::new(
        x,
        y,
        LOFI_DIMENSIONS.w,
        LOFI_DIMENSIONS.h,
        include_bytes!("../../../assets/sprite_raw/lofi.data"),
    )
}
