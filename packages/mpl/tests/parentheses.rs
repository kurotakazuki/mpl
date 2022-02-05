use mpl::output::Output;
use mpl::parser::Parser;
use mpl::rules::{RightRule, RightRuleKind};
use mpl::span::StartAndLenSpan;
use mpl::symbols::{StrTerminal, U8SliceTerminal, Variable};
use mpl::trees::{AST, CST};
use std::collections::HashMap;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum ParenthesesVariable {
    Open,
    Parentheses,
    Close,
}

impl Variable for ParenthesesVariable {}

enum ParseResult {
    Ok,
    Err,
}

const INPUTS: [(&str, ParseResult); 8] = [
    // Ok
    ("()", ParseResult::Ok),
    ("()(())", ParseResult::Ok),
    ("(()(()))", ParseResult::Ok),
    // Err
    ("", ParseResult::Ok),
    ("(", ParseResult::Err),
    (")", ParseResult::Err),
    ("()())", ParseResult::Err),
    ("(()(())))", ParseResult::Err),
];

struct ParenthesesParser;

/// ```
/// Open = '(' Parentheses / ()
/// Parentheses = Open Close / f
/// Close = ")" Open / f
/// ```
#[test]
fn str_parentheses() {
    impl<'i> Output<'i, str, ParenthesesVariable, StartAndLenSpan<u32, u16>> for String {
        fn output_ast(
            _input: &'i str,
            cst: CST<ParenthesesVariable, StartAndLenSpan<u32, u16>, Self>,
        ) -> AST<ParenthesesVariable, StartAndLenSpan<u32, u16>, Self> {
            match cst.node.value {
                ParenthesesVariable::Open => {
                    AST::from_cst_and_output(cst, Some(String::from("open")))
                }
                ParenthesesVariable::Parentheses => {
                    AST::from_cst_and_output(cst, Some(String::from("paren")))
                }
                ParenthesesVariable::Close => {
                    AST::from_cst_and_output(cst, Some(String::from("close")))
                }
            }
        }
    }

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

    let parser = ParenthesesParser;

    for input in INPUTS {
        let input_data = input.0;
        let parse_result = input.1;
        // all of the span
        let all_of_the_span =
            StartAndLenSpan::<u32, u16>::from_start_len(0, input_data.len() as u16);
        let result: Result<
            AST<ParenthesesVariable, StartAndLenSpan<u32, u16>, String>,
            AST<ParenthesesVariable, StartAndLenSpan<u32, u16>, String>,
        > = parser.parse(
            input_data,
            &rules,
            &ParenthesesVariable::Open,
            &all_of_the_span,
        );
        match parse_result {
            ParseResult::Ok => assert!(result.is_ok()),
            ParseResult::Err => assert!(result.is_err()),
        }
    }
}

/// ```
/// Open = '(' Parentheses / ()
/// Parentheses = Open Close / f
/// Close = ")" Open / f
/// ```
#[test]
fn u8_slice_parentheses() {
    let mut rules = HashMap::new();

    rules.insert(
        ParenthesesVariable::Open,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(U8SliceTerminal::Char('(')),
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
                RightRuleKind::T(U8SliceTerminal::Str(")")),
                RightRuleKind::V(ParenthesesVariable::Open),
            ),
            RightRuleKind::Failure,
        ),
    );

    let parser = ParenthesesParser;

    for input in INPUTS {
        let input_data = input.0.as_bytes();
        let parse_result = input.1;
        // all of the span
        let all_of_the_span =
            StartAndLenSpan::<u32, u16>::from_start_len(0, input_data.len() as u16);
        let result: Result<
            AST<ParenthesesVariable, StartAndLenSpan<u32, u16>, ()>,
            AST<ParenthesesVariable, StartAndLenSpan<u32, u16>, ()>,
        > = parser.parse(
            input_data,
            &rules,
            &ParenthesesVariable::Open,
            &all_of_the_span,
        );
        match parse_result {
            ParseResult::Ok => assert!(result.is_ok()),
            ParseResult::Err => assert!(result.is_err()),
        }
    }
}
