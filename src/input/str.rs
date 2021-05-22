use crate::input::Input;
use crate::output::Output;
use crate::parse::Parse;
use crate::span::{Len, Start, StartAndLenSpan};
use crate::symbols::{StrTerminal, Variable};

impl Input for str {}
impl Input for String {}

impl<'input, OutputT, V, P, L>
    Parse<'input, StrTerminal<'input>, OutputT, V, StartAndLenSpan<P, L>, P> for str
where
    OutputT: Output<'input, Self, V, StartAndLenSpan<P, L>>,
    V: Variable,
    P: Start<Self, L>,
    L: Len<Self, P>,
{
}
