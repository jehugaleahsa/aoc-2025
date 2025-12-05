#[derive(Debug, Copy, Clone)]
pub struct FreshRange {
    pub start: u64,
    pub end: u64,
}

impl FreshRange {
    #[inline]
    #[must_use]
    pub fn contains(self, id: u64) -> bool {
        id >= self.start && id <= self.end
    }

    #[must_use]
    pub fn count(self) -> u64 {
        self.end - self.start + 1
    }

    #[must_use]
    pub fn try_merge(self, other: Self) -> Option<Self> {
        if self.start <= other.start {
            if self.end >= other.end {
                Some(self)
            } else if self.end >= other.start {
                let range = FreshRange {
                    start: self.start,
                    end: other.end,
                };
                Some(range)
            } else {
                None
            }
        } else if self.end <= other.end {
            Some(other)
        } else if self.start <= other.end {
            let range = FreshRange {
                start: other.start,
                end: self.end,
            };
            Some(range)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fresh_range::FreshRange;

    #[test]
    fn test_range_count_1_10() {
        let range = FreshRange { start: 1, end: 10 };
        assert_eq!(10, range.count()); // Inclusive
    }

    #[test]
    fn test_try_merge_overlapping_tail_and_head() {
        let range1 = FreshRange { start: 1, end: 5 };
        let range2 = FreshRange { start: 3, end: 10 };
        let merged = range1.try_merge(range2).unwrap();
        assert_eq!(1, merged.start);
        assert_eq!(10, merged.end);
    }

    #[test]
    fn test_try_merge_overlapping_head_and_tail() {
        let range1 = FreshRange { start: 3, end: 10 };
        let range2 = FreshRange { start: 1, end: 5 };
        let merged = range1.try_merge(range2).unwrap();
        assert_eq!(1, merged.start);
        assert_eq!(10, merged.end);
    }

    #[test]
    fn test_try_merge_overlapping_wraps() {
        let range1 = FreshRange { start: 1, end: 10 };
        let range2 = FreshRange { start: 3, end: 8 };
        let merged = range1.try_merge(range2).unwrap();
        assert_eq!(1, merged.start);
        assert_eq!(10, merged.end);
    }

    #[test]
    fn test_try_merge_overlapping_embedded() {
        let range1 = FreshRange { start: 3, end: 8 };
        let range2 = FreshRange { start: 1, end: 10 };
        let merged = range1.try_merge(range2).unwrap();
        assert_eq!(1, merged.start);
        assert_eq!(10, merged.end);
    }

    #[test]
    fn test_try_merge_non_overlapping() {
        let range1 = FreshRange { start: 1, end: 5 };
        let range2 = FreshRange { start: 6, end: 10 };
        assert!(range1.try_merge(range2).is_none());
    }
}
