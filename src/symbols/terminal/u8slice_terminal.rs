use crate::span::{Len, Span, Start, StartAndLenSpan};
use crate::symbols::{Metasymbol, Terminal};
use crate::tree::{LeafNode, AST};
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

impl<'a, O, V, P, L> Terminal<'a, [u8], O, V, StartAndLenSpan<P, L>, P> for U8SliceTerminal<'a>
where
    P: Start<[u8], L>,
    L: Len<[u8], P>,
{
    fn eval(
        &'a self,
        input: &'a [u8],
        pos: P,
        max_pos: &P,
    ) -> Result<AST<O, V, StartAndLenSpan<P, L>>, ()> {
        match self {
            Self::Char(c) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = c.len_utf8();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos
                    && &input[pos..pos + len] == c.to_string()[..].as_bytes()
                {
                    Ok(AST::from_leaf_node(
                        LeafNode::from_m(Metasymbol::Omit),
                        span,
                    ))
                } else {
                    Err(())
                }
            }
            Self::Str(s) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let s_bytes = s.as_bytes();
                let len = s_bytes.len();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos && &input[pos..pos + len] == s_bytes {
                    Ok(AST::from_leaf_node(
                        LeafNode::from_m(Metasymbol::Omit),
                        span,
                    ))
                } else {
                    Err(())
                }
            }
            Self::U8Slice(slice) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = slice.len();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(ref s) = input.get(pos..pos + len) {
                        if slice == s {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }
            // TODO
            Self::BEf32(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<f32>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_be_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }
            Self::LEf32(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<f32>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_le_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }

            Self::BEf64(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<f64>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_be_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }
            Self::LEf64(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<f64>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_le_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }

            Self::BEu8(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<u8>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_be_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }
            Self::LEu8(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<u8>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_le_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }

            Self::BEi8(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<i8>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_be_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }
            Self::LEi8(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<i8>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_le_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }

            Self::BEu16(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<u16>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_be_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }
            Self::LEu16(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<u16>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_le_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }

            Self::BEi16(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<i16>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_be_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }
            Self::LEi16(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<i16>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_le_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }

            Self::BEu32(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<u32>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_be_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }
            Self::LEu32(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<u32>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_le_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }

            Self::BEi32(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<i32>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_be_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }
            Self::LEi32(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<i32>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_le_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }

            Self::BEu64(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<u64>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_be_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }
            Self::LEu64(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<u64>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_le_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }

            Self::BEi64(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<i64>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_be_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }
            Self::LEi64(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<i64>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_le_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }

            Self::BEu128(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<u128>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_be_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }
            Self::LEu128(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<u128>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_le_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }

            Self::BEi128(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<i128>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_be_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }
            Self::LEi128(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<i128>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_le_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }

            Self::BEusize(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<usize>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_be_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }
            Self::LEusize(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<usize>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_le_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }

            Self::BEisize(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<isize>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_be_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }
            Self::LEisize(n) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = mem::size_of::<isize>();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos {
                    if let Some(s) = input.get(pos..pos + len) {
                        if s == n.to_le_bytes() {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }
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
