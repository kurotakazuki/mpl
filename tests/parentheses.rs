use mpg::tree::{AST, CST};
use mpg::parse::Parse;
use mpg::rules::{RightRule, RightRuleKind, Rule, Rules};
use mpg::span::{ByteSpan, Span};
use mpg::symbols::{StrTerminal, Variable};

use mpg::output::Output;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum ParenthesesVariable {
    Open,
    Parentheses,
    Close,
}

impl Variable for ParenthesesVariable {}

impl<'input> Output<'input, str, ParenthesesVariable, ByteSpan> for String {
    fn output_ast(_input: &'input str, cst: CST<Self, ParenthesesVariable, ByteSpan>) -> AST<Self, ParenthesesVariable, ByteSpan> {
        match cst.node.value {
            ParenthesesVariable::Open => AST::from_cst_and_output(cst, Some(String::from("open"))),
            ParenthesesVariable::Parentheses => AST::from_cst_and_output(cst, Some(String::from("paren"))),
            ParenthesesVariable::Close => AST::from_cst_and_output(cst, Some(String::from("close"))),
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
    let result: Result<AST<String, ParenthesesVariable, ByteSpan>, ()> =
        input.mpg_parse(&rules, &ParenthesesVariable::Open, None);
    assert_eq!(
        result.unwrap().span,
        ByteSpan::from_lo_hi(0.into(), 2.into())
    );

    let input = "(()(()))";
    let result: Result<AST<String, ParenthesesVariable, ByteSpan>, ()> =
        input.mpg_parse(&rules, &ParenthesesVariable::Open, None);
    assert_eq!(
        result.unwrap().span,
        ByteSpan::from_lo_hi(0.into(), 8.into())
    );

    let input = "(()(())))";
    let result: Result<AST<String, ParenthesesVariable, ByteSpan>, ()> =
        input.mpg_parse(&rules, &ParenthesesVariable::Open, None);
    assert_eq!(result, Err(()));
}
