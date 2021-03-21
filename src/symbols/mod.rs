pub use self::e::E;
pub use self::metasymbol::Metasymbol;
pub use self::terminal_symbol::TerminalSymbol;
pub use self::variable::Variable;

use std::convert::TryFrom;

mod e;
mod metasymbol;
mod terminal_symbol;
mod variable;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MPGGTerminalType<'a> {
    // Char(char),
    Str(&'a str),
    U8Slice(&'a [u8]),
    // F32(f32),
    // F64(f64),
    // Isize(isize),
    // I8(i8),
    // I16(i16),
    // I32(i32),
    // I64(i64),
    // I128(i128),
    // Usize(usize),
    // U8(u8),
    // U16(u16),
    // U32(u32),
    // U64(u64),
    // U128(u128),
}

impl<'a> From<&'a str> for MPGGTerminalType<'a> {
    fn from(value: &'a str) -> Self {
        Self::Str(value)
    }
}

impl<'a> From<&'a [u8]> for MPGGTerminalType<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self::U8Slice(value)
    }
}

/// TODO: Change Error type
impl<'a> TryFrom<MPGGTerminalType<'a>> for &'a str {
    type Error = ();

    fn try_from(m_p_g_g_terminal_type: MPGGTerminalType<'a>) -> Result<Self, Self::Error> {
        match m_p_g_g_terminal_type {
            MPGGTerminalType::Str(s) => Ok(s),
            MPGGTerminalType::U8Slice(s) => std::str::from_utf8(s).map_err(|_| ()),
        }
    }
}

impl<'a> TryFrom<MPGGTerminalType<'a>> for &'a [u8] {
    type Error = ();

    fn try_from(m_p_g_g_terminal_type: MPGGTerminalType<'a>) -> Result<Self, Self::Error> {
        match m_p_g_g_terminal_type {
            MPGGTerminalType::Str(s) => Ok(s.as_bytes()),
            MPGGTerminalType::U8Slice(s) => Ok(s),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // #[test]
//     // fn valiable() {
//     //     // use crate::span::VSpan;

//     //     // enum VariableKind<V1, V2, V3> {
//     //     //     Number(RRuleWithValue<String, V1, V2, V3>),
//     //     //     Numeral(RRule<V1, V2, V3>),
//     //     //     Digit(RRule<V1, V2, V3>),
//     //     //     Zero(RRule<V1, V2, V3>),
//     //     //     One(RRule<V1, V2, V3>),
//     //     //     Nine(RRule<&str, V2, V3>),

//     //     // }

//     // }
// }
