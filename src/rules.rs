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
            RightRuleKind::Epsilon => E::T(TerminalSymbol::M(Metasymbol::Epsilon)),
            RightRuleKind::Failure => E::T(TerminalSymbol::M(Metasymbol::Failure)),
            RightRuleKind::Any(n) => E::T(TerminalSymbol::M(Metasymbol::Any(n))),
            RightRuleKind::All => E::T(TerminalSymbol::M(Metasymbol::All)),
            RightRuleKind::T(t) => E::T(TerminalSymbol::Original(t)),
            RightRuleKind::V(v) => E::V(v),
        }
    }
}

pub type Rule<T, V> = Equivalence<V, RightRule<T, V>>;

pub struct Rules<T, V: Eq + Hash>(pub HashMap<V, RightRule<T, V>>);

impl<T, V> Rules<T, V>
where
    V: Eq + Hash,
{
    pub fn new() -> Self {
        Rules(HashMap::new())
    }

    pub fn insert_rule(&mut self, rule: Rule<T, V>) -> Option<RightRule<T, V>> {
        self.0.insert(rule.value, rule.equal)
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

        let mut rules: Rules<BinDigitTerminal, BinDigitVariable> = Rules::new();

        rules.0.insert(
            BinDigitVariable::BinDigit,
            RightRule::new(
                choice::First::new(
                    E::t(TerminalSymbol::from_t(BinDigitTerminal::Char('0'))),
                    E::t(TerminalSymbol::from_m(Metasymbol::Epsilon)),
                ),
                choice::Second::new(E::v(BinDigitVariable::One)),
            ),
        );
        rules.0.insert(
            BinDigitVariable::One,
            RightRule::new(
                choice::First::new(
                    E::t(TerminalSymbol::from_t(BinDigitTerminal::Char('1'))),
                    E::t(TerminalSymbol::from_m(Metasymbol::Epsilon)),
                ),
                choice::Second::new(E::t(TerminalSymbol::from_m(Metasymbol::Failure))),
            ),
        );

        let mut rules2: Rules<BinDigitTerminal, BinDigitVariable> = Rules::new();

        rules2.0.insert(
            BinDigitVariable::BinDigit,
            RightRule::from_right_rule_kind(
                (
                    RightRuleKind::T(BinDigitTerminal::Char('0')),
                    RightRuleKind::Epsilon,
                ),
                RightRuleKind::V(BinDigitVariable::One),
            ),
        );
        rules2.0.insert(
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
            rules.0[&BinDigitVariable::BinDigit],
            rules2.0[&BinDigitVariable::BinDigit]
        );
        assert_eq!(
            rules.0[&BinDigitVariable::One],
            rules2.0[&BinDigitVariable::One]
        );
    }
}
