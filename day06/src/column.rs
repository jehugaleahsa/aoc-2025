use crate::operator::Operator;

#[derive(Debug)]
pub struct Column {
    pub values: Vec<i64>,
    pub operator: Operator,
}

impl Column {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            operator: Operator::Add,
        }
    }

    #[must_use]
    pub fn fold(&self) -> i64 {
        match self.operator {
            Operator::Add => self.values.iter().fold(0, |x, y| x + y),
            Operator::Multiply => self.values.iter().fold(1, |x, y| x * y),
        }
    }
}

impl Default for Column {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
