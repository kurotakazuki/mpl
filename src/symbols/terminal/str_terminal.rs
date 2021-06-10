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
        let eval_from = |len: usize, string: &str| -> Result<AST<O, V, StartAndLenSpan<P, L>>, ()> {
            let start = pos.clone();
            let pos: usize = P::into_usize(pos, input);
            let span = StartAndLenSpan::from_lo_len(start, len, input);
            if &span.hi(input) <= max_pos {
                if let Some(s) = input.get(pos..pos + len) {
                    if s == string {
                        return Ok(AST::from_leaf_node(
                            LeafNode::from_m(Metasymbol::Omit),
                            span,
                        ));
                    }
                }
            }
            Err(())
        };
        
        match self {
            StrTerminal::Char(c) => eval_from(c.len_utf8(), &c.to_string()),
            StrTerminal::Str(s) => eval_from(s.len(), s),
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
