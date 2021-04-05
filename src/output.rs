use crate::choice::Choice;
use crate::cst::CST;

pub trait Output<'input, I: ?Sized, V, S>: Sized {
    fn new(input: &'input I, variable: &V, span: &S, cst_choice: Choice<&CST<Self, V, S>>) -> Option<Self>;
}
