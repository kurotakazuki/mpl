use crate::tree::{LeafNode, AST};
use crate::position::BytePos;
use crate::span::{ByteSpan, Span};
use crate::symbols::{Metasymbol, Terminal};

use std::cmp::PartialEq;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SliceTerminal<'a, T> {
    Element(T),
    Slice(&'a [T]),
}

impl<T> From<T> for SliceTerminal<'_, T> {
    fn from(element: T) -> Self {
        Self::Element(element)
    }
}

impl<'a, T> From<&'a [T]> for SliceTerminal<'a, T> {
    fn from(slice: &'a [T]) -> Self {
        Self::Slice(slice)
    }
}

impl<'a, T: PartialEq, OutputT, V> Terminal<'a, [T], OutputT, V, ByteSpan, BytePos>
    for SliceTerminal<'a, T>
{
    fn eval(
        &'a self,
        input: &'a [T],
        pos: BytePos,
        max_pos: &BytePos,
    ) -> Result<AST<OutputT, V, ByteSpan>, ()> {
        match self {
            SliceTerminal::Element(element) => {
                let start = pos;
                let pos: usize = pos.0 as usize;
                // Length is 1.
                let span = ByteSpan::from_start_len(start, 1);

                if &span.hi() <= max_pos {
                    // let e = unsafe { input.get_unchecked(pos) };
                    if let Some(e) = input.get(pos) {
                        if element == e {
                            return Ok(AST::<OutputT, V, ByteSpan>::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }
            SliceTerminal::Slice(slice) => {
                let start = pos;
                let pos: usize = pos.0 as usize;
                let len = slice.len();
                let span = ByteSpan::from_start_len(start, len as u16);

                if &span.hi() <= max_pos {
                    if let Some(ref s) = input.get(pos..pos + len) {
                        if slice == s {
                            return Ok(AST::<OutputT, V, ByteSpan>::from_leaf_node(
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
        let c = SliceTerminal::from('A');
        let s = SliceTerminal::from(&['a', 'b', 'c'][..]);

        assert_eq!(c, SliceTerminal::Element('A'));
        assert_eq!(s, SliceTerminal::Slice(&['a', 'b', 'c']));
    }
}
