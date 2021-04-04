use mpg::cst::{LeafNode, CST};
use mpg::input::Input;
use mpg::output::Output;
use mpg::parse::Parse;
use mpg::position::BytePos;
use mpg::span::{ByteSpan, Span};
use mpg::symbols::{Terminal, Variable};

use mpg::rules::{RightRule, RightRuleKind, Rule, Rules};

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
    One,
}

struct ExtStr(pub String);

impl Input<'_, ByteSpan> for ExtStr {
    fn all_of_the_span(&self) -> ByteSpan {
        let len = self.0.len();
        ByteSpan::from_start_len(BytePos(0), len as u16)
    }
}

impl<'a> Terminal<'a, ExtStr, NumberTerminal<'a>, NumberVariable, ByteSpan, BytePos>
    for NumberTerminal<'a>
{
    fn eval(
        &'a self,
        input: &'a ExtStr,
        pos: BytePos,
        all_of_the_span: &ByteSpan,
    ) -> Result<CST<NumberTerminal<'a>, NumberVariable, ByteSpan>, ()> {
        match self {
            NumberTerminal::Str(digit) => {
                let start = pos;
                let pos: usize = pos.0 as usize;
                if pos + 1 <= all_of_the_span.len as usize + all_of_the_span.start.0 as usize
                    && &input.0.as_bytes()[pos..pos + 1] == digit.as_bytes()
                {
                    Ok(
                        CST::<NumberTerminal, NumberVariable, ByteSpan>::from_leaf_node(
                            LeafNode::from_t(NumberTerminal::Str(digit)),
                            ByteSpan::from_start_len(start, 1),
                        ),
                    )
                } else {
                    Err(())
                }
            }
            NumberTerminal::Char(digit) => {
                let start = pos;
                let pos: usize = pos.0 as usize;
                if pos + 1 <= all_of_the_span.len as usize + all_of_the_span.start.0 as usize
                    && &input.0.as_bytes()[pos..pos + 1] == digit.to_string()[..].as_bytes()
                {
                    Ok(
                        CST::<NumberTerminal, NumberVariable, ByteSpan>::from_leaf_node(
                            LeafNode::from_t(NumberTerminal::Char(*digit)),
                            ByteSpan::from_start_len(start, 1),
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
    fn new(input: &'input ExtStr, variable: NumberVariable, span: ByteSpan) -> Option<Self> {
        match variable {
            NumberVariable::Number => {
                let lo = span.start.0 as usize;
                let hi = lo + span.len as usize;
                let s = &input.0[lo..hi];

                Some(NumberTerminal::Str(s))
            }
            NumberVariable::Digit => Some(NumberTerminal::Str("ijjijijij")),
            _ => None,
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
/// Zero = "0" () / One
/// One = "1" () / f
/// ```
#[test]
fn number() {
    let number_rule: Rule<NumberTerminal, NumberVariable> = Rule::new(
        NumberVariable::Number,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(NumberVariable::Digit),
                RightRuleKind::V(NumberVariable::Numeral),
            ),
            RightRuleKind::Failed,
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
            RightRuleKind::Failed,
        ),
    );

    let zero_rule: Rule<NumberTerminal, NumberVariable> = Rule::new(
        NumberVariable::Zero,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(NumberTerminal::Str("0")),
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
            RightRuleKind::Failed,
        ),
    );

    let mut rules = Rules::new();

    rules.insert_rule(number_rule);
    rules.insert_rule(numeral_rule);
    rules.insert_rule(digit_rule);
    rules.insert_rule(zero_rule);
    rules.insert_rule(one_rule);

    let input = ExtStr(String::from("012001"));
    let result = input.mpg_parse(&rules, &NumberVariable::Number, None);

    assert_eq!(result, Err(()));

    let input = ExtStr(String::from("001"));
    let result = input.mpg_parse(&rules, &NumberVariable::Number, None);

    assert_eq!(
        result.unwrap().span,
        ByteSpan::from_lo_hi(0.into(), 3.into())
    );
}
