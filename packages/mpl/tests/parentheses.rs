use mpl::output::Output;
use mpl::parse::Parse;
use mpl::rules::{RightRule, RightRuleKind};
use mpl::span::{Span, StartAndLenSpan};
use mpl::symbols::{StrTerminal, Variable};
use mpl::trees::{AST, CST};
use std::collections::HashMap;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum ParenthesesVariable {
    Open,
    Parentheses,
    Close,
}

impl Variable for ParenthesesVariable {}

impl<'input> Output<'input, str, ParenthesesVariable, StartAndLenSpan<u32, u16>> for String {
    fn output_ast(
        _input: &'input str,
        cst: CST<ParenthesesVariable, StartAndLenSpan<u32, u16>, Self>,
    ) -> AST<ParenthesesVariable, StartAndLenSpan<u32, u16>, Self> {
        match cst.node.value {
            ParenthesesVariable::Open => AST::from_cst_and_output(cst, Some(String::from("open"))),
            ParenthesesVariable::Parentheses => {
                AST::from_cst_and_output(cst, Some(String::from("paren")))
            }
            ParenthesesVariable::Close => {
                AST::from_cst_and_output(cst, Some(String::from("close")))
            }
        }
    }
}

/// ```
/// Open = '(' Parentheses / ()
/// Parentheses = Open Close / f
/// Close = ")" Open / f
/// ```
#[test]
fn parentheses() {
    let mut rules = HashMap::new();

    rules.insert(
        ParenthesesVariable::Open,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(StrTerminal::Char('(')),
                RightRuleKind::V(ParenthesesVariable::Parentheses),
            ),
            RightRuleKind::Empty,
        ),
    );
    rules.insert(
        ParenthesesVariable::Parentheses,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(ParenthesesVariable::Open),
                RightRuleKind::V(ParenthesesVariable::Close),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        ParenthesesVariable::Close,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(StrTerminal::Str(")")),
                RightRuleKind::V(ParenthesesVariable::Open),
            ),
            RightRuleKind::Failure,
        ),
    );

    let input = "()";

    // all of the span
    let all_of_the_span = StartAndLenSpan::<u32, u16>::from_start_len(0, input.len() as u16);

    let result: Result<
        AST<ParenthesesVariable, StartAndLenSpan<u32, u16>, String>,
        AST<ParenthesesVariable, StartAndLenSpan<u32, u16>, String>,
    > = input.minimal_parse(&rules, &ParenthesesVariable::Open, &all_of_the_span);
    assert_eq!(
        result.unwrap().span,
        StartAndLenSpan::from_lo_hi(0, 2, input)
    );

    let input = "(()(()))";

    // all of the span
    let all_of_the_span = StartAndLenSpan::<u32, u16>::from_start_len(0, input.len() as u16);

    let result: Result<
        AST<ParenthesesVariable, StartAndLenSpan<u32, u16>, String>,
        AST<ParenthesesVariable, StartAndLenSpan<u32, u16>, String>,
    > = input.minimal_parse(&rules, &ParenthesesVariable::Open, &all_of_the_span);
    assert_eq!(
        result.unwrap().span,
        StartAndLenSpan::from_lo_hi(0, 8, input)
    );

    let input = "(()(())))";

    // all of the span
    let all_of_the_span = StartAndLenSpan::<u32, u16>::from_start_len(0, input.len() as u16);

    let result: Result<
        AST<ParenthesesVariable, StartAndLenSpan<u32, u16>, String>,
        AST<ParenthesesVariable, StartAndLenSpan<u32, u16>, String>,
    > = input.minimal_parse(&rules, &ParenthesesVariable::Open, &all_of_the_span);
    assert!(result.is_err());
}
