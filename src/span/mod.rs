// pub use self::byte_span::ByteSpan;
pub use self::spanned::Spanned;
pub use self::start_and_len_span::{Len, Start, StartAndLenSpan};

use crate::input::Input;
use crate::position::Position;

mod byte_span;
mod spanned;
mod start_and_len_span;

pub trait Span<I, P>: Clone + PartialEq
where
    I: Input + ?Sized,
    P: Position,
{
    fn lo(&self, input: &I) -> P;
    fn hi(&self, input: &I) -> P;
    fn from_lo_len(lo: P, len: usize, input: &I) -> Self;
    fn from_lo_hi(lo: P, hi: P, input: &I) -> Self;
    /// lhs.hi() and rhs.lo() must be equal.
    fn merge_lhs_and_rhs(lhs: &Self, rhs: &Self, input: &I) -> Self;
}
