use crate::symbols::{StrTerminal, Variable};
use crate::position::BytePos;
use crate::input::Input;
use crate::parse::Parse;
use crate::span::ByteSpan;

use std::convert::TryFrom;

impl<'input> Input<'input, ByteSpan> for str {
    fn all_of_the_span(&'input self) -> ByteSpan { 
        ByteSpan::from_start_len(BytePos(0), self.len() as u16)
    }
}

impl<'input, OutputT, V> Parse<'input, StrTerminal<'input>, OutputT, V, ByteSpan, BytePos> for str where OutputT: TryFrom<(&'input Self, V, ByteSpan)>, V: Variable {}
