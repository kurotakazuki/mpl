use crate::mplg::MplgVariable;
use mpl::choices::{Choice, First, Second};
use mpl::output::Output;
use mpl::rules::{RightRule, Rule};
use mpl::span::{Span, StartAndLenSpan};
use mpl::symbols::TerminalSymbol;
use mpl::trees::{Node, AST, CST};

#[derive(Clone, Debug)]
pub enum MplgOutput<'a> {
    Lines(Vec<MplgOutput<'a>>),
    Str(&'a str),
    Rule(Rule<&'a str, &'a str>),
}

impl<'a> MplgOutput<'a> {
    pub fn to_lines(self) -> Vec<MplgOutput<'a>> {
        match self {
            MplgOutput::Lines(l) => l,
            _ => panic!(),
        }
    }

    fn to_str(self) -> &'a str {
        match self {
            MplgOutput::Str(s) => s,
            _ => panic!(),
        }
    }
}

impl<'input> Output<'input, [u8], MplgVariable, StartAndLenSpan<u32, u32>> for MplgOutput<'input> {
    fn output_ast(
        input: &'input [u8],
        cst: CST<MplgVariable, StartAndLenSpan<u32, u32>, Self>,
    ) -> AST<MplgVariable, StartAndLenSpan<u32, u32>, Self> {
        match cst.node.value {
            MplgVariable::Mplg => {
                let span = cst.span;
                let mut lines = Vec::new();
                let mut zero_or_more_lines = cst.node.equal.into_first().unwrap().lhs;
                loop {
                    match zero_or_more_lines.node {
                        // ZeroOrMoreLines
                        Node::Internal(internal) => {
                            let first = internal.into_first().expect("first");
                            // Expect MplgOutput::Str or MplgOutput::Rule.
                            // None if Metasymbol::Empty.
                            if let Some(o) = first.lhs.into_original() {
                                lines.push(o);
                            }
                            zero_or_more_lines = first.rhs;
                        }
                        // ()
                        Node::Leaf(_) => {
                            return AST::from_leaf(
                                TerminalSymbol::Original(MplgOutput::Lines(lines)),
                                span,
                            );
                        }
                    }
                }
            }
            MplgVariable::Line => {
                let span = cst.span;
                let first = cst.node.equal.into_first().unwrap();
                let line1 = first.lhs.node.into_internal().unwrap();
                match *line1.equal {
                    Choice::First(first) => {
                        let mut line_comment = first.lhs;
                        line_comment.span = span;
                        line_comment
                    }
                    Choice::Second(second) => {
                        let line2 = second.0;
                        match line2.node {
                            Node::Internal(internal) => {
                                let mut rule = internal.into_first().unwrap().lhs;
                                rule.span = span;
                                rule
                            }
                            Node::Leaf(leaf) => AST::from_leaf(leaf, span),
                        }
                    }
                }
            }
            MplgVariable::LineComment | MplgVariable::Variable | MplgVariable::E => {
                let lo = cst.span.start as usize;
                let hi = cst.span.hi(input) as usize;
                let s = std::str::from_utf8(&input[lo..hi]).expect("str");

                AST::from_leaf(TerminalSymbol::from_original(MplgOutput::Str(s)), cst.span)
            }
            MplgVariable::Rule => {
                let span = cst.span;
                let first = cst.node.equal.into_first().unwrap();
                let variable = first.lhs.into_original().unwrap().to_str();
                let rule2 = first.rhs.into_first().unwrap().rhs.into_first().unwrap();
                // First
                // lhs
                let fl = rule2.lhs.into_original().unwrap().to_str();
                let rule4 = rule2.rhs.into_first().unwrap().rhs.into_first().unwrap();
                // rhs
                let fr = rule4.lhs.into_original().unwrap().to_str();
                let rule6 = rule4.rhs.into_first().unwrap().rhs.into_first().unwrap();
                // Second
                let s = rule6.lhs.into_original().unwrap().to_str();

                let rule = Rule::new(variable, RightRule::new(First::new(fl, fr), Second::new(s)));
                AST::from_leaf(TerminalSymbol::from_original(MplgOutput::Rule(rule)), span)
            }
            _ => AST::from_cst(cst),
        }
    }
}
