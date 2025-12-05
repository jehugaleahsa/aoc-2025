use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct AdventError {
    message: String,
}

impl AdventError {
    #[inline]
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for AdventError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AdventError {}
