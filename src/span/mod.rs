pub use self::byte_span::ByteSpan;
pub use self::spanned::Spanned;
pub use self::start_and_len_span::{StartAndLenSpan, StartAndLenSpanHi, StartAndLenSpanLen};

use crate::position::Position;

mod byte_span;
mod spanned;
mod start_and_len_span;

pub trait Span<P>: Clone where P: Position {
    fn from_lo_hi(lo: P, hi: P) -> Self;
    fn with_lo(&self, lo: P) -> Self;
    fn with_hi(&self, hi: P) -> Self;
    fn lo(&self) -> P;
    fn hi(&self) -> P;
    // fn stretch(&self, other: &Self) -> Self;

    /// lhs.hi() and rhs.lo() must be equal.
    fn merge_lhs_and_rhs(lhs: &Self, rhs: &Self) -> Self;
}

