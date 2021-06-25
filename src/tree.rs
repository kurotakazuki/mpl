use crate::choice::{Choice, First, Second};
use crate::span::Spanned;
use crate::symbols::{Equivalence, Metasymbol, TerminalSymbol};

pub type LeafNode<O> = TerminalSymbol<O>;
pub type InternalNode<V, S, O> = Equivalence<(V, Option<O>), Box<Choice<AST<V, S, O>>>>;

impl<V, S, O> InternalNode<V, S, O> {
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
pub enum ASTKind<V, S, O> {
    /// Terminal symbol
    LeafNode(LeafNode<O>),
    /// Viriable
    InternalNode(InternalNode<V, S, O>),
}

impl<V, S, O> From<LeafNode<O>> for ASTKind<V, S, O> {
    fn from(leaf_node: LeafNode<O>) -> Self {
        Self::LeafNode(leaf_node)
    }
}

impl<V, S, O> From<InternalNode<V, S, O>> for ASTKind<V, S, O> {
    fn from(internal_node: InternalNode<V, S, O>) -> Self {
        Self::InternalNode(internal_node)
    }
}

impl<V, S, O> ASTKind<V, S, O> {
    pub fn as_leaf_node(&self) -> Option<&LeafNode<O>> {
        match self {
            Self::LeafNode(leaf_node) => Some(leaf_node),
            Self::InternalNode(_) => None,
        }
    }

    pub fn as_internal_node(&self) -> Option<&InternalNode<V, S, O>> {
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

    pub fn into_internal_node(self) -> Option<InternalNode<V, S, O>> {
        match self {
            Self::LeafNode(_) => None,
            Self::InternalNode(internal_node) => Some(internal_node),
        }
    }
}

pub type AST<V, S, O> = Spanned<ASTKind<V, S, O>, S>;

pub type CST<V, S, O> = Spanned<Equivalence<V, Choice<AST<V, S, O>>>, S>;

impl<V, S, O> AST<V, S, O> {
    // Leaf Node
    pub fn from_leaf_node(leaf_node: LeafNode<O>, span: S) -> Self {
        Self::new(leaf_node.into(), span)
    }

    // Internal Node
    pub fn from_internal_node(internal_node: InternalNode<V, S, O>, span: S) -> Self {
        Self::new(internal_node.into(), span)
    }

    pub fn from_cst_and_output(cst: CST<V, S, O>, output: Option<O>) -> Self {
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

    pub fn from_cst(cst: CST<V, S, O>) -> Self {
        Self::from_cst_and_output(cst, None)
    }

    pub fn as_leaf_node(&self) -> Option<&LeafNode<O>> {
        self.node.as_leaf_node()
    }

    pub fn as_internal_node(&self) -> Option<&InternalNode<V, S, O>> {
        self.node.as_internal_node()
    }

    pub fn into_leaf_node(self) -> Option<LeafNode<O>> {
        self.node.into_leaf_node()
    }

    pub fn into_internal_node(self) -> Option<InternalNode<V, S, O>> {
        self.node.into_internal_node()
    }

    pub fn as_first(&self) -> Option<&First<AST<V, S, O>>> {
        self.as_internal_node().and_then(|n| n.as_first())
    }

    pub fn as_second(&self) -> Option<&Second<AST<V, S, O>>> {
        self.as_internal_node().and_then(|n| n.as_second())
    }

    pub fn into_first(self) -> Option<First<AST<V, S, O>>> {
        self.into_internal_node().and_then(|n| n.into_first())
    }

    pub fn into_second(self) -> Option<Second<AST<V, S, O>>> {
        self.into_internal_node().and_then(|n| n.into_second())
    }

    pub fn as_original(&self) -> Option<&O> {
        self.as_leaf_node().and_then(|n| n.as_original())
    }

    pub fn as_metasymbol(&self) -> Option<&Metasymbol> {
        self.as_leaf_node().and_then(|n| n.as_metasymbol())
    }

    pub fn into_original(self) -> Option<O> {
        self.into_leaf_node().and_then(|n| n.into_original())
    }

    pub fn into_metasymbol(self) -> Option<Metasymbol> {
        self.into_leaf_node().and_then(|n| n.into_metasymbol())
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
        self.node.equal = AST::from_leaf_node(Metasymbol::Omit.into(), self.span.clone()).into();
        self
    }
}
