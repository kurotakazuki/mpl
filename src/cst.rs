use crate::choice::Choice;
use crate::span::Spanned;
use crate::symbols::{TerminalSymbol, Variable};

pub type LeafNode<OutputT> = TerminalSymbol<OutputT>;
pub type InternalNode<OutputT, V, S> = Variable<V, Box<Choice<CST<OutputT, V, S>>>>;

impl<OutputT, V, S> InternalNode<OutputT, V, S> {
    pub fn from_first(v: V, l: CST<OutputT, V, S>, r: CST<OutputT, V, S>) -> Self {
        Variable::new(v, Box::new(Choice::first(l, r)))
    }

    pub fn from_second(v: V, e: CST<OutputT, V, S>) -> Self {
        Variable::new(v, Box::new(Choice::second(e)))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CSTKind<OutputT, V, S> {
    /// Terminal symbol
    LeafNode(LeafNode<OutputT>),
    /// Viriable
    InternalNode(InternalNode<OutputT, V, S>),
    // InternalNode { variable: V, choice: Box<Choice<CST<OutputT, V, S, L>>> },
}

pub type CST<OutputT, V, S> = Spanned<CSTKind<OutputT, V, S>, S>;

impl<OutputT, V, S> CST<OutputT, V, S> {
    pub fn from_leaf_node(leaf_node: LeafNode<OutputT>, span: S) -> Self {
        Self::new(CSTKind::LeafNode(leaf_node), span)
    }

    pub fn from_internal_node(internal_node: InternalNode<OutputT, V, S>, span: S) -> Self {
        Self::new(CSTKind::InternalNode(internal_node), span)
    }
}
