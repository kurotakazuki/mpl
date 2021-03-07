#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Variable {
    name: String,

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
    Meta(Metasymbol),
}

// #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
// pub enum E<T, V> {
//     T(TerminalSymbol<T>),
//     V(V),
// }

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct RRuleWithValue<T, V1, V2, V3> {
    value: T,
    rrule: RRule<V1, V2, V3>
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct RRule<V1, V2, V3> {
    variable1: V1,
    variable2: V2,
    variable3: V3,
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