use crate::span::StartAndLenSpan;
use crate::trees::AST;

pub mod metasymbol;
pub mod slice_terminal;
pub mod str_terminal;
pub mod terminal_symbol;
pub mod u8slice_terminal;

/// Original terminal symbol types.
pub trait Terminal<'i, I, V, S, P, O>
where
    I: ?Sized,
{
    fn eval(&self, input: &'i I, pos: P, max_pos: &P) -> Result<AST<V, S, O>, AST<V, S, O>>;
}

type StartAndLenResult<V, P, L, O> =
    Result<AST<V, StartAndLenSpan<P, L>, O>, AST<V, StartAndLenSpan<P, L>, O>>;
