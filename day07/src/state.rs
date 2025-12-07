#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    Space,
    Start,
    Beam,
    Splitter,
}

impl State {
    #[must_use]
    pub fn parse(value: char, allow_beam: bool) -> Option<Self> {
        match value {
            '.' => Some(Self::Space),
            'S' => Some(Self::Start),
            '^' => Some(Self::Splitter),
            '|' if allow_beam => Some(Self::Beam),
            _ => None,
        }
    }
}
