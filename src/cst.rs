use crate::choice::Choice;
use crate::span::{Span, Spanned};
use crate::symbols::{TerminalSymbol, Variable};

pub type LeafNode<T> = TerminalSymbol<T>;
pub type InternalNode<T, V, S, L> = Variable<V, Box<Choice<CST<T, V, S, L>>>>;

impl<T, V, S, L> InternalNode<T, V, S, L> {
    pub fn from_first(v: V, l: CST<T, V, S, L>, r: CST<T, V, S, L>) -> Self {
        Variable::new(v, Box::new(Choice::first(l, r)))
    }

    pub fn from_second(v: V, e: CST<T, V, S, L>) -> Self {
        Variable::new(v, Box::new(Choice::second(e)))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CSTKind<T, V, S, L> {
    /// Terminal symbol
    LeafNode(LeafNode<T>),
    /// Viriable
    InternalNode(InternalNode<T, V, S, L>),
    // InternalNode { variable: V, choice: Box<Choice<CST<T, V, S, L>>> },
}

pub type CST<T, V, S, L> = Spanned<CSTKind<T, V, S, L>, S, L>;

impl<T, V, S, L> CST<T, V, S, L> {
    pub fn leaf_node(leaf_node: LeafNode<T>, span: Span<S, L>) -> Self {
        Self::new(CSTKind::LeafNode(leaf_node), span)
    }

    pub fn internal_node(internal_node: InternalNode<T, V, S, L>, span: Span<S, L>) -> Self {
        Self::new(CSTKind::InternalNode(internal_node), span)
    }
}
