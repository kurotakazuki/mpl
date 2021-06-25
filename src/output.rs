use crate::tree::{AST, CST};

/// Output types.
pub trait Output<'input, I: ?Sized, V, S>: Sized {
    fn output_ast(input: &'input I, cst: CST<V, S, Self>) -> AST<V, S, Self>;
}

impl<'input, I, V, S> Output<'input, I, V, S> for () {
    fn output_ast(_input: &'input I, cst: CST<V, S, Self>) -> AST<V, S, Self> {
        AST::from_cst(cst)
    }
}
