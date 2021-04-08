use crate::position::BytePos;
use crate::span::{ByteSpan, Span};
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

impl<'a, OutputT, V> Terminal<'a, str, OutputT, V, ByteSpan, BytePos> for StrTerminal<'a> {
    fn eval(
        &'a self,
        input: &'a str,
        pos: BytePos,
        max_pos: &BytePos,
    ) -> Result<AST<OutputT, V, ByteSpan>, ()> {
        match self {
            StrTerminal::Char(c) => {
                let start = pos;
                let pos: usize = pos.0 as usize;
                let len = c.len_utf8();
                let span = ByteSpan::from_start_len(start, len as u16);
                if &span.hi() <= max_pos
                    && &input.as_bytes()[pos..pos + len] == c.to_string()[..].as_bytes()
                {
                    Ok(AST::<OutputT, V, ByteSpan>::from_leaf_node(
                        LeafNode::from_m(Metasymbol::Omit),
                        span,
                    ))
                } else {
                    Err(())
                }
            }
            StrTerminal::Str(s) => {
                let start = pos;
                let pos: usize = pos.0 as usize;
                let s_bytes = s.as_bytes();
                let len = s_bytes.len();
                let span = ByteSpan::from_start_len(start, len as u16);
                if &span.hi() <= max_pos && &input.as_bytes()[pos..pos + len] == s.as_bytes() {
                    Ok(AST::<OutputT, V, ByteSpan>::from_leaf_node(
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
