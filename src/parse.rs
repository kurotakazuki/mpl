use crate::cst::{InternalNode, LeafNode, CST};
use crate::rules::Rules;
use crate::span::{Span};
use crate::symbols::E::*;
use std::hash::Hash;

/// This is terminal.
/// I is input.
// pub trait Terminal<S, L, I>
// where
//     S: Clone + Ord + From<L> + SpanHi<L, I>,
//     L: Clone + Ord + SpanLen<S, I>,
//     I: Input<S, L>,
//     Span<S, L>: SpanTrait<S, L, I>,
// {
//     fn into_slice_and_n(&self, pos: &S, slice: &I) -> Result<S, ()>;
//     fn check(&self, pos: &S, input: &I) -> Result<S, ()> {
//                 if input.len().hi() >= pos + l && &self.as_bytes()[pos..pos + l] == literal.as_bytes() {
//                     RuleResult::Matched(pos + l, ())
//                 } else {
//                     RuleResult::Failed
//                 }
//     }
// }

// pub trait ParseSlice {

// }

pub trait Input<S, P> {
    /// This method represents the start and length of the input.
    /// Recommended span.start is 0.
    fn len(&self) -> S;
}

/// T is (enum of) Terminal type.
/// V is (enum of) Variable.
/// S is span.start.
/// L is span.len.
pub trait Parse<'a, T, V, S, P>: Input<S, P>
where
    V: Copy + Eq + Hash,
    S: Span<P>
{
    fn parse(
        &self,
        rules: &Rules<T, V>,
        start_variable: &V,
        span: Option<S>,
    ) -> Result<CST<T, V, S>, ()> {
        let span = match span {
            Some(span) => span,
            None => self.len(),
        };
        self.eval(&span.lo(), &rules, &start_variable, &span)
    }

    fn eval(
        &self,
        pos: &P,
        rules: &Rules<T, V>,
        variable: &V,
        span: &S,
    ) -> Result<CST<T, V, S>, ()> {
        let right_rule = rules
            .get(variable)
            .expect("Get the right_rule from a variable");

        // First choice
        // left-hand side of first choice
        let left_cst: Result<CST<T, V, S>, ()> = match &right_rule.first.lhs {
            T(terminal_symbol) => {
                unimplemented!()
            }
            V(lhs_of_fc_v) => self.eval(pos, rules, &lhs_of_fc_v, span),
        };

        if let Ok(left_cst) = left_cst {
            // right-hand side of first choice
            let right_cst: Result<CST<T, V, S>, ()> = match &right_rule.first.rhs {
                T(terminal_symbol) => {
                    unimplemented!()
                }
                V(rhs_of_fc_v) => self.eval(&left_cst.span.hi(), rules, &rhs_of_fc_v, span),
            };

            if let Ok(right_cst) = right_cst {
                let stretched_span = left_cst.span.stretch(&right_cst.span);
                return Ok(CST::internal_node(
                    InternalNode::from_first(*variable, left_cst, right_cst),
                    stretched_span,
                ));
            }
        }

        // Second choice
        match &right_rule.second.0 {
            T(terminal_symbol) => {
                unimplemented!()
                // if self.len() >= pos + l && &self.as_bytes()[pos..pos + l] == literal.as_bytes() {
                //     RuleResult::Matched(pos + l, ())
                // } else {
                //     RuleResult::Failed
                // }
            }
            V(sc_v) => {
                let cst = self.eval(pos, rules, &sc_v, span)?;
                let span = cst.span.clone();
                Ok(CST::internal_node(
                    InternalNode::from_second(*variable, cst),
                    span,
                ))
            }
        }
    }
}
