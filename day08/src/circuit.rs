use crate::junction::Junction;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Circuit {
    junctions: HashSet<Junction>,
}

impl Circuit {
    pub fn new() -> Self {
        let junctions = HashSet::new();
        Self { junctions }
    }

    #[inline]
    pub fn add(&mut self, junction: Junction) {
        self.junctions.insert(junction);
    }

    pub fn merge(&self, other: &Self) -> Self {
        let mut junctions = HashSet::new();
        junctions.extend(&self.junctions);
        junctions.extend(&other.junctions);
        Self { junctions }
    }

    #[inline]
    pub fn junctions(&self) -> impl Iterator<Item = &Junction> {
        self.junctions.iter()
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.junctions.len()
    }
}
