use crate::input::Input;
use crate::output::Output;
use crate::parse::Parse;
use crate::position::BytePos;
use crate::span::ByteSpan;
use crate::symbols::{SliceTerminal, Variable};

impl<'input, T> Input<'input, ByteSpan> for [T] where T: 'input {
    fn all_of_the_span(&'input self) -> ByteSpan {
        ByteSpan::from_start_len(BytePos(0), self.len() as u16)
    }
}

/// T represents the element type.
impl<'input, T, OutputT, V> Parse<'input, SliceTerminal<'input, T>, OutputT, V, ByteSpan, BytePos> for [T]
where
    T: PartialEq,
    OutputT: Output<'input, Self, V, ByteSpan>,
    V: Variable,
{
}
