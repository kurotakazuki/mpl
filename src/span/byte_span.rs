use crate::position::BytePos;
use crate::span::{Span, SpanHi, SpanLen};

pub type ByteSpan = Span<BytePos, u16>;

impl<I> SpanHi<u16, I> for BytePos {
    fn hi(start: Self, len: u16, _: &I) -> Self {
        start + BytePos(len as u32)
    }
}

impl<I> SpanLen<BytePos, I> for u16 {
    fn len(lo: BytePos, hi: BytePos, _: &I) -> Self {
        u32::from(hi - lo) as u16
    }
}
