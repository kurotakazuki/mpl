#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Variable {
    name: String,

}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenKind<'a> {
    Str(&'a str),
    Slice(&'a [u8]),
    F32(f32),
    F64(f64),
    Isize(isize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Usize(usize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),

}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Metasymbol {
    Epsilon,
    Failed,
    Any(usize),
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum TerminalSymbol<T> {
    Original(T),
    M(Metasymbol),
}

trait TerminalAndInput {
    type Input;
    
    fn from_terminal(&self) -> Self::Input;
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum E<T, V> {
    T(TerminalSymbol<T>),
    V(V),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valiable() {
        // use crate::span::VSpan;

        // enum VariableKind<V1, V2, V3> {
        //     Number(RRuleWithValue<String, V1, V2, V3>),
        //     Numeral(RRule<V1, V2, V3>),
        //     Digit(RRule<V1, V2, V3>),
        //     Zero(RRule<V1, V2, V3>),
        //     One(RRule<V1, V2, V3>),
        //     Nine(RRule<&str, V2, V3>),

        // }

    }
}