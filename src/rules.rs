use crate::choice;
use crate::symbols::{E, Metasymbol, TerminalSymbol};
use std::collections::HashMap;

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

    pub fn from_right_rule_kind(first: (RightRuleKind<T, V>, RightRuleKind<T, V>), second: RightRuleKind<T, V>) -> Self {
        Self {
            first: choice::First::new(first.0.into(), first.1.into()),
            second: choice::Second::new(second.into())
        }
    }
}

pub type Rules<T, V> = HashMap<V, RightRule<T, V>>;

/// This is used when creating a RightRule.
pub enum RightRuleKind<T, V> {
    Epsilon,
    Failed,
    Any,
    All,
    T(T),
    // M(Metasymbol),
    V(V),
}

impl<T, V> From<RightRuleKind<T, V>> for E<T, V> {
    fn from(right_rule_kind: RightRuleKind<T, V>) -> Self {
        match right_rule_kind {
            RightRuleKind::Epsilon => E::T(TerminalSymbol::M(Metasymbol::Epsilon)),
            RightRuleKind::Failed => E::T(TerminalSymbol::M(Metasymbol::Failed)),
            RightRuleKind::Any => E::T(TerminalSymbol::M(Metasymbol::Any)),
            RightRuleKind::All => E::T(TerminalSymbol::M(Metasymbol::All)),
            RightRuleKind::T(t) => E::T(TerminalSymbol::Original(t)),
            RightRuleKind::V(v) => E::V(v),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::symbols::{Metasymbol, TerminalSymbol};
    use super::*;

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

        let mut rules: Rules<BinDigitTerminal, BinDigitVariable> = HashMap::new();

        rules.insert(
            BinDigitVariable::BinDigit,
            RightRule::new(
                choice::First::new(
                    E::t(TerminalSymbol::from_t(BinDigitTerminal::Char('0'))),
                    E::t(TerminalSymbol::from_m(Metasymbol::Epsilon))
                ),
                choice::Second::new(
                    E::v(BinDigitVariable::One)
                )
            )
        );
        rules.insert(
            BinDigitVariable::One,
            RightRule::new(
                choice::First::new(
                    E::t(TerminalSymbol::from_t(BinDigitTerminal::Char('1'))),
                    E::t(TerminalSymbol::from_m(Metasymbol::Epsilon))
                ),
                choice::Second::new(
                    E::t(TerminalSymbol::from_m(Metasymbol::Failed))
                )
            )
        );

        let mut rules2: Rules<BinDigitTerminal, BinDigitVariable> = HashMap::new();

        rules2.insert(BinDigitVariable::BinDigit,
            RightRule::from_right_rule_kind(
                (RightRuleKind::T(BinDigitTerminal::Char('0')), RightRuleKind::Epsilon),
                RightRuleKind::V(BinDigitVariable::One)
            )
        );
        rules2.insert(BinDigitVariable::One,
            RightRule::from_right_rule_kind(
                (RightRuleKind::T(BinDigitTerminal::Char('1')), RightRuleKind::Epsilon),
                RightRuleKind::Failed
            )
        );

        assert_eq!(rules[&BinDigitVariable::BinDigit], rules2[&BinDigitVariable::BinDigit]);
        assert_eq!(rules[&BinDigitVariable::One], rules2[&BinDigitVariable::One]);
    }
}
