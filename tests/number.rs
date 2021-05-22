use mpl::input::Input;
use mpl::output::Output;
use mpl::parse::Parse;
use mpl::position::Position;
use mpl::rules::{RightRule, RightRuleKind, Rule, Rules};
use mpl::span::{Len, Start, StartAndLenSpan};
use mpl::symbols::{Metasymbol, Terminal, Variable};
use mpl::tree::{InternalNode, LeafNode, AST, CST};
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum NumberTerminal<'a> {
    Str(&'a str),
    Char(char),
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum NumberVariable {
    Number,
    Numeral,
    Digit,
    Zero,
    FZero,
    One,
    FOne,
}

struct ExtStr(pub String);

impl Input for ExtStr {}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BytePos(pub u32);

impl From<BytePos> for u32 {
    fn from(byte_pos: BytePos) -> Self {
        byte_pos.0
    }
}

impl From<u32> for BytePos {
    fn from(value: u32) -> Self {
        BytePos(value)
    }
}

impl Add for BytePos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Sub for BytePos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl Position for BytePos {}

pub type ByteSpan = StartAndLenSpan<BytePos, u16>;

impl Start<ExtStr, u16> for BytePos {
    fn into_usize(start: Self, _: &ExtStr) -> usize {
        start.0 as usize
    }
    fn start(_: &ExtStr) -> Self {
        0.into()
    }
    fn hi_from_start_and_len(start: Self, len: u16, _: &ExtStr) -> Self {
        start + BytePos(len as u32)
    }
}

impl Len<ExtStr, BytePos> for u16 {
    fn from_usize(_: BytePos, len: usize, _: &ExtStr) -> Self {
        len as Self
    }
    fn len_from_lo_and_hi(lo: BytePos, hi: BytePos, _: &ExtStr) -> Self {
        u32::from(hi - lo) as Self
    }
}

impl<'a> Terminal<'a, ExtStr, NumberTerminal<'a>, NumberVariable, ByteSpan, BytePos>
    for NumberTerminal<'a>
{
    fn eval(
        &'a self,
        input: &'a ExtStr,
        pos: BytePos,
        max_pos: &BytePos,
    ) -> Result<AST<NumberTerminal<'a>, NumberVariable, ByteSpan>, ()> {
        match self {
            NumberTerminal::Str(digit) => {
                let start = pos;
                let pos: usize = pos.0 as usize;
                let len = digit.len();
                if pos + len <= max_pos.0 as usize
                    && &input.0.as_bytes()[pos..pos + len] == digit.as_bytes()
                {
                    Ok(
                        AST::<NumberTerminal, NumberVariable, ByteSpan>::from_leaf_node(
                            LeafNode::from_t(NumberTerminal::Str(digit)),
                            ByteSpan::from_start_len(start, len as u16),
                        ),
                    )
                } else {
                    Err(())
                }
            }
            NumberTerminal::Char(digit) => {
                let start = pos;
                let pos: usize = pos.0 as usize;
                let len = digit.len_utf8();

                if pos + len <= max_pos.0 as usize
                    && &input.0.as_bytes()[pos..pos + len] == digit.to_string()[..].as_bytes()
                {
                    Ok(
                        AST::<NumberTerminal, NumberVariable, ByteSpan>::from_leaf_node(
                            LeafNode::from_t(NumberTerminal::Char(*digit)),
                            ByteSpan::from_start_len(start, len as u16),
                        ),
                    )
                } else {
                    Err(())
                }
            }
        }
    }
}

