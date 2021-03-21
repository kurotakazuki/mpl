use crate::span::Span;

#[derive(Clone, Debug, PartialEq)]
pub struct Spanned<N, S, L> {
    pub node: N,
    pub span: Span<S, L>,
}

impl<N, S, L> Spanned<N, S, L> {
    pub fn new(node: N, span: Span<S, L>) -> Self {
        Spanned { node, span }
    }
}
