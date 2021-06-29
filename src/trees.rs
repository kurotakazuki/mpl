//! Trees

use crate::choices::{Choice, First, Second};
use crate::span::Spanned;
use crate::symbols::{Equivalence, Metasymbol, TerminalSymbol};

/// Leaf Node
pub type Leaf<O = ()> = TerminalSymbol<O>;
/// Internal Node
pub type Internal<V, S, O = ()> = Equivalence<(V, Option<O>), Box<Choice<AST<V, S, O>>>>;

impl<V, S, O> Internal<V, S, O> {
    pub fn from_first(value: (V, Option<O>), l: AST<V, S, O>, r: AST<V, S, O>) -> Self {
        Equivalence::new(value, Box::new((l, r).into()))
    }

    pub fn from_second(value: (V, Option<O>), e: AST<V, S, O>) -> Self {
        Equivalence::new(value, Box::new(e.into()))
    }

    pub fn as_first(&self) -> Option<&First<AST<V, S, O>>> {
        self.equal.as_first()
    }

    pub fn as_second(&self) -> Option<&Second<AST<V, S, O>>> {
        self.equal.as_second()
    }

    pub fn into_first(self) -> Option<First<AST<V, S, O>>> {
        self.equal.into_first()
    }

    pub fn into_second(self) -> Option<Second<AST<V, S, O>>> {
        self.equal.into_second()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Node<V, S, O = ()> {
    /// Leaf Node
    Leaf(Leaf<O>),
    /// Internal Node
    Internal(Internal<V, S, O>),
}

impl<V, S, O> From<Leaf<O>> for Node<V, S, O> {
    fn from(leaf: Leaf<O>) -> Self {
        Self::Leaf(leaf)
    }
}

impl<V, S, O> From<Internal<V, S, O>> for Node<V, S, O> {
    fn from(internal: Internal<V, S, O>) -> Self {
        Self::Internal(internal)
    }
}

impl<V, S, O> Node<V, S, O> {
    pub fn as_leaf(&self) -> Option<&Leaf<O>> {
        match self {
            Self::Leaf(leaf) => Some(leaf),
            Self::Internal(_) => None,
        }
    }

    pub fn as_internal(&self) -> Option<&Internal<V, S, O>> {
        match self {
            Self::Leaf(_) => None,
            Self::Internal(internal) => Some(internal),
        }
    }

    pub fn into_leaf(self) -> Option<Leaf<O>> {
        match self {
            Self::Leaf(leaf) => Some(leaf),
            Self::Internal(_) => None,
        }
    }

    pub fn into_internal(self) -> Option<Internal<V, S, O>> {
        match self {
            Self::Leaf(_) => None,
            Self::Internal(internal) => Some(internal),
        }
    }
}

/// AST
pub type AST<V, S, O = ()> = Spanned<Node<V, S, O>, S>;
/// CST
pub type CST<V, S, O = ()> = Spanned<Equivalence<V, Choice<AST<V, S, O>>>, S>;

impl<V, S, O> AST<V, S, O> {
    // Leaf Node
    pub fn from_leaf(leaf: Leaf<O>, span: S) -> Self {
        Self::new(leaf.into(), span)
    }

    // Internal Node
    pub fn from_internal(internal: Internal<V, S, O>, span: S) -> Self {
        Self::new(internal.into(), span)
    }

    pub fn from_cst_and_output(cst: CST<V, S, O>, output: Option<O>) -> Self {
        match cst.node.equal {
            Choice::First(first) => Self::from_internal(
                Internal::from_first((cst.node.value, output), first.lhs, first.rhs),
                cst.span,
            ),
            Choice::Second(second) => Self::from_internal(
                Internal::from_second((cst.node.value, output), second.0),
                cst.span,
            ),
        }
    }

    pub fn from_cst(cst: CST<V, S, O>) -> Self {
        Self::from_cst_and_output(cst, None)
    }

    pub fn as_leaf(&self) -> Option<&Leaf<O>> {
        self.node.as_leaf()
    }

    pub fn as_internal(&self) -> Option<&Internal<V, S, O>> {
        self.node.as_internal()
    }

    pub fn into_leaf(self) -> Option<Leaf<O>> {
        self.node.into_leaf()
    }

    pub fn into_internal(self) -> Option<Internal<V, S, O>> {
        self.node.into_internal()
    }

    pub fn as_first(&self) -> Option<&First<AST<V, S, O>>> {
        self.as_internal().and_then(|n| n.as_first())
    }

    pub fn as_second(&self) -> Option<&Second<AST<V, S, O>>> {
        self.as_internal().and_then(|n| n.as_second())
    }

    pub fn into_first(self) -> Option<First<AST<V, S, O>>> {
        self.into_internal().and_then(|n| n.into_first())
    }

    pub fn into_second(self) -> Option<Second<AST<V, S, O>>> {
        self.into_internal().and_then(|n| n.into_second())
    }

    pub fn as_original(&self) -> Option<&O> {
        self.as_leaf().and_then(|n| n.as_original())
    }

    pub fn as_metasymbol(&self) -> Option<&Metasymbol> {
        self.as_leaf().and_then(|n| n.as_metasymbol())
    }

    pub fn into_original(self) -> Option<O> {
        self.into_leaf().and_then(|n| n.into_original())
    }

    pub fn into_metasymbol(self) -> Option<Metasymbol> {
        self.into_leaf().and_then(|n| n.into_metasymbol())
    }
}

impl<V, S, O> CST<V, S, O> {
    pub fn as_first(&self) -> Option<&First<AST<V, S, O>>> {
        self.node.equal.as_first()
    }

    pub fn as_second(&self) -> Option<&Second<AST<V, S, O>>> {
        self.node.equal.as_second()
    }

    pub fn into_first(self) -> Option<First<AST<V, S, O>>> {
        self.node.equal.into_first()
    }

    pub fn into_second(self) -> Option<Second<AST<V, S, O>>> {
        self.node.equal.into_second()
    }
}

impl<O, V, S: Clone> CST<V, S, O> {
    pub fn into_omit(mut self) -> Self {
        self.node.equal = AST::from_leaf(Metasymbol::Omit.into(), self.span.clone()).into();
        self
    }
}
