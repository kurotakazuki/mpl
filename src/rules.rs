use crate::choice;
use crate::symbols::E;
use std::collections::HashMap;

/// This structure is used when defining the rule for a variable.
#[derive(Clone, Debug, PartialEq)]
pub struct RightRule<T, V> {
    pub first: choice::First<E<T, V>>,
    pub second: choice::Second<E<T, V>>,
}

pub type Rules<T, V> = HashMap<V, RightRule<T, V>>;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn number() {
//         #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
//         enum DigitTerminal {
//             Zero,
//             One,
//             Two,
//             Three,
//             Four,
//             Five,
//             Six,
//             Seven,
//             Eight,
//             Nine,
//         }
//     }
// }
