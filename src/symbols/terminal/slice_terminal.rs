use crate::span::{Len, Span, Start, StartAndLenSpan};
use crate::symbols::terminal::StartAndLenResult;
use crate::symbols::{Metasymbol, Terminal};
use crate::trees::AST;
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

impl<'a, T: PartialEq, V, P, L, O> Terminal<'a, [T], V, StartAndLenSpan<P, L>, P, O>
    for SliceTerminal<'a, T>
where
    P: Start<[T], L>,
    L: Len<[T], P>,
{
    fn eval(&self, input: &'a [T], pos: P, max_pos: &P) -> StartAndLenResult<V, P, L, O> {
        let ast_hi_pos = |pos: P, len| {
            let start = pos.clone();
            let pos: usize = P::into_usize(pos, input);
            let span = StartAndLenSpan::from_lo_len(start, len, input);
            let hi = span.hi(input);
            let ast = AST::from_leaf_node(Metasymbol::Omit.into(), span);
            (ast, hi, pos)
        };

        match self {
            SliceTerminal::Element(element) => {
                // Length is 1.
                let (ast, hi, pos) = ast_hi_pos(pos, 1);
                if &hi <= max_pos {
                    // let e = unsafe { input.get_unchecked(pos) };
                    if let Some(e) = input.get(pos) {
                        if element == e {
                            return Ok(ast);
                        }
                    }
                }
                Err(ast)
            }
            SliceTerminal::Slice(slice) => {
                let len = slice.len();
                let (ast, hi, pos) = ast_hi_pos(pos, len);
                if &hi <= max_pos {
                    if let Some(ref s) = input.get(pos..pos + len) {
                        if s == slice {
                            return Ok(ast);
                        }
                    }
                }
                Err(ast)
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
