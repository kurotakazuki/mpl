use crate::span::{Len, Span, Start, StartAndLenSpan};
use crate::symbols::terminal::StartAndLenResult;
use crate::symbols::{Metasymbol, Terminal};
use crate::trees::AST;
use std::cmp::PartialEq;
use std::mem;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum U8SliceTerminal<'a> {
    Char(char),
    Str(&'a str),
    U8Slice(&'a [u8]),
    // Big Endian
    BEf32(f32),
    BEf64(f64),
    BEu8(u8),
    BEi8(i8),
    BEu16(u16),
    BEi16(i16),
    BEu32(u32),
    BEi32(i32),
    BEu64(u64),
    BEi64(i64),
    BEu128(u128),
    BEi128(i128),
    BEusize(usize),
    BEisize(isize),
    // Little Endian
    LEf32(f32),
    LEf64(f64),
    LEu8(u8),
    LEi8(i8),
    LEu16(u16),
    LEi16(i16),
    LEu32(u32),
    LEi32(i32),
    LEu64(u64),
    LEi64(i64),
    LEu128(u128),
    LEi128(i128),
    LEusize(usize),
    LEisize(isize),
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
            let ast = AST::from_leaf(Metasymbol::Omit.into(), span);
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

            Self::BEf32(n) => eval_from(mem::size_of::<f32>(), &n.to_be_bytes()),
            Self::LEf32(n) => eval_from(mem::size_of::<f32>(), &n.to_le_bytes()),
            Self::BEf64(n) => eval_from(mem::size_of::<f64>(), &n.to_be_bytes()),
            Self::LEf64(n) => eval_from(mem::size_of::<f64>(), &n.to_le_bytes()),

            Self::BEu8(n) => eval_from(mem::size_of::<u8>(), &n.to_be_bytes()),
            Self::LEu8(n) => eval_from(mem::size_of::<u8>(), &n.to_le_bytes()),
            Self::BEi8(n) => eval_from(mem::size_of::<i8>(), &n.to_be_bytes()),
            Self::LEi8(n) => eval_from(mem::size_of::<i8>(), &n.to_le_bytes()),

            Self::BEu16(n) => eval_from(mem::size_of::<u16>(), &n.to_be_bytes()),
            Self::LEu16(n) => eval_from(mem::size_of::<u16>(), &n.to_le_bytes()),
            Self::BEi16(n) => eval_from(mem::size_of::<i16>(), &n.to_be_bytes()),
            Self::LEi16(n) => eval_from(mem::size_of::<i16>(), &n.to_le_bytes()),

            Self::BEu32(n) => eval_from(mem::size_of::<u32>(), &n.to_be_bytes()),
            Self::LEu32(n) => eval_from(mem::size_of::<u32>(), &n.to_le_bytes()),
            Self::BEi32(n) => eval_from(mem::size_of::<i32>(), &n.to_be_bytes()),
            Self::LEi32(n) => eval_from(mem::size_of::<i32>(), &n.to_le_bytes()),

            Self::BEu64(n) => eval_from(mem::size_of::<u64>(), &n.to_be_bytes()),
            Self::LEu64(n) => eval_from(mem::size_of::<u64>(), &n.to_le_bytes()),
            Self::BEi64(n) => eval_from(mem::size_of::<i64>(), &n.to_be_bytes()),
            Self::LEi64(n) => eval_from(mem::size_of::<i64>(), &n.to_le_bytes()),

            Self::BEu128(n) => eval_from(mem::size_of::<u128>(), &n.to_be_bytes()),
            Self::LEu128(n) => eval_from(mem::size_of::<u128>(), &n.to_le_bytes()),
            Self::BEi128(n) => eval_from(mem::size_of::<i128>(), &n.to_be_bytes()),
            Self::LEi128(n) => eval_from(mem::size_of::<i128>(), &n.to_le_bytes()),

            Self::BEusize(n) => eval_from(mem::size_of::<usize>(), &n.to_be_bytes()),
            Self::LEusize(n) => eval_from(mem::size_of::<usize>(), &n.to_le_bytes()),
            Self::BEisize(n) => eval_from(mem::size_of::<isize>(), &n.to_be_bytes()),
            Self::LEisize(n) => eval_from(mem::size_of::<isize>(), &n.to_le_bytes()),
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
