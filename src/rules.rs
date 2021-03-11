use crate::symbols::E;

// #[derive(Clone, Debug, Hash, Eq, PartialEq)]
// pub struct RRuleWithValue<T, V1, V2, V3> {
//     value: T,
//     rrule: RRule<V1, V2, V3>
// }

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Choice<T, V> {
    First(E<T, V>, E<T, V>),
    Second(E<T, V>),
}


#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct RightRule<T, V> {
    first: (E<T, V>, E<T, V>),
    second: E<T, V>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number() {
        #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
        enum DigitTerminal {
            Zero,
            One,
            Two,
            Three,
            Four,
            Five,
            Six,
            Seven,
            Eight,
            Nine,
        }
    }
}