#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Junction {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Junction {
    #[inline]
    #[must_use]
    pub fn from_x_y_z(x: u32, y: u32, z: u32) -> Self {
        Junction { x, y, z }
    }
}
