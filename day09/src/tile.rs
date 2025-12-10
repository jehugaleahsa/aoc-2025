#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Tile {
    pub x: u32,
    pub y: u32,
}

impl Tile {
    #[inline]
    #[must_use]
    pub fn from_x_y(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}
