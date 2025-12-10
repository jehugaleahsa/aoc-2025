use crate::tile::Tile;

#[derive(Debug, Copy, Clone)]
pub struct Square {
    pub first: Tile,
    pub second: Tile,
}

impl Square {
    #[inline]
    #[must_use]
    pub fn new(first: Tile, second: Tile) -> Self {
        Self { first, second }
    }

    #[inline]
    #[must_use]
    pub fn area(&self) -> u64 {
        let width = (self.first.x as i64 - self.second.x as i64).unsigned_abs() + 1;
        let height = (self.first.y as i64 - self.second.y as i64).unsigned_abs() + 1;
        let area = width * height;
        area
    }
}

#[cfg(test)]
mod tests {
    use crate::square::Square;
    use crate::tile::Tile;

    #[test]
    fn test_area_simple() {
        let first = Tile::from_x_y(1, 1);
        let second = Tile::from_x_y(10, 10);
        let square = Square::new(first, second);
        let area = square.area();
        assert_eq!(100, area);
    }

    #[test]
    fn test_area_simple2() {
        let first = Tile::from_x_y(1, 10);
        let second = Tile::from_x_y(1, 10);
        let square = Square::new(first, second);
        let area = square.area();
        assert_eq!(1, area);
    }

    #[test]
    fn test_area_example1() {
        let first = Tile::from_x_y(2, 5);
        let second = Tile::from_x_y(9, 7);
        let square = Square::new(first, second);
        let area = square.area();
        assert_eq!(24, area);
    }

    #[test]
    fn test_area_example2() {
        let first = Tile::from_x_y(2, 5);
        let second = Tile::from_x_y(11, 1);
        let square = Square::new(first, second);
        let area = square.area();
        assert_eq!(50, area);
    }
}
