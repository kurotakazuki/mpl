use crate::mplg::{MplgVariables, MplgVariables::*};

use mpl::choices::{First, Second};
use mpl::e_from;
use mpl::rules::{RightRule, Rules};
use mpl::symbols::{Metasymbol::*, TerminalSymbol, U8SliceTerminal, U8SliceTerminal::*, E};

pub struct MplgRules;

type Rule<'a> = RightRule<U8SliceTerminal<'a>, MplgVariables>;

macro_rules! mplg_rule {
    ($rule_ident:ident, $v:ty, $fl:tt, $fr:tt, $s:tt) => {
        /// $v = $fl $fr / $s
        const $rule_ident: Rule<'a> = RightRule {
            first: First {
                lhs: e_from!($fl),
                rhs: e_from!($fr),
            },
            second: Second(e_from!($s)),
        };
    };
}
