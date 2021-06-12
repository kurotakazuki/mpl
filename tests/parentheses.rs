use mpl::output::Output;
use mpl::parse::Parse;
use mpl::rules::{RightRule, RightRuleKind, Rule, Rules};
use mpl::span::{Span, StartAndLenSpan};
use mpl::symbols::{StrTerminal, Variable};
use mpl::tree::{AST, CST};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum ParenthesesVariable {
    Open,
    Parentheses,
    Close,
}

impl Variable for ParenthesesVariable {}

impl<'input> Output<'input, str, ParenthesesVariable, StartAndLenSpan<u32, u16>> for String {
    fn output_ast(
        _input: &'input str,
        cst: CST<Self, ParenthesesVariable, StartAndLenSpan<u32, u16>>,
    ) -> AST<Self, ParenthesesVariable, StartAndLenSpan<u32, u16>> {
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
    let open_rule: Rule<StrTerminal, ParenthesesVariable> = Rule::new(
        ParenthesesVariable::Open,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(StrTerminal::Char('(')),
                RightRuleKind::V(ParenthesesVariable::Parentheses),
            ),
            RightRuleKind::Epsilon,
        ),
    );
    let parentheses_rule: Rule<StrTerminal, ParenthesesVariable> = Rule::new(
        ParenthesesVariable::Parentheses,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(ParenthesesVariable::Open),
                RightRuleKind::V(ParenthesesVariable::Close),
            ),
            RightRuleKind::Failure,
        ),
    );
    let close_rule: Rule<StrTerminal, ParenthesesVariable> = Rule::new(
        ParenthesesVariable::Close,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(StrTerminal::Str(")")),
                RightRuleKind::V(ParenthesesVariable::Open),
            ),
            RightRuleKind::Failure,
        ),
    );

    let mut rules = Rules::new();

    rules.insert_rule(open_rule);
    rules.insert_rule(parentheses_rule);
    rules.insert_rule(close_rule);

    let input = "()";

    // all of the span
    let all_of_the_span = StartAndLenSpan::<u32, u16>::from_start_len(0, input.len() as u16);

    let result: Result<AST<String, ParenthesesVariable, StartAndLenSpan<u32, u16>>, ()> =
        input.minimal_parse(&rules, &ParenthesesVariable::Open, all_of_the_span);
    assert_eq!(
        result.unwrap().span,
        StartAndLenSpan::from_lo_hi(0, 2, input)
    );

    let input = "(()(()))";

    // all of the span
    let all_of_the_span = StartAndLenSpan::<u32, u16>::from_start_len(0, input.len() as u16);

    let result: Result<AST<String, ParenthesesVariable, StartAndLenSpan<u32, u16>>, ()> =
        input.minimal_parse(&rules, &ParenthesesVariable::Open, all_of_the_span);
    assert_eq!(
        result.unwrap().span,
        StartAndLenSpan::from_lo_hi(0, 8, input)
    );

    let input = "(()(())))";

    // all of the span
    let all_of_the_span = StartAndLenSpan::<u32, u16>::from_start_len(0, input.len() as u16);

    let result: Result<AST<String, ParenthesesVariable, StartAndLenSpan<u32, u16>>, ()> =
        input.minimal_parse(&rules, &ParenthesesVariable::Open, all_of_the_span);
    assert_eq!(result, Err(()));
}
