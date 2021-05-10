use crate::input::Input;
use crate::output::Output;
use crate::parse::Parse;
use crate::position::BytePos;
use crate::span::ByteSpan;
use crate::symbols::{StrTerminal, Variable};

impl Input<ByteSpan> for str {
    fn all_of_the_span(&self) -> ByteSpan {
        ByteSpan::from_start_len(BytePos(0), self.len() as u16)
    }
}

impl<'input, OutputT, V> Parse<'input, StrTerminal<'input>, OutputT, V, ByteSpan, BytePos> for str
where
    OutputT: Output<'input, str, V, ByteSpan>,
    V: Variable,
{
}
