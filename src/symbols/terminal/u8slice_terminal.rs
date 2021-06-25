use crate::span::{Len, Span, Start, StartAndLenSpan};
use crate::symbols::terminal::StartAndLenResult;
use crate::symbols::{Metasymbol, Terminal};
use crate::tree::AST;
use std::cmp::PartialEq;
use std::mem;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum U8SliceTerminal<'a> {
    Char(char),
    Str(&'a str),
    U8Slice(&'a [u8]),
    // Big Endian
    BEF32(f32),
    BEF64(f64),
    BEU8(u8),
    BEI8(i8),
    BEU16(u16),
    BEI16(i16),
    BEU32(u32),
    BEI32(i32),
    BEU64(u64),
    BEI64(i64),
    BEU128(u128),
    BEI128(i128),
    BEUsize(usize),
    BEIsize(isize),
    // Little Endian
    LEF32(f32),
    LEF64(f64),
    LEU8(u8),
    LEI8(i8),
    LEU16(u16),
    LEI16(i16),
    LEU32(u32),
    LEI32(i32),
    LEU64(u64),
    LEI64(i64),
    LEU128(u128),
    LEI128(i128),
    LEUsize(usize),
    LEIsize(isize),
}

impl From<char> for U8SliceTerminal<'_> {
    fn from(c: char) -> Self {
        Self::Char(c)
    }
}

impl<'a> From<&'a str> for U8SliceTerminal<'a> {
    fn from(s: &'a str) -> Self {
        Self::Str(s)
    }
}

impl<'a> From<&'a [u8]> for U8SliceTerminal<'a> {
    fn from(s: &'a [u8]) -> Self {
        Self::U8Slice(s)
    }
}

impl<'a, V, P, L, O> Terminal<'a, [u8], V, StartAndLenSpan<P, L>, P, O> for U8SliceTerminal<'a>
where
    P: Start<[u8], L>,
    L: Len<[u8], P>,
{
    fn eval(&self, input: &'a [u8], pos: P, max_pos: &P) -> StartAndLenResult<V, P, L, O> {
        let eval_from = |len: usize, slice: &[u8]| {
            let start = pos.clone();
            let pos: usize = P::into_usize(pos, input);
            let span = StartAndLenSpan::from_lo_len(start, len, input);
            let hi = span.hi(input);
            let ast = AST::from_leaf_node(Metasymbol::Omit.into(), span);
            if &hi <= max_pos {
                if let Some(s) = input.get(pos..pos + len) {
                    if s == slice {
                        return Ok(ast);
                    }
                }
            }
            Err(ast)
        };

        match self {
            // TODO: create test
            Self::Char(c) => eval_from(c.len_utf8(), c.to_string().as_bytes()),
            // TODO: create test
            Self::Str(s) => eval_from(s.len(), s.as_bytes()),
            Self::U8Slice(slice) => eval_from(slice.len(), slice),

            Self::BEF32(n) => eval_from(mem::size_of::<f32>(), &n.to_be_bytes()),
            Self::LEF32(n) => eval_from(mem::size_of::<f32>(), &n.to_le_bytes()),
            Self::BEF64(n) => eval_from(mem::size_of::<f64>(), &n.to_be_bytes()),
            Self::LEF64(n) => eval_from(mem::size_of::<f64>(), &n.to_le_bytes()),

            Self::BEU8(n) => eval_from(mem::size_of::<u8>(), &n.to_be_bytes()),
            Self::LEU8(n) => eval_from(mem::size_of::<u8>(), &n.to_le_bytes()),
            Self::BEI8(n) => eval_from(mem::size_of::<i8>(), &n.to_be_bytes()),
            Self::LEI8(n) => eval_from(mem::size_of::<i8>(), &n.to_le_bytes()),

            Self::BEU16(n) => eval_from(mem::size_of::<u16>(), &n.to_be_bytes()),
            Self::LEU16(n) => eval_from(mem::size_of::<u16>(), &n.to_le_bytes()),
            Self::BEI16(n) => eval_from(mem::size_of::<i16>(), &n.to_be_bytes()),
            Self::LEI16(n) => eval_from(mem::size_of::<i16>(), &n.to_le_bytes()),

            Self::BEU32(n) => eval_from(mem::size_of::<u32>(), &n.to_be_bytes()),
            Self::LEU32(n) => eval_from(mem::size_of::<u32>(), &n.to_le_bytes()),
            Self::BEI32(n) => eval_from(mem::size_of::<i32>(), &n.to_be_bytes()),
            Self::LEI32(n) => eval_from(mem::size_of::<i32>(), &n.to_le_bytes()),

            Self::BEU64(n) => eval_from(mem::size_of::<u64>(), &n.to_be_bytes()),
            Self::LEU64(n) => eval_from(mem::size_of::<u64>(), &n.to_le_bytes()),
            Self::BEI64(n) => eval_from(mem::size_of::<i64>(), &n.to_be_bytes()),
            Self::LEI64(n) => eval_from(mem::size_of::<i64>(), &n.to_le_bytes()),

            Self::BEU128(n) => eval_from(mem::size_of::<u128>(), &n.to_be_bytes()),
            Self::LEU128(n) => eval_from(mem::size_of::<u128>(), &n.to_le_bytes()),
            Self::BEI128(n) => eval_from(mem::size_of::<i128>(), &n.to_be_bytes()),
            Self::LEI128(n) => eval_from(mem::size_of::<i128>(), &n.to_le_bytes()),

            Self::BEUsize(n) => eval_from(mem::size_of::<usize>(), &n.to_be_bytes()),
            Self::LEUsize(n) => eval_from(mem::size_of::<usize>(), &n.to_le_bytes()),
            Self::BEIsize(n) => eval_from(mem::size_of::<isize>(), &n.to_be_bytes()),
            Self::LEIsize(n) => eval_from(mem::size_of::<isize>(), &n.to_le_bytes()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert() {
        let c = U8SliceTerminal::from('A');
        let s = U8SliceTerminal::from(&[0, 1, 2][..]);

        assert_eq!(c, U8SliceTerminal::Char('A'));
        assert_eq!(s, U8SliceTerminal::U8Slice(&[0, 1, 2]));
    }
}
