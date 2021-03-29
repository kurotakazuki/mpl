use crate::position::Position;
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
    P: StartAndLenSpanHi<L> + Position,
    L: Clone + ops::Add<Output = L> + StartAndLenSpanLen<P>,
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

    fn merge_lhs_and_rhs(lhs: &Self, rhs: &Self) -> Self {
        Self::from_start_len(lhs.lo(), lhs.len.clone() + rhs.len.clone())
    }

    // fn stretch(&self, other: &Self) -> Self {
    //     use std::cmp::{max, min};

    //     let self_lo = self.lo();
    //     let other_lo = other.lo();

    //     let lo = if self_lo <= other_lo {
    //         self_lo
    //     } else {
    //         other_lo
    //     };

    //     min(self.lo(), other.lo());
    //     let hi = max(self.hi(), other.hi());

    //     Self::from_lo_hi(lo.clone(), hi.clone())
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::Position;

    impl Position for u32 {
        fn with_one_added(&self) -> Self {
            self + 1
        }
    }

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
    fn merge_lhs_and_rhs() {
        let lhs = StartAndLenSpan::<u32, u16>::from_lo_hi(5, 100);
        let rhs = StartAndLenSpan::from_lo_hi(100, 1000);

        let merged_span = Span::merge_lhs_and_rhs(&lhs, &rhs);

        assert_eq!(5, merged_span.start);
        assert_eq!(1000, merged_span.hi());

        let lhs = StartAndLenSpan::<u32, u16>::from_lo_hi(0, 5);
        let rhs = StartAndLenSpan::from_lo_hi(5, 10);

        let merged_span = Span::merge_lhs_and_rhs(&lhs, &rhs);

        assert_eq!(0, merged_span.start);
        assert_eq!(10, merged_span.hi());

        assert_eq!(StartAndLenSpan::from_lo_hi(0, 10), merged_span);
    }
}
