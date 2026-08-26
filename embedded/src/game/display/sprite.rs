use super::render;

/// A simple sprite ready to display to the screen.
/// Must have the w, h, and data from its sprite sheet.
/// The x & y specify its position on the screen,
/// with 0,0 being at the top left.
/// Sprites should be created via the [`sprite_factory`].
pub struct Sprite<'a> {
    pub x: i32,
    pub y: i32,
    pub w: usize,
    pub h: usize,
    data: &'a [u8],
}

impl<'a> Sprite<'a> {
    pub fn new(x: i32, y: i32, w: usize, h: usize, data: &'a [u8]) -> Self {
        Sprite { x, y, w, h, data }
    }

    pub fn draw(&self, frame: usize) {
        let offset = frame * self.w * self.h;
        render::blit_from_offset(self.x, self.y, offset, self.w, self.h, self.data)
    }
}
