use crate::span::Span;
use std::ops;

#[derive(Clone, Debug, PartialEq)]
pub struct StartAndLenSpan<P, L> {
    pub start: P,
    pub len: L,
}

pub trait StartAndLenSpanHi<L> {
    fn hi_from_start_and_len(start: Self, len: L) -> Self;
}

pub trait StartAndLenSpanLen<P> {
    fn len_from_lo_and_hi(lo: P, hi: P) -> Self;
}

impl<P, L> StartAndLenSpan<P, L> {
    pub fn from_start_len(start: P, len: L) -> Self {
        Self { start, len }
    }
}

impl<P, L> Span<P> for StartAndLenSpan<P, L>
where
    P: Clone + Ord + StartAndLenSpanHi<L>,
    L: Clone + Ord + StartAndLenSpanLen<P>,
{
    fn from_lo_hi(lo: P, hi: P) -> Self {
        Self {
            start: lo.clone(),
            len: L::len_from_lo_and_hi(lo, hi),
        }
    }

    fn with_lo(&self, lo: P) -> Self {
        Self::from_lo_hi(lo, self.hi().clone())
    }

    fn with_hi(&self, hi: P) -> Self {
        Self::from_lo_hi(self.lo().clone(), hi)
    }

    fn lo(&self) -> P {
        self.start.clone()
    }

    fn hi(&self) -> P {
        P::hi_from_start_and_len(self.start.clone(), self.len.clone())
    }

    fn stretch(&self, other: &Self) -> Self {
        use std::cmp::{max, min};

        let lo = min(self.lo(), other.lo());
        let hi = max(self.hi(), other.hi());

        Self::from_lo_hi(lo.clone(), hi.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl StartAndLenSpanHi<u16> for u32 {
        fn hi_from_start_and_len(start: Self, len: u16) -> Self {
            start + len as u32
        }
    }

    impl StartAndLenSpanLen<u32> for u16 {
        fn len_from_lo_and_hi(lo: u32, hi: u32) -> Self {
            (hi - lo) as u16
        }
    }

    #[test]
    fn from_lo_hi_input() {
        let span = StartAndLenSpan::<u32, u16>::from_lo_hi(1, 10);

        assert_eq!(1, span.start);
        assert_eq!(10, span.hi());
    }

    #[test]
    fn stretch() {
        let span = StartAndLenSpan::<u32, u16>::from_lo_hi(5, 100);
        let other_span = StartAndLenSpan::from_lo_hi(1, 10);

        let another_span = span.stretch(&other_span);

        assert_eq!(1, another_span.start);
        assert_eq!(100, another_span.hi());

        let span = StartAndLenSpan::<u32, u16>::from_lo_hi(5, 100);
        let other_span = StartAndLenSpan::from_lo_hi(1, 255);

        let another_span = span.stretch(&other_span);

        assert_eq!(StartAndLenSpan::from_lo_hi(1, 255), another_span);
    }
}
