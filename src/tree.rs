use crate::choice::{Choice, First, Second};
use crate::span::Spanned;
use crate::symbols::{Equivalence, Metasymbol, TerminalSymbol};

pub type LeafNode<O> = TerminalSymbol<O>;
pub type InternalNode<O, V, S> = Equivalence<(V, Option<O>), Box<Choice<AST<O, V, S>>>>;

impl<O, V, S> InternalNode<O, V, S> {
    pub fn from_first(value: (V, Option<O>), l: AST<O, V, S>, r: AST<O, V, S>) -> Self {
        Equivalence::new(value, Box::new((l, r).into()))
    }

    pub fn from_second(value: (V, Option<O>), e: AST<O, V, S>) -> Self {
        Equivalence::new(value, Box::new(e.into()))
    }

    pub fn as_first(&self) -> Option<&First<AST<O, V, S>>> {
        self.equal.as_first()
    }

    pub fn as_second(&self) -> Option<&Second<AST<O, V, S>>> {
        self.equal.as_second()
    }

    pub fn into_first(self) -> Option<First<AST<O, V, S>>> {
        self.equal.into_first()
    }

    pub fn into_second(self) -> Option<Second<AST<O, V, S>>> {
        self.equal.into_second()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ASTKind<O, V, S> {
    /// Terminal symbol
    LeafNode(LeafNode<O>),
    /// Viriable
    InternalNode(InternalNode<O, V, S>),
}

impl<O, V, S> From<LeafNode<O>> for ASTKind<O, V, S> {
    fn from(leaf_node: LeafNode<O>) -> Self {
        Self::LeafNode(leaf_node)
    }
}

impl<O, V, S> From<InternalNode<O, V, S>> for ASTKind<O, V, S> {
    fn from(internal_node: InternalNode<O, V, S>) -> Self {
        Self::InternalNode(internal_node)
    }
}

impl<O, V, S> ASTKind<O, V, S> {
    pub fn as_leaf_node(&self) -> Option<&LeafNode<O>> {
        match self {
            Self::LeafNode(leaf_node) => Some(leaf_node),
            Self::InternalNode(_) => None,
        }
    }

    pub fn as_internal_node(&self) -> Option<&InternalNode<O, V, S>> {
        match self {
            Self::LeafNode(_) => None,
            Self::InternalNode(internal_node) => Some(internal_node),
        }
    }

    pub fn into_leaf_node(self) -> Option<LeafNode<O>> {
        match self {
            Self::LeafNode(leaf_node) => Some(leaf_node),
            Self::InternalNode(_) => None,
        }
    }

    pub fn into_internal_node(self) -> Option<InternalNode<O, V, S>> {
        match self {
            Self::LeafNode(_) => None,
            Self::InternalNode(internal_node) => Some(internal_node),
        }
    }
}

pub type AST<O, V, S> = Spanned<ASTKind<O, V, S>, S>;

pub type CST<O, V, S> = Spanned<Equivalence<V, Choice<AST<O, V, S>>>, S>;

impl<O, V, S> AST<O, V, S> {
    // Leaf Node
    pub fn from_leaf_node(leaf_node: LeafNode<O>, span: S) -> Self {
        Self::new(leaf_node.into(), span)
    }

    // Internal Node
    pub fn from_internal_node(internal_node: InternalNode<O, V, S>, span: S) -> Self {
        Self::new(internal_node.into(), span)
    }

    pub fn from_cst_and_output(cst: CST<O, V, S>, output: Option<O>) -> Self {
        match cst.node.equal {
            Choice::First(first) => Self::from_internal_node(
                InternalNode::from_first((cst.node.value, output), first.lhs, first.rhs),
                cst.span,
            ),
            Choice::Second(second) => Self::from_internal_node(
                InternalNode::from_second((cst.node.value, output), second.0),
                cst.span,
            ),
        }
    }

    pub fn from_cst(cst: CST<O, V, S>) -> Self {
        Self::from_cst_and_output(cst, None)
    }

    pub fn as_leaf_node(&self) -> Option<&LeafNode<O>> {
        self.node.as_leaf_node()
    }

    pub fn as_internal_node(&self) -> Option<&InternalNode<O, V, S>> {
        self.node.as_internal_node()
    }

    pub fn as_first(&self) -> Option<&First<AST<O, V, S>>> {
        self.as_internal_node().and_then(|n| n.as_first())
    }

    pub fn as_second(&self) -> Option<&Second<AST<O, V, S>>> {
        self.as_internal_node().and_then(|n| n.as_second())
    }

    pub fn into_leaf_node(self) -> Option<LeafNode<O>> {
        self.node.into_leaf_node()
    }

    pub fn into_internal_node(self) -> Option<InternalNode<O, V, S>> {
        self.node.into_internal_node()
    }

    pub fn into_first(self) -> Option<First<AST<O, V, S>>> {
        self.into_internal_node().and_then(|n| n.into_first())
    }

    pub fn into_second(self) -> Option<Second<AST<O, V, S>>> {
        self.into_internal_node().and_then(|n| n.into_second())
    }
}

impl<O, V, S> CST<O, V, S> {
    pub fn as_first(&self) -> Option<&First<AST<O, V, S>>> {
        self.node.equal.as_first()
    }

    pub fn as_second(&self) -> Option<&Second<AST<O, V, S>>> {
        self.node.equal.as_second()
    }

    pub fn into_first(self) -> Option<First<AST<O, V, S>>> {
        self.node.equal.into_first()
    }

    pub fn into_second(self) -> Option<Second<AST<O, V, S>>> {
        self.node.equal.into_second()
    }
}

impl<O, V, S: Clone> CST<O, V, S> {
    pub fn into_omit(mut self) -> Self {
        self.node.equal =
            AST::from_leaf_node(LeafNode::from_m(Metasymbol::Omit), self.span.clone()).into();
        self
    }
}
