use crate::junction::Junction;

#[derive(Debug)]
pub struct Connection {
    pub first: Junction,
    pub second: Junction,
}

impl Connection {
    #[must_use]
    pub fn distance(&self) -> f64 {
        let x_diff = f64::from(self.second.x) - f64::from(self.first.x);
        let y_diff = f64::from(self.second.y) - f64::from(self.first.y);
        let z_diff = f64::from(self.second.z) - f64::from(self.first.z);
        let x_squared = x_diff * x_diff;
        let y_squared = y_diff * y_diff;
        let z_squared = z_diff * z_diff;
        let sum = x_squared + y_squared + z_squared;
        sum.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use crate::connection::Connection;
    use crate::junction::Junction;

    #[test]
    fn test_distance() {
        let first = Junction::from_x_y_z(162, 817, 812);
        let second = Junction::from_x_y_z(425, 690, 689);
        let connection = Connection { first, second };
        let distance = connection.distance();
        assert!(distance < 316.903);
        assert!(distance > 316.902);
    }
}
