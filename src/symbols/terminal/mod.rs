use crate::tree::AST;

pub mod metasymbol;
pub mod slice_terminal;
pub mod str_terminal;
pub mod terminal_symbol;
pub mod u8slice_terminal;

pub trait Terminal<'input, I, O, V, S, P>
where
    I: ?Sized,
{
    fn eval(
        &'input self,
        input: &'input I,
        pos: P,
        max_pos: &P,
    ) -> Result<AST<O, V, S>, AST<O, V, S>>;
}
