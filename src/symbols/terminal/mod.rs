use crate::cst::CST;

pub mod metasymbol;
pub mod terminal_symbol;

pub trait Terminal<T, V, S, P, I: ?Sized> {
    fn eval(&self, input: &I, pos: P) -> Result<CST<T, V, S>, ()>;
}