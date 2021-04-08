use crate::tree::{AST, CST};

pub trait Output<'input, I: ?Sized, V, S>: Sized {
    fn output_ast(input: &'input I, cst: CST<Self, V, S>) -> AST<Self, V, S>;
}
