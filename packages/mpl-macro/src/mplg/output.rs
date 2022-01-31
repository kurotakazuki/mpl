use crate::mplg::{MplgRules, MplgVariables};
use mpl::choices::Choice;
use mpl::output::Output;
use mpl::span::{Span, StartAndLenSpan};
use mpl::symbols::{TerminalSymbol, U8SliceTerminal};
use mpl::trees::{Node, AST, CST};
use std::convert::TryInto;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MplgOutput<'a> {
    Str(&'a str),
}

impl<'input> Output<'input, [u8], MplgVariables, StartAndLenSpan<u32, u32>> for MplgOutput<'input> {
    fn output_ast(
        input: &'input [u8],
        cst: CST<MplgVariables, StartAndLenSpan<u32, u32>, Self>,
    ) -> AST<MplgVariables, StartAndLenSpan<u32, u32>, Self> {
        match cst.node.value {
            MplgVariables::Variable | MplgVariables::E => {
                let lo = cst.span.start as usize;
                let hi = cst.span.hi(input) as usize;
                let s = std::str::from_utf8(&input[lo..hi]).expect("str");

                AST::from_leaf(TerminalSymbol::from_original(MplgOutput::Str(s)), cst.span)
            }
            // MplgVariables::Mplg => {
            //     let span = cst.span;
            //     let mut bub_fns = Mplg::new();
            //     let mut first = cst.node.equal.into_first().unwrap();
            //     loop {
            //         bub_fns.push(*first.lhs.into_original().unwrap().into_bub_fn().unwrap());
            //         match first.rhs.node {
            //             // ZeroOrMoreMplg
            //             Node::Internal(internal) => {
            //                 first = internal.into_first().unwrap();
            //             }
            //             // ()
            //             Node::Leaf(_) => {
            //                 return AST::from_leaf(TerminalSymbol::Original(bub_fns.into()), span);
            //             }
            //         }
            //     }
            // }
            // // Into First rhs Child Node
            // MplgVariables::SpaceAndBubFn => {
            //     let span = cst.span;
            //     let mut rhs_child = cst.node.equal.into_first().unwrap().rhs;
            //     rhs_child.span = span;

            //     rhs_child
            // }
            // MplgVariables::BubFn => {
            //     let span = cst.span;
            //     let bub_fn_equal = cst.node.equal.into_first().unwrap();
            //     let x0 = bub_fn_equal.lhs;

            //     let bub_fn1_equal = bub_fn_equal.rhs.into_first().unwrap();
            //     let y0 = bub_fn1_equal.lhs;

            //     let bub_fn2_equal = bub_fn1_equal.rhs.into_first().unwrap();
            //     let z0 = bub_fn2_equal.lhs;

            //     let bub_fn3_equal = bub_fn2_equal.rhs.into_first().unwrap();
            //     let domain = bub_fn3_equal.lhs;

            //     let bub_fn4_equal = bub_fn3_equal.rhs.into_second().unwrap();
            //     let volume = bub_fn4_equal.0;

            //     let bub_fn = BubFn {
            //         bub_absolute_coord: (x0, y0, z0),
            //         domain,
            //         volume,
            //     };

            //     AST::from_leaf(TerminalSymbol::Original(bub_fn.into()), span)
            // }
            // // Into First lhs Child Node
            // MplgVariables::SumAndSpace | MplgVariables::OrOrExprAndSpace => {
            //     let span = cst.span;
            //     let mut lhs_child = cst.node.equal.into_first().unwrap().lhs;
            //     lhs_child.span = span;

            //     lhs_child
            // }
            // MplgVariables::Comparison
            // | MplgVariables::Atom
            // | MplgVariables::PlusOrMinus
            // | MplgVariables::StarOrSlash
            // | MplgVariables::Function
            // | MplgVariables::Variable => {
            //     let mut equal = cst.node.equal;
            //     loop {
            //         match equal {
            //             Choice::First(first) => {
            //                 return first.lhs;
            //             }
            //             Choice::Second(second) => {
            //                 equal = *second.0.node.into_internal().unwrap().equal;
            //             }
            //         }
            //     }
            // }
            // MplgVariables::FloatLiteral | MplgVariables::IntegerLiteral => {
            //     let n = match cst.node.equal {
            //         Choice::First(_) => {
            //             let lo = cst.span.start as usize;
            //             let hi = cst.span.hi(input) as usize;

            //             std::str::from_utf8(&input[lo..hi])
            //                 .unwrap()
            //                 .parse::<f64>()
            //                 .unwrap()
            //                 .into()
            //         }
            //         Choice::Second(second) => second.0.into_original().unwrap(),
            //     };

            //     AST::from_leaf(TerminalSymbol::from_original(n), cst.span)
            // }
            // MplgVariables::BytesF64Literal => {
            //     let lo = cst.span.start as usize + 1;
            //     let hi = cst.span.hi(input) as usize;

            //     let n: f64 = f64::from_le_bytes(
            //         input[lo..hi]
            //             .try_into()
            //             .expect("slice with incorrect length"),
            //     );

            //     AST::from_leaf(TerminalSymbol::from_original(n.into()), cst.span)
            // }
            // MplgVariables::Constant => match cst.node.equal {
            //     Choice::First(first) => first.lhs,
            //     Choice::Second(second) => second.0,
            // },
            // MplgVariables::Constant1 => {
            //     let o = cst
            //         .node
            //         .equal
            //         .into_first()
            //         .unwrap()
            //         .lhs
            //         .into_original()
            //         .unwrap();
            //     AST::from_leaf(TerminalSymbol::from_original(o), cst.span)
            // }
            // MplgVariables::E => AST::from_leaf(
            //     TerminalSymbol::from_original(std::f64::consts::E.into()),
            //     cst.span,
            // ),
            // MplgVariables::Pi => AST::from_leaf(
            //     TerminalSymbol::from_original(std::f64::consts::PI.into()),
            //     cst.span,
            // ),
            // // First lhs into Second
            // // A = B () / f
            // // A = B
            // MplgVariables::BubFn4
            // | MplgVariables::OrOr
            // | MplgVariables::AndAnd
            // | MplgVariables::EqEq
            // | MplgVariables::Ne
            // | MplgVariables::Ge
            // | MplgVariables::Le
            // | MplgVariables::Gt
            // | MplgVariables::Lt
            // | MplgVariables::UppercaseX
            // | MplgVariables::UppercaseY
            // | MplgVariables::UppercaseZ
            // | MplgVariables::LowercaseX
            // | MplgVariables::LowercaseY
            // | MplgVariables::LowercaseZ
            // | MplgVariables::UppercaseN
            // | MplgVariables::LowercaseN
            // | MplgVariables::UppercaseF
            // | MplgVariables::UppercaseS
            // | MplgVariables::Plus
            // | MplgVariables::Minus
            // | MplgVariables::Star
            // | MplgVariables::Slash
            // | MplgVariables::Space => {
            //     if let Choice::First(first) = cst.node.equal {
            //         cst.node.equal = first.lhs.into();
            //     }
            //     AST::from_cst(cst)
            // }
            _ => AST::from_cst(cst),
        }
    }
}
