use std::convert::TryFrom;

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

#[derive(Clone, Debug, PartialEq)]
pub enum Metasymbol {
    Epsilon,
    Failed,
    Any,
}

impl Metasymbol {
    pub fn epsilon() -> Self {
        Self::Epsilon
    }

    pub fn failed() -> Self {
        Self::Failed
    }

    pub fn any() -> Self {
        Self::Any
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TerminalSymbol<T> {
    Original(T),
    M(Metasymbol),
}

// impl<T> From<Metasymbol> for TerminalSymbol<T> {
//     fn from(metasymbol: Metasymbol) -> Self {
//         Self::M(metasymbol)
//     }
// }

// impl<T> From<T> for TerminalSymbol<T> {
//     fn from(t: T) -> Self {
//         Self::Original(t)
//     }
// }

impl<T> TerminalSymbol<T> {
    pub fn original(t: T) -> Self {
        Self::Original(t)
    }

    pub fn m(metasymbol: Metasymbol) -> Self {
        Self::M(metasymbol)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variable<V, E> {
    value: V,
    equal: E,
}

impl<V, E> Variable<V, E> {
    pub fn new(value: V, equal: E) -> Self {
        Self { value, equal }
    }
}

// #[derive(Clone, Debug, PartialEq)]
// pub enum E<T, V> {
//     T(TerminalSymbol<T>),
//     V(V),
// }

// trait TerminalAndInput {
//     type Input;

//     fn from_terminal(&self) -> Self::Input;
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn valiable() {
    //     // use crate::span::VSpan;

    //     // enum VariableKind<V1, V2, V3> {
    //     //     Number(RRuleWithValue<String, V1, V2, V3>),
    //     //     Numeral(RRule<V1, V2, V3>),
    //     //     Digit(RRule<V1, V2, V3>),
    //     //     Zero(RRule<V1, V2, V3>),
    //     //     One(RRule<V1, V2, V3>),
    //     //     Nine(RRule<&str, V2, V3>),

    //     // }

    // }
}
