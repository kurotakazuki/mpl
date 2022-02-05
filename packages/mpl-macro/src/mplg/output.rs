use crate::mplg::MplgVariable;
use mpl::choices::{Choice, First, Second};
use mpl::output::Output;
use mpl::rules::{RightRule, Rule};
use mpl::span::{Span, StartAndLenSpan};
use mpl::symbols::U8SliceTerminal;
use mpl::symbols::{Metasymbol, TerminalSymbol, E};
use mpl::trees::{Node, AST, CST};

#[derive(Clone, Debug)]
pub enum MplgOutput<'a> {
    Lines(Vec<MplgOutput<'a>>),
    Rule(Rule<U8SliceTerminal<'a>, &'a str>),
    Str(&'a str),
    E(E<U8SliceTerminal<'a>, &'a str>),
}

impl<'a> MplgOutput<'a> {
    pub fn to_lines(self) -> Vec<MplgOutput<'a>> {
        match self {
            MplgOutput::Lines(l) => l,
            _ => panic!("expect lines"),
        }
    }

    fn to_str(self) -> &'a str {
        match self {
            MplgOutput::Str(s) => s,
            _ => panic!("expect str"),
        }
    }

    fn to_e(self) -> E<U8SliceTerminal<'a>, &'a str> {
        match self {
            MplgOutput::E(e) => e,
            _ => panic!("expect E"),
        }
    }
}

impl<'i> Output<'i, [u8], MplgVariable, StartAndLenSpan<u32, u32>> for MplgOutput<'i> {
    fn output_ast(
        input: &'i [u8],
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
            MplgVariable::LineComment | MplgVariable::Variable => {
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
                let fl = rule2.lhs.into_original().unwrap().to_e();
                let rule4 = rule2.rhs.into_first().unwrap().rhs.into_first().unwrap();
                // rhs
                let fr = rule4.lhs.into_original().unwrap().to_e();
                let rule6 = rule4.rhs.into_first().unwrap().rhs.into_first().unwrap();
                // Second
                let s = rule6.lhs.into_original().unwrap().to_e();

                let rule = Rule::new(variable, RightRule::new(First::new(fl, fr), Second::new(s)));
                AST::from_leaf(TerminalSymbol::from_original(MplgOutput::Rule(rule)), span)
            }
            // E
            MplgVariable::E => {
                let span = cst.span;
                match cst.node.equal {
                    Choice::First(first) => {
                        let e = first.lhs.into_original().expect("Terminal symbol");
                        AST::from_leaf(TerminalSymbol::from_original(e), span)
                    }
                    Choice::Second(second) => {
                        let s = second.0.into_original().expect("Variable");
                        AST::from_leaf(
                            TerminalSymbol::from_original(MplgOutput::E(E::V(s.to_str()))),
                            span,
                        )
                    }
                }
            }
            // Terminal symbol
            MplgVariable::TerminalSymbol => {
                let span = cst.span;
                let expr = cst.node.equal.into_first().unwrap().lhs;
                let literal_expr = expr.into_first().unwrap().lhs.into_internal().unwrap();
                let e = match *literal_expr.equal {
                    Choice::First(first) => first.lhs.into_original().expect("Metasymbol literal"),
                    Choice::Second(second) => second.0.into_original().expect("String literal"),
                };
                AST::from_leaf(TerminalSymbol::from_original(e), span)
            }
            // Metasymbol
            MplgVariable::EmptyLiteral => AST::from_leaf(
                TerminalSymbol::from_original(MplgOutput::E(Metasymbol::Empty.into())),
                cst.span,
            ),
            MplgVariable::FailureLiteral => AST::from_leaf(
                TerminalSymbol::from_original(MplgOutput::E(Metasymbol::Failure.into())),
                cst.span,
            ),
            MplgVariable::AnyLiteral => {
                let n = cst.span.len as usize;
                AST::from_leaf(
                    TerminalSymbol::from_original(MplgOutput::E(Metasymbol::Any(n).into())),
                    cst.span,
                )
            }
            MplgVariable::AllLiteral => AST::from_leaf(
                TerminalSymbol::from_original(MplgOutput::E(Metasymbol::All.into())),
                cst.span,
            ),
            MplgVariable::MetasymbolLiteral => {
                let span = cst.span;
                let mut choice = cst.node.equal;
                loop {
                    match choice {
                        Choice::First(first) => {
                            let meta = first.lhs.into_original().expect("Metasymbol");
                            return AST::from_leaf(TerminalSymbol::from_original(meta), span);
                        }
                        Choice::Second(second) => {
                            choice = *second.0.node.into_internal().expect("internal").equal;
                        }
                    }
                }
            }
            // Original
            MplgVariable::StringLiteral => {
                let lo = cst.span.start as usize;
                let hi = cst.span.hi(input) as usize;
                let s = std::str::from_utf8(&input[lo + 1..hi - 1]).expect("str");

                AST::from_leaf(
                    TerminalSymbol::from_original(MplgOutput::E(
                        TerminalSymbol::Original(s.into()).into(),
                    )),
                    cst.span,
                )
            }
            _ => AST::from_cst(cst),
        }
    }
}
