use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span<T> {
    start: T,
    len: T,
}

impl<T> Span<T> where T: Add<Output = T> + Sub<Output = T> + Copy + Ord {
    pub fn new(start: T, len: T) -> Self {
        Self { start, len }
    }

    pub fn lo(&self) -> T {
        self.start
    }

    /// Warning: This may overflow.
    pub fn hi(&self) -> T {
        self.start + self.len
    }

    pub fn merge(&self, other: &Self) -> Self {
        use std::cmp::{max, min};

        let start = min(self.start, other.start);

        Self {
            start,
            len: max(self.hi(), other.hi()) - start,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_new_lo_hi() {
        let span = Span::new(1, 10);

        assert_eq!(1, span.lo());
        assert_eq!(11, span.hi());
    }

    #[test]
    fn span_merge() {
        let span = Span::new(5, 100);
        let other_span = Span::new(1, 10);

        let another_span = span.merge(&other_span);

        assert_eq!(1, another_span.lo());
        assert_eq!(105, another_span.hi());
    }
}