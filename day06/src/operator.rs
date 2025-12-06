#[derive(Debug, Copy, Clone)]
pub enum Operator {
    Add,
    Multiply,
}

impl Operator {
    #[must_use]
    pub fn parse_str(value: &str) -> Option<Operator> {
        match value {
            "+" => Some(Operator::Add),
            "*" => Some(Operator::Multiply),
            _ => None,
        }
    }

    #[must_use]
    pub fn parse(value: char) -> Option<Operator> {
        match value {
            '+' => Some(Operator::Add),
            '*' => Some(Operator::Multiply),
            _ => None,
        }
    }
}
