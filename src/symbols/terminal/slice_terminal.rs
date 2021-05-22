use crate::span::{Len, Span, Start, StartAndLenSpan};
use crate::symbols::{Metasymbol, Terminal};
use crate::tree::{LeafNode, AST};
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

impl<'a, T: PartialEq, OutputT, V, P, L> Terminal<'a, [T], OutputT, V, StartAndLenSpan<P, L>, P>
    for SliceTerminal<'a, T>
where
    P: Start<[T], L>,
    L: Len<[T], P>,
{
    fn eval(
        &'a self,
        input: &'a [T],
        pos: P,
        max_pos: &P,
    ) -> Result<AST<OutputT, V, StartAndLenSpan<P, L>>, ()> {
        match self {
            SliceTerminal::Element(element) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                // Length is 1.
                let span = StartAndLenSpan::from_lo_len(start, 1, input);
                if &span.hi(input) <= max_pos {
                    // let e = unsafe { input.get_unchecked(pos) };
                    if let Some(e) = input.get(pos) {
                        if element == e {
                            return Ok(AST::from_leaf_node(
                                LeafNode::from_m(Metasymbol::Omit),
                                span,
                            ));
                        }
                    }
                }
                Err(())
            }
            SliceTerminal::Slice(slice) => {
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
