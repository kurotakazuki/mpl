use crate::cst::{InternalNode, LeafNode, CST};
use crate::position::Position;
use crate::rules::Rules;
use crate::span::{Span};
use crate::symbols::{E::*, Metasymbol, Terminal, TerminalSymbol};
use std::hash::Hash;

// pub trait Variable: Copy + Eq + Hash {}

pub trait Input<S, P> {
    /// This method represents the start and length of the input.
    /// Recommended span.start is 0.
    fn span(&self) -> S;
}

/// T is (enum of) Terminal type.
/// V is (enum of) Variable.
/// S is span.start.
/// L is span.len.
pub trait Parse<'a, T, V, S, P>: Input<S, P>
where
    T: Terminal<T, V, S, P, Self>,
    V: Copy + Eq + Hash,
    S: Span<P>,
    P: Position,
{
    fn parse(
        &self,
        rules: &Rules<T, V>,
        start_variable: &V,
        span: Option<S>,
    ) -> Result<CST<T, V, S>, ()> {
        let span = match span {
            Some(span) => span,
            None => self.span(),
        };
        self.eval(&span.lo(), &rules, &start_variable, &span)
    }

    fn into_epsilon_cst(&self, pos: P) -> Result<CST<T, V, S>, ()> {
        Ok(CST::from_leaf_node(
            TerminalSymbol::M(Metasymbol::Epsilon),
            Span::from_lo_hi(pos.clone(), pos)
        ))
    }

    fn into_failed_cst(&self, pos: P) -> Result<CST<T, V, S>, ()> {
        // Ok(CST::from_leaf_node(
        //     TerminalSymbol::M(Metasymbol::Failed),
        //     Span::from_lo_hi(pos.clone(), pos)
        // ))
        Err(())
    }

    fn into_any_cst(&self, pos: P) -> Result<CST<T, V, S>, ()> {
        let pos_with_one_added = pos.with_one_added();
        if self.span().hi() >= pos_with_one_added {
            Ok(CST::from_leaf_node(
                TerminalSymbol::M(Metasymbol::Any),
                Span::from_lo_hi(pos.clone(), pos_with_one_added)
            ))
        } else {
            Err(())
        }
    }

    fn into_all_cst(&self, pos: P) -> Result<CST<T, V, S>, ()> {
        Ok(CST::from_leaf_node(
            TerminalSymbol::M(Metasymbol::All),
            Span::from_lo_hi(pos, self.span().hi())
        ))
    }

    fn eval_terminal_symbol(&self, terminal_symbol: &LeafNode<T>, pos: P)-> Result<CST<T, V, S>, ()> {
        match terminal_symbol {
            TerminalSymbol::Original(t) => {
                t.eval(self, pos)
            }
            TerminalSymbol::M(metasymbol) => {
                match metasymbol {
                    Metasymbol::Epsilon => self.into_epsilon_cst(pos),
                    Metasymbol::Failed => self.into_failed_cst(pos),
                    Metasymbol::Any => self.into_any_cst(pos),
                    Metasymbol::All => self.into_all_cst(pos),
                }
            }
        }
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
                self.eval_terminal_symbol(terminal_symbol, pos.clone())
            }
            V(lhs_of_fc_v) => self.eval(pos, rules, &lhs_of_fc_v, span),
        };

        if let Ok(left_cst) = left_cst {
            // right-hand side of first choice
            let right_cst: Result<CST<T, V, S>, ()> = match &right_rule.first.rhs {
                T(terminal_symbol) => {
                    self.eval_terminal_symbol(terminal_symbol, pos.clone())
                }
                V(rhs_of_fc_v) => self.eval(&left_cst.span.hi(), rules, &rhs_of_fc_v, span),
            };

            if let Ok(right_cst) = right_cst {
                let merged_span = Span::merge_lhs_and_rhs(&left_cst.span, &right_cst.span);
                return Ok(CST::from_internal_node(
                    InternalNode::from_first(*variable, left_cst, right_cst),
                    merged_span,
                ));
            }
        }

        // Second choice
        match &right_rule.second.0 {
            T(terminal_symbol) => {
                self.eval_terminal_symbol(terminal_symbol, pos.clone())
            }
            V(sc_v) => {
                let cst = self.eval(pos, rules, &sc_v, span)?;
                let span = cst.span.clone();
                Ok(CST::from_internal_node(
                    InternalNode::from_second(*variable, cst),
                    span,
                ))
            }
        }
    }
}