impl<'input> Output<'input, ExtStr, NumberVariable, ByteSpan> for NumberTerminal<'input> {
    fn output_ast(
        input: &'input ExtStr,
        cst: CST<Self, NumberVariable, ByteSpan>,
    ) -> AST<Self, NumberVariable, ByteSpan> {
        match cst.node.value {
            NumberVariable::Number => {
                let lo = cst.span.start.0 as usize;
                let hi = lo + cst.span.len as usize;
                let s = &input.0[lo..hi];

                AST::from_cst_and_output(cst, Some(NumberTerminal::Str(s)))
            }
            NumberVariable::Digit => {
                let span = cst.span;

                let lo = span.start.0 as usize;
                let hi = lo + span.len as usize;
                let s = &input.0[lo..hi];

                let omit: AST<Self, NumberVariable, ByteSpan> =
                    AST::from_leaf_node(LeafNode::from_m(Metasymbol::Omit), span.clone());

                let internal_node =
                    InternalNode::from_second((cst.node.value, Some(NumberTerminal::Str(s))), omit);

                AST::from_internal_node(internal_node, span)

                // AST::from_vandchoice_and_output(v_and_choice, Some(NumberTerminal::Str(s)))
            }
            _ => AST::from_cst(cst),
        }
    }
}

impl Variable for NumberVariable {}

impl<'a> Parse<'a, NumberTerminal<'a>, NumberTerminal<'a>, NumberVariable, ByteSpan, BytePos>
    for ExtStr
{
}

/// The following syntax is a lexical syntax for numbers.
/// ```
/// Number = Digit Numeral / f
/// Numeral = Digit Numeral / ()
/// Digit = Zero () / f
/// Zero = "0" () / FZero
/// FZero = '０' () / One
/// One = '1' () / FOne
/// FOne = "１" () / f
/// ```
fn main() {
    let number_rule: Rule<NumberTerminal, NumberVariable> = Rule::new(
        NumberVariable::Number,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(NumberVariable::Digit),
                RightRuleKind::V(NumberVariable::Numeral),
            ),
            RightRuleKind::Failure,
        ),
    );
    let numeral_rule: Rule<NumberTerminal, NumberVariable> = Rule::new(
        NumberVariable::Numeral,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(NumberVariable::Digit),
                RightRuleKind::V(NumberVariable::Numeral),
            ),
            RightRuleKind::Epsilon,
        ),
    );
    let digit_rule: Rule<NumberTerminal, NumberVariable> = Rule::new(
        NumberVariable::Digit,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(NumberVariable::Zero),
                RightRuleKind::Epsilon,
            ),
            RightRuleKind::Failure,
        ),
    );

    let zero_rule: Rule<NumberTerminal, NumberVariable> = Rule::new(
        NumberVariable::Zero,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(NumberTerminal::Str("0")),
                RightRuleKind::Epsilon,
            ),
            RightRuleKind::V(NumberVariable::FZero),
        ),
    );
    let f_zero_rule: Rule<NumberTerminal, NumberVariable> = Rule::new(
        NumberVariable::FZero,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(NumberTerminal::Char('０')),
                RightRuleKind::Epsilon,
            ),
            RightRuleKind::V(NumberVariable::One),
        ),
    );
    let one_rule: Rule<NumberTerminal, NumberVariable> = Rule::new(
        NumberVariable::One,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(NumberTerminal::Char('1')),
                RightRuleKind::Epsilon,
            ),
            RightRuleKind::V(NumberVariable::FOne),
        ),
    );
    let f_one_rule: Rule<NumberTerminal, NumberVariable> = Rule::new(
        NumberVariable::FOne,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(NumberTerminal::Str("１")),
                RightRuleKind::Epsilon,
            ),
            RightRuleKind::Failure,
        ),
    );

    let mut rules = Rules::new();

    rules.insert_rule(number_rule);
    rules.insert_rule(numeral_rule);
    rules.insert_rule(digit_rule);
    rules.insert_rule(zero_rule);
    rules.insert_rule(f_zero_rule);
    rules.insert_rule(one_rule);
    rules.insert_rule(f_one_rule);

    let input = ExtStr(String::from("012001"));
    // all of the span
    let all_of_the_span = StartAndLenSpan::from_start_len(BytePos(0), input.0.len() as u16);
    let result = input.minimal_parse(&rules, &NumberVariable::Number, all_of_the_span);

    assert_eq!(result, Err(()));

    let input = ExtStr(String::from("0１0０1"));
    // all of the span
    let all_of_the_span = StartAndLenSpan::from_start_len(BytePos(0), input.0.len() as u16);
    let result = input.minimal_parse(&rules, &NumberVariable::Number, all_of_the_span);

    assert_eq!(result.unwrap().span.len, 9);
}
