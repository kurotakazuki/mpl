use crate::choice::Choice;
use crate::span::Spanned;
use crate::symbols::{TerminalSymbol, VAndE};

pub type LeafNode<O> = TerminalSymbol<O>;
pub type InternalNode<O, V, S> = VAndE<(V, Option<O>), Box<Choice<AST<O, V, S>>>>;

// impl LeafNode {
//     pub fn from_t() -> Self {
//         TerminalSymbol::from_t(())
//     }

//     pub fn from_m(metasymbol: Metasymbol) -> Self {
//         TerminalSymbol::from_m(metasymbol)
//     }
// }

impl<O, V, S> InternalNode<O, V, S> {
    pub fn from_first(value: (V, Option<O>), l: AST<O, V, S>, r: AST<O, V, S>) -> Self {
        VAndE::new(value, Box::new(Choice::first(l, r)))
    }

    pub fn from_second(value: (V, Option<O>), e: AST<O, V, S>) -> Self {
        VAndE::new(value, Box::new(Choice::second(e)))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ASTKind<O, V, S> {
    /// Terminal symbol
    LeafNode(LeafNode<O>),
    /// Viriable
    InternalNode(InternalNode<O, V, S>),
    // InternalNode { variable: V, choice: Box<Choice<AST<O, V, S, L>>> },
}

pub type AST<O, V, S> = Spanned<ASTKind<O, V, S>, S>;

pub type CST<O, V, S> = Spanned<VAndE<V, Choice<AST<O, V, S>>>, S>;

impl<O, V, S> AST<O, V, S> {
    // Leaf Node
    pub fn from_leaf_node(leaf_node: LeafNode<O>, span: S) -> Self {
        Self::new(ASTKind::LeafNode(leaf_node), span)
    }

    // Internal Node
    pub fn from_internal_node(internal_node: InternalNode<O, V, S>, span: S) -> Self {
        Self::new(ASTKind::InternalNode(internal_node), span)
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
}
