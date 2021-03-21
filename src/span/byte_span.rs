use crate::position::BytePos;
use crate::span::{StartAndLenSpan, StartAndLenSpanHi, StartAndLenSpanLen};

pub type ByteSpan = StartAndLenSpan<BytePos, u16>;

impl StartAndLenSpanHi<u16> for BytePos {
    fn hi_from_start_and_len(start: Self, len: u16) -> Self {
        start + BytePos(len as u32)
    }
}

impl StartAndLenSpanLen<BytePos> for u16 {
    fn len_from_lo_and_hi(lo: BytePos, hi: BytePos) -> Self {
        u32::from(hi - lo) as u16
    }
}
