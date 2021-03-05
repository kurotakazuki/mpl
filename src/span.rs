#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span<S, L> {
    start: S,
    len: L,
}

pub trait SpanHi<L, I> {
    fn hi(start: Self, len: L, input: &I) -> Self;
}

pub trait SpanLen<S, I> {
    fn len(lo: S, hi: S, input: &I) -> Self;
}

impl<S, L> Span<S, L> {
    fn from_start_len(start: S, len: L) -> Self {
        Self { start, len }
    }
}

impl<S, L> Span<S, L> where S: Clone {
    fn lo(&self) -> S {
        self.start.clone()
    }
}

trait Spantrait<S, L, I> {
    fn from_lo_hi_input(lo: S, hi: S, input: &I) -> Self;
    fn with_lo(&self, lo: S, input: &I) -> Self;
    fn with_hi(&self, hi: S, input: &I) -> Self;
    fn hi(&self, input: &I) -> S;
    fn stretch(&self, other: &Self, input: &I) -> Self;
}

impl<S, L, I> Spantrait<S, L, I> for Span<S, L>
where
    S: Clone + Ord + From<L> + SpanHi<L, I>,
    L: Clone + Ord + SpanLen<S, I>,
{
    fn from_lo_hi_input(lo: S, hi: S, input: &I) -> Self {
        Self {
            start: lo.clone(),
            len: L::len(lo, hi, input),
        }
    }

    fn with_lo(&self, lo: S, input: &I) -> Self {
        Self::from_lo_hi_input(lo, self.hi(input), input)
    }

    fn with_hi(&self, hi: S, input: &I) -> Self {
        Self::from_lo_hi_input(self.lo(), hi, input)
    }

    fn hi(&self, input: &I) -> S {
        S::hi(self.start.clone(), self.len.clone(), input)
    }

    fn stretch(&self, other: &Self, input: &I) -> Self {
        use std::cmp::{max, min};

        let lo = min(self.lo(), other.lo());
        let hi = max(self.hi(input), other.hi(input));

        Self::from_lo_hi_input(lo, hi, input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl SpanHi<u16, ()> for u32 {
        fn hi(start: Self, len: u16, input: &()) -> Self {
            start + len as u32
        }
    }

    impl SpanLen<u32, ()> for u16 {
        fn len(lo: u32, hi: u32, input: &()) -> Self {
            (hi - lo) as u16
        }
    }

    impl SpanHi<u16, u32> for u32 {
        fn hi(start: Self, len: u16, input: &u32) -> Self {
            input + start + len as u32
        }
    }

    impl SpanLen<u32, u32> for u16 {
        fn len(lo: u32, hi: u32, input: &u32) -> Self {
            (input + hi - lo) as u16
        }
    }

    #[test]
    fn from_lo_hi_input() {
        let span = Span::<u32, u16>::from_lo_hi_input(1, 10, &());

        assert_eq!(1, span.start);
        assert_eq!(10, span.hi(&()));
    }

    #[test]
    fn stretch() {
        let span = Span::<u32, u16>::from_lo_hi_input(5, 100, &());
        let other_span = Span::from_lo_hi_input(1, 10, &());

        let another_span = span.stretch(&other_span, &());

        assert_eq!(1, another_span.start);
        assert_eq!(100, another_span.hi(&()));

        let span = Span::<u32, u16>::from_lo_hi_input(5, 100, &());
        let other_span = Span::from_lo_hi_input(1, 255, &());

        let another_span = span.stretch(&other_span, &());

        assert_eq!(Span::from_lo_hi_input(1, 255, &()), another_span);
    }

    #[test]
    fn input_u32() {
        let span = Span::<u32, u16>::from_lo_hi_input(1, 10, &5);

        assert_eq!(1, span.start);
        assert_eq!(14, span.len);
        assert_eq!(20, u32::hi(span.start, span.len, &5));
        assert_eq!(20, span.hi(&5));
    }
}
