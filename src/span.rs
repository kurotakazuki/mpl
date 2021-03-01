use std::convert::TryFrom;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpanData<S, L> {
    lo: S,
    hi: S,
    phantom: PhantomData<L>,
}

impl<S, L> SpanData<S, L>
where
    S: Add<Output = S> + Sub<Output = S> + Copy + Ord + From<L>,
    L: Add<Output = L> + Sub<Output = L> + Copy + Default + Ord + TryFrom<S>,
    L::Error: Debug,
{
    #[inline]
    pub fn with_lo(&self, lo: S) -> Span<S, L> {
        Span::new(lo, self.hi)
    }
    #[inline]
    pub fn with_hi(&self, hi: S) -> Span<S, L> {
        Span::new(self.lo, hi)
    }
}

/// Recommend that `S` has a wider scope of values than `L`.
/// `hi = span.start + span.len` must be `<= S::MAX`.
/// `span.len` must be `<= L::MAX`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span<S, L> {
    start: S,
    len: L,
}

impl<S, L> Span<S, L>
where
    S: Add<Output = S> + Sub<Output = S> + Copy + Ord + From<L>,
    L: Add<Output = L> + Sub<Output = L> + Copy + Default + Ord + TryFrom<S>,
    L::Error: Debug,
{
    /// `span.len` must be `<= L::MAX`.
    pub fn new(lo: S, hi: S) -> Self {
        Self {
            start: lo,
            len: L::try_from(hi - lo).expect("hi - lo <= L::MAX"),
        }
    }

    /// `hi = span.start + span.len` must be `<= S::MAX`.
    pub fn from_S_L(start: S, len: L) -> Self {
        Self { start, len }
    }

    /// Warning: This may overflow.
    pub fn data(&self) -> SpanData<S, L> {
        SpanData {
            lo: self.start,
            // Warning: This may overflow.
            hi: self.start + self.len.into(),
            phantom: PhantomData,
        }
    }

    pub fn lo(&self) -> S {
        self.data().lo
    }

    pub fn hi(&self) -> S {
        self.data().hi
    }

    pub fn with_lo(&self, lo: S) -> Self {
        self.data().with_lo(lo)
    }

    pub fn with_hi(&self, hi: S) -> Self {
        self.data().with_hi(hi)
    }

    pub fn merge(&self, other: &Self) -> Self {
        use std::cmp::{max, min};

        let lo = min(self.lo(), other.lo());
        let hi = max(self.hi(), other.hi());

        Self::new(lo, hi)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_new_lo_hi() {
        let span = Span::<u32, u16>::new(1, 10);

        assert_eq!(1, span.lo());
        assert_eq!(10, span.hi());
    }

    #[test]
    fn span_merge() {
        let span = Span::<u32, u16>::new(5, 100);
        let other_span = Span::new(1, 10);

        let another_span = span.merge(&other_span);

        assert_eq!(1, another_span.lo());
        assert_eq!(100, another_span.hi());

        let span = Span::<u16, u8>::new(5, 100);
        let other_span = Span::new(1, 255);

        let another_span = span.merge(&other_span);

        assert_eq!(Span::new(1, 255), another_span);
    }
}
