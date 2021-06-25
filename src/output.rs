use crate::tree::{AST, CST};

/// Output types.
pub trait Output<'input, I: ?Sized, V, S>: Sized {
    fn output_ast(input: &'input I, cst: CST<Self, V, S>) -> AST<Self, V, S>;
}

impl<'input, I, V, S> Output<'input, I, V, S> for () {
    fn output_ast(
        _input: &'input I,
        cst: CST<Self, V, S>,
    ) -> AST<Self, V, S> {
        AST::from_cst(cst)
    }
}
