//! Rules

use crate::choices::{First, Second};
use crate::symbols::{Equivalence, Metasymbol, TerminalSymbol, E};
use std::collections::HashMap;
use std::hash::Hash;

/// This structure is used when defining the right rule for a variable.
#[derive(Clone, Debug, PartialEq)]
pub struct RightRule<GenE> {
    pub first: First<GenE>,
    pub second: Second<GenE>,
}

impl<GenE> RightRule<GenE> {
    pub const fn new(first: First<GenE>, second: Second<GenE>) -> Self {
        Self { first, second }
    }
}

impl<T, V> RightRule<E<T, V>> {
    pub fn from_right_rule_kind(
        first: (RightRuleKind<T, V>, RightRuleKind<T, V>),
        second: RightRuleKind<T, V>,
    ) -> Self {
        Self {
            first: First::new(first.0.into(), first.1.into()),
            second: Second::new(second.into()),
        }
    }
}

/// This is used when creating a `RightRule`.
pub enum RightRuleKind<T, V> {
    Empty,
    Failure,
    Any(usize),
    All,
    T(T),
    V(V),
}

impl<T, V> From<RightRuleKind<T, V>> for E<T, V> {
    fn from(right_rule_kind: RightRuleKind<T, V>) -> Self {
        match right_rule_kind {
            RightRuleKind::Empty => Metasymbol::Empty.into(),
            RightRuleKind::Failure => Metasymbol::Failure.into(),
            RightRuleKind::Any(n) => Metasymbol::Any(n).into(),
            RightRuleKind::All => Metasymbol::All.into(),
            RightRuleKind::T(t) => E::T(TerminalSymbol::Original(t)),
            RightRuleKind::V(v) => E::V(v),
        }
    }
}

pub type Rule<V, GenE> = Equivalence<V, RightRule<GenE>>;

/// Rules types.
///
/// `R` is a finite set of rules of the form.
pub trait Rules<T, V> {
    fn get(&self, variable: &V) -> Option<&RightRule<E<T, V>>>;
}

impl<T, V> Rules<T, V> for HashMap<V, RightRule<E<T, V>>>
where
    V: Eq + Hash,
{
    fn get(&self, variable: &V) -> Option<&RightRule<E<T, V>>> {
        self.get(variable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbols::{Metasymbol, TerminalSymbol};

    /// ```
    /// BinDigit = '0' () / One
    /// One = '1' () / f
    /// ```
    #[test]
    fn rules() {
        #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
        enum BinDigitTerminal {
            Char(char),
        }

        #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
        enum BinDigitVariable {
            BinDigit,
            One,
        }

        let mut rules: HashMap<BinDigitVariable, RightRule<E<BinDigitTerminal, BinDigitVariable>>> =
            HashMap::new();

        rules.insert(
            BinDigitVariable::BinDigit,
            RightRule::new(
                First::new(
                    TerminalSymbol::from_original(BinDigitTerminal::Char('0')).into(),
                    Metasymbol::Empty.into(),
                ),
                Second::new(E::from_v(BinDigitVariable::One)),
            ),
        );
        rules.insert(
            BinDigitVariable::One,
            RightRule::new(
                First::new(
                    TerminalSymbol::from_original(BinDigitTerminal::Char('1')).into(),
                    Metasymbol::Empty.into(),
                ),
                Second::new(Metasymbol::Failure.into()),
            ),
        );

        let mut rules2: HashMap<
            BinDigitVariable,
            RightRule<E<BinDigitTerminal, BinDigitVariable>>,
        > = HashMap::new();

        rules2.insert(
            BinDigitVariable::BinDigit,
            RightRule::from_right_rule_kind(
                (
                    RightRuleKind::T(BinDigitTerminal::Char('0')),
                    RightRuleKind::Empty,
                ),
                RightRuleKind::V(BinDigitVariable::One),
            ),
        );
        rules2.insert(
            BinDigitVariable::One,
            RightRule::from_right_rule_kind(
                (
                    RightRuleKind::T(BinDigitTerminal::Char('1')),
                    RightRuleKind::Empty,
                ),
                RightRuleKind::Failure,
            ),
        );

        assert_eq!(
            rules[&BinDigitVariable::BinDigit],
            rules2[&BinDigitVariable::BinDigit]
        );
        assert_eq!(
            rules[&BinDigitVariable::One],
            rules2[&BinDigitVariable::One]
        );
    }
}
