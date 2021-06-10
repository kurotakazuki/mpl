use crate::input::Input;
use crate::output::Output;
use crate::parse::Parse;
use crate::span::{Len, Start, StartAndLenSpan};
use crate::symbols::{SliceTerminal, U8SliceTerminal, Variable};

impl<T> Input for [T] {}

/// T represents the element type.
impl<'input, T, O, V, P, L> Parse<'input, SliceTerminal<'input, T>, O, V, StartAndLenSpan<P, L>, P>
    for [T]
where
    T: PartialEq,
    O: Output<'input, Self, V, StartAndLenSpan<P, L>>,
    V: Variable,
    P: Start<Self, L>,
    L: Len<Self, P>,
{
}

impl<'input, O, V, P, L> Parse<'input, U8SliceTerminal<'input>, O, V, StartAndLenSpan<P, L>, P>
    for [u8]
where
    O: Output<'input, Self, V, StartAndLenSpan<P, L>>,
    V: Variable,
    P: Start<Self, L>,
    L: Len<Self, P>,
{
}
