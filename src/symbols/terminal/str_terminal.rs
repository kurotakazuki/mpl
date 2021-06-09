use crate::span::{Len, Span, Start, StartAndLenSpan};
use crate::symbols::{Metasymbol, Terminal};
use crate::tree::{LeafNode, AST};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StrTerminal<'a> {
    Char(char),
    Str(&'a str),
}

impl From<char> for StrTerminal<'_> {
    fn from(value: char) -> Self {
        Self::Char(value)
    }
}

impl<'a> From<&'a str> for StrTerminal<'a> {
    fn from(value: &'a str) -> Self {
        Self::Str(value)
    }
}

impl<'a, O, V, P, L> Terminal<'a, str, O, V, StartAndLenSpan<P, L>, P> for StrTerminal<'a>
where
    P: Start<str, L>,
    L: Len<str, P>,
{
    fn eval(
        &'a self,
        input: &'a str,
        pos: P,
        max_pos: &P,
    ) -> Result<AST<O, V, StartAndLenSpan<P, L>>, ()> {
        match self {
            StrTerminal::Char(c) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let len = c.len_utf8();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos
                    && &input.as_bytes()[pos..pos + len] == c.to_string()[..].as_bytes()
                {
                    Ok(AST::from_leaf_node(
                        LeafNode::from_m(Metasymbol::Omit),
                        span,
                    ))
                } else {
                    Err(())
                }
            }
            StrTerminal::Str(s) => {
                let start = pos.clone();
                let pos: usize = P::into_usize(pos, input);
                let s_bytes = s.as_bytes();
                let len = s_bytes.len();
                let span = StartAndLenSpan::from_lo_len(start, len, input);
                if &span.hi(input) <= max_pos && &input.as_bytes()[pos..pos + len] == s_bytes {
                    Ok(AST::from_leaf_node(
                        LeafNode::from_m(Metasymbol::Omit),
                        span,
                    ))
                } else {
                    Err(())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert() {
        let c = StrTerminal::from('A');
        let s = StrTerminal::from("abc");

        assert_eq!(c, StrTerminal::Char('A'));
        assert_eq!(s, StrTerminal::Str("abc"));
    }
}
