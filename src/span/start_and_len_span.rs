use crate::input::Input;
use crate::position::Position;
use crate::span::Span;
use std::fmt::Debug;
use std::ops;

#[derive(Clone, Debug, PartialEq)]
pub struct StartAndLenSpan<P, L> {
    pub start: P,
    pub len: L,
}

pub trait Start<I, L>: Position
where
    I: Input + ?Sized,
{
    fn into_usize(start: Self, input: &I) -> usize;
    fn start(input: &I) -> Self;
    fn hi_from_start_and_len(start: Self, len: L, input: &I) -> Self;
}

pub trait Len<I, P>: Clone + ops::Add<Output = Self> + PartialEq
where
    I: Input + ?Sized,
{
    fn from_usize(lo: P, len: usize, input: &I) -> Self;
    fn len_from_lo_and_hi(lo: P, hi: P, input: &I) -> Self;
}

impl<P, L> StartAndLenSpan<P, L> {
    pub fn from_start_len(start: P, len: L) -> Self {
        Self { start, len }
    }
}

impl<I, P, L> Span<I, P> for StartAndLenSpan<P, L>
where
    I: Input + ?Sized,
    P: Start<I, L>,
    L: Len<I, P>,
{
    fn lo(&self, _: &I) -> P {
        self.start.clone()
    }

    fn hi(&self, input: &I) -> P {
        P::hi_from_start_and_len(self.start.clone(), self.len.clone(), input)
    }

    // TODO: return Result
    fn from_lo_len(lo: P, len: usize, input: &I) -> Self {
        Self {
            start: lo.clone(),
            // TODO: return Result
            len: L::from_usize(lo, len, input),
        }
    }

    fn from_lo_hi(lo: P, hi: P, input: &I) -> Self {
        Self {
            start: lo.clone(),
            len: L::len_from_lo_and_hi(lo, hi, input),
        }
    }

    fn merge_lhs_and_rhs(lhs: &Self, rhs: &Self, input: &I) -> Self {
        Self::from_start_len(lhs.lo(input), lhs.len.clone() + rhs.len.clone())
    }
}

macro_rules! direct_product_impl {
    ($name:ident, $t1:ty, $t2:ty) => (
        $name!($t1, $t2);
        $name!($t2, $t1);
    );

    ($name:ident, $t:ty, $($ts:ty),+ $(,)?) => (
        $name!($t, $t);
        $(
            direct_product_impl!($name, $t, $ts);
        )+
        direct_product_impl!($name, $($ts),+);
    );
}

macro_rules! start_impl {
    ($t1:ty, $t2:ty) => {
        impl<I> Start<I, $t2> for $t1
        where
            I: Input + ?Sized,
        {
            fn into_usize(start: Self, _: &I) -> usize {
                start as usize
            }
            fn start(_: &I) -> Self {
                0 as $t1
            }
            fn hi_from_start_and_len(start: Self, len: $t2, _: &I) -> Self {
                start + len as $t1
            }
        }
    };
}

direct_product_impl!(
    start_impl, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);

macro_rules! len_impl {
    ($t1:ty, $t2:ty) => {
        impl<I> Len<I, $t2> for $t1
        where
            I: Input + ?Sized,
        {
            fn from_usize(_: $t2, len: usize, _: &I) -> Self {
                len as Self
            }
            fn len_from_lo_and_hi(lo: $t2, hi: $t2, _: &I) -> Self {
                (hi - lo) as Self
            }
        }
    };
}

direct_product_impl!(
    len_impl, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::Span;

    #[test]
    fn from_lo_len() {
        let input = "0123456789";
        let span = StartAndLenSpan::<usize, usize>::from_lo_len(1, 9, input);

        assert_eq!(1, span.start);
        assert_eq!(9, span.len);
        assert_eq!(10, span.hi(input));
    }

    #[test]
    fn from_lo_hi() {
        let input = "0123456789";
        let span = StartAndLenSpan::<u32, u16>::from_lo_hi(1, 10, input);

        assert_eq!(1, span.start);
        assert_eq!(9, span.len);
        assert_eq!(10, span.hi(input));
    }

    #[test]
    fn merge_lhs_and_rhs() {
        let input: String = (0..=54)
            .into_iter()
            .map(|n: usize| n.to_string())
            .collect::<Vec<String>>()
            .join("");
        let input: &str = &input;

        let lhs = StartAndLenSpan::<u16, f64>::from_lo_hi(5, 100, input);
        let rhs = StartAndLenSpan::from_lo_hi(100, 1000, input);

        let merged_span = Span::merge_lhs_and_rhs(&lhs, &rhs, input);

        assert_eq!(5, merged_span.start);
        assert_eq!(1000, merged_span.hi(input));

        let lhs = StartAndLenSpan::<u16, f64>::from_lo_hi(0, 5, input);
        let rhs = StartAndLenSpan::from_lo_hi(5, 10, input);

        let merged_span = Span::merge_lhs_and_rhs(&lhs, &rhs, input);

        assert_eq!(0, merged_span.start);
        assert_eq!(10, merged_span.hi(input));

        assert_eq!(StartAndLenSpan::from_lo_hi(0, 10, input), merged_span);
    }
}
