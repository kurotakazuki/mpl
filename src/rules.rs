use crate::choice;
use crate::symbols::{Equivalence, Metasymbol, TerminalSymbol, E};
use std::collections::HashMap;
use std::hash::Hash;

/// This structure is used when defining the rule for a variable.
#[derive(Clone, Debug, PartialEq)]
pub struct RightRule<T, V> {
    pub first: choice::First<E<T, V>>,
    pub second: choice::Second<E<T, V>>,
}

impl<T, V> RightRule<T, V> {
    pub fn new(first: choice::First<E<T, V>>, second: choice::Second<E<T, V>>) -> Self {
        Self { first, second }
    }

    pub fn from_right_rule_kind(
        first: (RightRuleKind<T, V>, RightRuleKind<T, V>),
        second: RightRuleKind<T, V>,
    ) -> Self {
        Self {
            first: choice::First::new(first.0.into(), first.1.into()),
            second: choice::Second::new(second.into()),
        }
    }
}

/// This is used when creating a RightRule.
pub enum RightRuleKind<T, V> {
    Epsilon,
    Failure,
    Any(usize),
    All,
    T(T),
    V(V),
}

impl<T, V> From<RightRuleKind<T, V>> for E<T, V> {
    fn from(right_rule_kind: RightRuleKind<T, V>) -> Self {
        match right_rule_kind {
            RightRuleKind::Epsilon => Metasymbol::Epsilon.into(),
            RightRuleKind::Failure => Metasymbol::Failure.into(),
            RightRuleKind::Any(n) => Metasymbol::Any(n).into(),
            RightRuleKind::All => Metasymbol::All.into(),
            RightRuleKind::T(t) => E::T(TerminalSymbol::Original(t)),
            RightRuleKind::V(v) => E::V(v),
        }
    }
}

pub type Rule<T, V> = Equivalence<V, RightRule<T, V>>;

pub trait Rules<T, V> {
    fn get(&self, variable: &V) -> Option<&RightRule<T, V>>;
}

impl<T, V> Rules<T, V> for HashMap<V, RightRule<T, V>>
where
    V: Eq + Hash,
{
    fn get(&self, variable: &V) -> Option<&RightRule<T, V>> {
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

        let mut rules: HashMap<BinDigitVariable, RightRule<BinDigitTerminal, BinDigitVariable>> =
            HashMap::new();

        rules.insert(
            BinDigitVariable::BinDigit,
            RightRule::new(
                choice::First::new(
                    TerminalSymbol::from_original(BinDigitTerminal::Char('0')).into(),
                    Metasymbol::Epsilon.into(),
                ),
                choice::Second::new(E::from_v(BinDigitVariable::One)),
            ),
        );
        rules.insert(
            BinDigitVariable::One,
            RightRule::new(
                choice::First::new(
                    TerminalSymbol::from_original(BinDigitTerminal::Char('1')).into(),
                    Metasymbol::Epsilon.into(),
                ),
                choice::Second::new(Metasymbol::Failure.into()),
            ),
        );

        let mut rules2: HashMap<BinDigitVariable, RightRule<BinDigitTerminal, BinDigitVariable>> =
            HashMap::new();

        rules2.insert(
            BinDigitVariable::BinDigit,
            RightRule::from_right_rule_kind(
                (
                    RightRuleKind::T(BinDigitTerminal::Char('0')),
                    RightRuleKind::Epsilon,
                ),
                RightRuleKind::V(BinDigitVariable::One),
            ),
        );
        rules2.insert(
            BinDigitVariable::One,
            RightRule::from_right_rule_kind(
                (
                    RightRuleKind::T(BinDigitTerminal::Char('1')),
                    RightRuleKind::Epsilon,
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
