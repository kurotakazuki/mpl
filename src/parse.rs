use crate::cst::{CST, InternalNode, LeafNode};
use crate::rules::Rules;
use crate::span::{Span, SpanHi, SpanLen, SpanTrait};
use crate::symbols::E::*;
use std::hash::Hash;

/// This is terminal.
/// I is input.
pub trait Terminal<S, L, Slice> {
    fn into_slice_and_n(&self, pos: &S, slice: Slice) -> Result<S, ()>;
    fn check(&self, pos: &S, slice: Slice) -> Result<S, ()>;
}

// pub trait ParseSlice {

// }

pub trait Input<S, L> {
    /// This method represents the start and length of the input.
    /// Recommended span.start is 0.
    fn len(&self) -> Span<S, L>;
}

/// T is (enum of) Terminal type.
/// V is (enum of) Variable.
/// S is span.start.
/// L is span.len.
pub trait Parse<'a, T, V, S, L>: Input<S, L>
where
    V: Copy + Eq + Hash,
    S: Clone + Ord + From<L> + SpanHi<L, Self>,
    L: Clone + Ord + SpanLen<S, Self>,
    Span<S, L>: SpanTrait<S, L, Self>,
{

    fn parse(
        &self,
        rules: &Rules<T, V>,
        start_variable: &V,
        span: Option<Span<S, L>>,
    ) -> Result<CST<T, V, S, L>, ()> {
        let span = match span {
            Some(span) => span,
            None => self.len(),
        };
        self.eval(&span.start, &rules, &start_variable, &span)
    }

    fn eval(
        &self,
        pos: &S,
        rules: &Rules<T, V>,
        variable: &V,
        span: &Span<S, L>,
    ) -> Result<CST<T, V, S, L>, ()> {
        let right_rule = rules
            .get(variable)
            .expect("Get the right_rule from a variable");

        // First choice
        // left-hand side of first choice
        let left_cst: Result<CST<T, V, S, L>, ()> = match &right_rule.first.lhs {
            T(terminal_symbol) => {
                unimplemented!()
            }
            V(lhs_of_fc_v) => self.eval(pos, rules, &lhs_of_fc_v, span),
        };

        if let Ok(left_cst) = left_cst {
            // right-hand side of first choice
            let right_cst: Result<CST<T, V, S, L>, ()> = match &right_rule.first.rhs {
                T(terminal_symbol) => {
                    unimplemented!()
                }
                V(rhs_of_fc_v) => self.eval(&left_cst.span.hi(self), rules, &rhs_of_fc_v, span),
            };

            if let Ok(right_cst) = right_cst {
                let stretched_span = left_cst.span.stretch(&right_cst.span, self);
                return Ok(CST::internal_node(InternalNode::from_first(*variable, left_cst, right_cst), stretched_span));
            }
        }

        // Second choice
        match &right_rule.second.0 {
            T(terminal_symbol) => {
                unimplemented!()
            }
            V(sc_v) => {
                let cst = self.eval(pos, rules, &sc_v, span)?;
                let span = cst.span.clone();
                Ok(CST::internal_node(InternalNode::from_second(*variable, cst), span))
            },
        }
    }
}
