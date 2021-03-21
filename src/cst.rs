use crate::choice::Choice;
use crate::span::Spanned;
use crate::symbols::{TerminalSymbol, Variable};

pub type LeafNode<T> = TerminalSymbol<T>;
pub type InternalNode<T, V, S> = Variable<V, Box<Choice<CST<T, V, S>>>>;

impl<T, V, S> InternalNode<T, V, S> {
    pub fn from_first(v: V, l: CST<T, V, S>, r: CST<T, V, S>) -> Self {
        Variable::new(v, Box::new(Choice::first(l, r)))
    }

    pub fn from_second(v: V, e: CST<T, V, S>) -> Self {
        Variable::new(v, Box::new(Choice::second(e)))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CSTKind<T, V, S> {
    /// Terminal symbol
    LeafNode(LeafNode<T>),
    /// Viriable
    InternalNode(InternalNode<T, V, S>),
    // InternalNode { variable: V, choice: Box<Choice<CST<T, V, S, L>>> },
}

pub type CST<T, V, S> = Spanned<CSTKind<T, V, S>, S>;

impl<T, V, S> CST<T, V, S> {
    pub fn leaf_node(leaf_node: LeafNode<T>, span: S) -> Self {
        Self::new(CSTKind::LeafNode(leaf_node), span)
    }

    pub fn internal_node(internal_node: InternalNode<T, V, S>, span: S) -> Self {
        Self::new(CSTKind::InternalNode(internal_node), span)
    }
}
