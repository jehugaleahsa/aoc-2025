#[derive(Debug)]
pub struct MarkedTiles {
    min_x: usize,
    min_y: usize,
    x_range: usize,
    marked: Vec<bool>,
}

impl MarkedTiles {
    pub fn new(min_x: u32, max_x: u32, min_y: u32, max_y: u32) -> Self {
        let min_x = min_x as usize;
        let max_x = max_x as usize;
        let min_y = min_y as usize;
        let max_y = max_y as usize;
        let x_range = max_x - min_x + 1;
        let y_range = max_y - min_y + 1;
        let tile_count = x_range * y_range;
        let marked = vec![false; tile_count];
        Self {
            min_x,
            min_y,
            x_range,
            marked,
        }
    }

    pub fn is_set(&self, x: u32, y: u32) -> bool {
        let translated_x = x as usize - self.min_x;
        let translated_y = y as usize - self.min_y;
        let index = translated_y * self.x_range + translated_x;
        self.marked[index]
    }

    pub fn set(&mut self, x: u32, y: u32) {
        let translated_x = x as usize - self.min_x;
        let translated_y = y as usize - self.min_y;
        let index = translated_y * self.x_range + translated_x;
        self.marked[index] = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::marked_tiles::MarkedTiles;

    #[test]
    pub fn test_indexing() {
        let mut tiles = MarkedTiles::new(0, 2, 0, 2);
        tiles.set(0, 0);
        assert!(tiles.is_set(0, 0));
        tiles.set(0, 1);
        assert!(tiles.is_set(0, 1));
        tiles.set(0, 2);
        assert!(tiles.is_set(0, 2));
    }
}
