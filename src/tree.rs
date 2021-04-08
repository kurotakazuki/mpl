use crate::choice::Choice;
use crate::span::Spanned;
use crate::symbols::{TerminalSymbol, VAndE};

pub type LeafNode<OutputT> = TerminalSymbol<OutputT>;
pub type InternalNode<OutputT, V, S> = VAndE<(V, Option<OutputT>), Box<Choice<AST<OutputT, V, S>>>>;

// impl LeafNode {
//     pub fn from_t() -> Self {
//         TerminalSymbol::from_t(())
//     }

//     pub fn from_m(metasymbol: Metasymbol) -> Self {
//         TerminalSymbol::from_m(metasymbol)
//     }
// }

impl<OutputT, V, S> InternalNode<OutputT, V, S> {
    pub fn from_first(
        value: (V, Option<OutputT>),
        l: AST<OutputT, V, S>,
        r: AST<OutputT, V, S>,
    ) -> Self {
        VAndE::new(value, Box::new(Choice::first(l, r)))
    }

    pub fn from_second(value: (V, Option<OutputT>), e: AST<OutputT, V, S>) -> Self {
        VAndE::new(value, Box::new(Choice::second(e)))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ASTKind<OutputT, V, S> {
    /// Terminal symbol
    LeafNode(LeafNode<OutputT>),
    /// Viriable
    InternalNode(InternalNode<OutputT, V, S>),
    // InternalNode { variable: V, choice: Box<Choice<AST<OutputT, V, S, L>>> },
}

pub type AST<OutputT, V, S> = Spanned<ASTKind<OutputT, V, S>, S>;

pub type CST<OutputT, V, S> = Spanned<VAndE<V, Choice<AST<OutputT, V, S>>>, S>;

impl<OutputT, V, S> AST<OutputT, V, S> {
    // Leaf Node
    pub fn from_leaf_node(leaf_node: LeafNode<OutputT>, span: S) -> Self {
        Self::new(ASTKind::LeafNode(leaf_node), span)
    }

    // Internal Node
    pub fn from_internal_node(internal_node: InternalNode<OutputT, V, S>, span: S) -> Self {
        Self::new(ASTKind::InternalNode(internal_node), span)
    }

    pub fn from_cst_and_output(
        cst: CST<OutputT, V, S>,
        output: Option<OutputT>,
    ) -> Self
    {
        match cst.node.equal {
            Choice::First(first) => {
                Self::from_internal_node(
                    InternalNode::from_first((cst.node.value, output), first.lhs, first.rhs),
                    cst.span,
                )
            }
            Choice::Second(second) => {
                Self::from_internal_node(
                    InternalNode::from_second((cst.node.value, output), second.0),
                    cst.span,
                )
            }
        }
    }

    pub fn from_cst(
        cst: CST<OutputT, V, S>,
    ) -> Self
    {
        Self::from_cst_and_output(cst, None)
    }
}
