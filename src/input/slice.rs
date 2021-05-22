use crate::input::Input;
use crate::output::Output;
use crate::parse::Parse;
use crate::span::{Len, Start, StartAndLenSpan};
use crate::symbols::{SliceTerminal, Variable};

impl<T> Input for [T] {}

/// T represents the element type.
impl<'input, T, OutputT, V, P, L>
    Parse<'input, SliceTerminal<'input, T>, OutputT, V, StartAndLenSpan<P, L>, P> for [T]
where
    T: PartialEq,
    OutputT: Output<'input, Self, V, StartAndLenSpan<P, L>>,
    V: Variable,
    P: Start<Self, L>,
    L: Len<Self, P>,
{
}
