//! Output

use crate::trees::{AST, CST};

/// Output types.
pub trait Output<'i, I: ?Sized, V, S>: Sized {
    fn output_ast(input: &'i I, cst: CST<V, S, Self>) -> AST<V, S, Self>;
}

impl<'i, I: ?Sized, V, S> Output<'i, I, V, S> for () {
    fn output_ast(_input: &'i I, cst: CST<V, S, Self>) -> AST<V, S, Self> {
        AST::from_cst(cst)
    }
}
