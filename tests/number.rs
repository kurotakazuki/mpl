use mpl::input::Input;
use mpl::output::Output;
use mpl::parse::Parse;
use mpl::position::Position;
use mpl::rules::{RightRule, RightRuleKind, Rules};
use mpl::span::{Len, Span, Start, StartAndLenSpan};
use mpl::symbols::{Metasymbol, Terminal, Variable};
use mpl::tree::{InternalNode, LeafNode, AST, CST};
use std::collections::HashMap;
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
        &self,
        input: &'a ExtStr,
        pos: BytePos,
        max_pos: &BytePos,
    ) -> Result<
        AST<NumberTerminal<'a>, NumberVariable, ByteSpan>,
        AST<NumberTerminal<'a>, NumberVariable, ByteSpan>,
    > {
        let eval_from = |len: usize, value: &str, number_terminal: NumberTerminal<'a>| {
            let start = pos;
            let pos: usize = pos.0 as usize;

            let span = ByteSpan::from_start_len(start, len as u16);
            let hi = span.hi(input);

            let ast = AST::from_leaf_node(LeafNode::from_original(number_terminal), span);

            if &hi <= max_pos {
                if let Some(s) = input.0.get(pos..pos + len) {
                    if s == value {
                        return Ok(ast);
                    }
                }
            }
            Err(ast)
        };

        match self {
            NumberTerminal::Str(digit) => eval_from(digit.len(), digit, NumberTerminal::Str(digit)),
            NumberTerminal::Char(digit) => eval_from(
                digit.len_utf8(),
                &digit.to_string(),
                NumberTerminal::Char(*digit),
            ),
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
                    AST::from_leaf_node(Metasymbol::Omit.into(), span.clone());

                let internal_node =
                    InternalNode::from_second((cst.node.value, Some(NumberTerminal::Str(s))), omit);

                AST::from_internal_node(internal_node, span)
            }
            _ => AST::from_cst(cst),
        }
    }
}

impl Variable for NumberVariable {}

impl<'a, R> Parse<'a, NumberTerminal<'a>, NumberVariable, ByteSpan, BytePos, R, NumberTerminal<'a>>
    for ExtStr
where
    R: Rules<NumberTerminal<'a>, NumberVariable>,
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
#[test]
fn number() {
    let mut rules = HashMap::new();

    rules.insert(
        NumberVariable::Number,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(NumberVariable::Digit),
                RightRuleKind::V(NumberVariable::Numeral),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        NumberVariable::Numeral,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(NumberVariable::Digit),
                RightRuleKind::V(NumberVariable::Numeral),
            ),
            RightRuleKind::Empty,
        ),
    );
    rules.insert(
        NumberVariable::Digit,
        RightRule::from_right_rule_kind(
            (RightRuleKind::V(NumberVariable::Zero), RightRuleKind::Empty),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        NumberVariable::Zero,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(NumberTerminal::Str("0")),
                RightRuleKind::Empty,
            ),
            RightRuleKind::V(NumberVariable::FZero),
        ),
    );
    rules.insert(
        NumberVariable::FZero,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(NumberTerminal::Char('０')),
                RightRuleKind::Empty,
            ),
            RightRuleKind::V(NumberVariable::One),
        ),
    );
    rules.insert(
        NumberVariable::One,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(NumberTerminal::Char('1')),
                RightRuleKind::Empty,
            ),
            RightRuleKind::V(NumberVariable::FOne),
        ),
    );
    rules.insert(
        NumberVariable::FOne,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(NumberTerminal::Str("１")),
                RightRuleKind::Empty,
            ),
            RightRuleKind::Failure,
        ),
    );

    let input = ExtStr(String::from("012001"));
    // all of the span
    let all_of_the_span = StartAndLenSpan::from_start_len(BytePos(0), input.0.len() as u16);
    let result = input.minimal_parse(&rules, &NumberVariable::Number, &all_of_the_span);

    assert!(result.is_err());

    let input = ExtStr(String::from("0１0０1"));
    // all of the span
    let all_of_the_span = StartAndLenSpan::from_start_len(BytePos(0), input.0.len() as u16);
    let result = input.minimal_parse(&rules, &NumberVariable::Number, &all_of_the_span);

    assert_eq!(result.unwrap().span.len, 9);
}
