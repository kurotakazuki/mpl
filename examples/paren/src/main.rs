use mpl::parser::Parser;
use mpl::rules::{RightRule, Rules};
use mpl::span::StartAndLenSpan;
use mpl::symbols::U8SliceTerminal;
use mpl::trees::AST;
use mpl_macro::Parse;
#[derive(Parse, Debug)]
#[mplg = "../../examples/paren/parentheses.mplg"] // TODO Use release mpl_macro
pub struct ParenParser;

impl<'a> Rules<U8SliceTerminal<'a>, ParenVariable> for ParenRules {
    fn get(
        &self,
        variable: &ParenVariable,
    ) -> Option<&RightRule<U8SliceTerminal<'a>, ParenVariable>> {
        Some(match variable {
            ParenVariable::Open => &Self::Open_RULE,
            ParenVariable::Parentheses => &Self::Parentheses_RULE,
            ParenVariable::Close => &Self::Close_RULE,
        })
    }
}

enum ParseResult {
    Ok,
    Err,
}

const INPUTS: [(&str, ParseResult); 8] = [
    // Ok
    ("", ParseResult::Ok),
    ("()", ParseResult::Ok),
    ("()(())", ParseResult::Ok),
    ("(()(()))", ParseResult::Ok),
    // Err
    ("(", ParseResult::Err),
    (")", ParseResult::Err),
    ("()())", ParseResult::Err),
    ("(()(())))", ParseResult::Err),
];

fn main() {
    let parser = ParenParser;

    for input in INPUTS {
        let input_data = input.0.as_bytes();
        let parse_result = input.1;
        // all of the span
        let all_of_the_span =
            StartAndLenSpan::<u32, u16>::from_start_len(0, input_data.len() as u16);
        let result: Result<
            AST<ParenVariable, StartAndLenSpan<u32, u16>, ()>,
            AST<ParenVariable, StartAndLenSpan<u32, u16>, ()>,
        > = parser.parse(
            input_data,
            &ParenRules,
            &ParenVariable::Open,
            &all_of_the_span,
        );
        match parse_result {
            ParseResult::Ok => assert!(result.is_ok()),
            ParseResult::Err => assert!(result.is_err()),
        }
    }
}
