use crate::span::Span;

#[derive(Clone, Debug, PartialEq)]
pub struct Spanned<N, S> {
    pub node: N,
    pub span: S,
}

impl<N, S> Spanned<N, S> {
    pub fn new(node: N, span: S) -> Self {
        Spanned { node, span }
    }
}
