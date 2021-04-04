// use crate::choice::Choice;
use crate::cst::{InternalNode, CST};
use crate::input::Input;
use crate::output::Output;
use crate::position::Position;
use crate::rules::Rules;
use crate::span::Span;
use crate::symbols::{Metasymbol, Terminal, TerminalSymbol, Variable, E};

/// T is terminal symbols.
/// OutputT is output type.
/// V is (enum of) Variables.
/// S is Span.
/// P is position.
pub trait Parse<'input, T, OutputT, V, S, P>: Input<'input, S>
where
    T: Clone + Terminal<'input, Self, OutputT, V, S, P>,
    //TODO
    // OutputT: TryFrom<(&'input Self, V, S, Choice<CST<OutputT, V, S>>)>,
    OutputT: Output<'input, Self, V, S>,
    V: Variable,
    S: Span<P>,
    P: Position,
{
    fn mpg_parse(
        &'input self,
        rules: &'input Rules<T, V>,
        start_variable: &V,
        all_of_the_span: Option<S>,
    ) -> Result<CST<OutputT, V, S>, ()> {
        let all_of_the_span = match all_of_the_span {
            Some(all_of_the_span) => all_of_the_span,
            None => self.all_of_the_span(),
        };
        let cst = self.eval(
            &all_of_the_span.lo(),
            &rules,
            &start_variable,
            &all_of_the_span,
        )?;

        if cst.span == all_of_the_span {
            Ok(cst)
        } else {
            Err(())
        }
    }

    fn into_epsilon_cst(&'input self, pos: P) -> Result<CST<OutputT, V, S>, ()> {
        Ok(CST::from_leaf_node(
            TerminalSymbol::M(Metasymbol::Epsilon),
            Span::from_lo_hi(pos.clone(), pos),
        ))
    }

    fn into_failed_cst(&'input self, _pos: P) -> Result<CST<OutputT, V, S>, ()> {
        // Ok(CST::from_leaf_node(
        //     TerminalSymbol::M(Metasymbol::Failed),
        //     Span::from_lo_hi(pos.clone(), pos)
        // ))
        Err(())
    }

    fn into_any_cst(&'input self, pos: P) -> Result<CST<OutputT, V, S>, ()> {
        let pos_with_one_added = pos.with_one_added();
        if self.all_of_the_span().hi() >= pos_with_one_added {
            Ok(CST::from_leaf_node(
                TerminalSymbol::M(Metasymbol::Any),
                Span::from_lo_hi(pos.clone(), pos_with_one_added),
            ))
        } else {
            Err(())
        }
    }

    fn into_all_cst(&'input self, pos: P) -> Result<CST<OutputT, V, S>, ()> {
        Ok(CST::from_leaf_node(
            TerminalSymbol::M(Metasymbol::All),
            Span::from_lo_hi(pos, self.all_of_the_span().hi()),
        ))
    }

    fn eval_terminal_symbol(
        &'input self,
        terminal_symbol: &'input TerminalSymbol<T>,
        pos: P,
        all_of_the_span: &S,
    ) -> Result<CST<OutputT, V, S>, ()> {
        match terminal_symbol {
            TerminalSymbol::Original(t) => t.eval(self, pos, all_of_the_span),
            TerminalSymbol::M(metasymbol) => match metasymbol {
                Metasymbol::Epsilon => self.into_epsilon_cst(pos),
                Metasymbol::Failed => self.into_failed_cst(pos),
                Metasymbol::Any => self.into_any_cst(pos),
                Metasymbol::All => self.into_all_cst(pos),
            },
        }
    }

    fn eval(
        &'input self,
        pos: &P,
        rules: &'input Rules<T, V>,
        variable: &V,
        all_of_the_span: &S,
    ) -> Result<CST<OutputT, V, S>, ()> {
        let right_rule = rules
            .0
            .get(variable)
            .expect("Get the right_rule from a variable");

        // First choice
        // left-hand side of first choice
        let left_cst: Result<CST<OutputT, V, S>, ()> = match &right_rule.first.lhs {
            E::T(terminal_symbol) => {
                self.eval_terminal_symbol(terminal_symbol, pos.clone(), all_of_the_span)
            }
            E::V(lhs_of_fc_v) => self.eval(pos, rules, &lhs_of_fc_v, all_of_the_span),
        };

        if let Ok(left_cst) = left_cst {
            // right-hand side of first choice
            let right_cst: Result<CST<OutputT, V, S>, ()> = match &right_rule.first.rhs {
                E::T(terminal_symbol) => {
                    self.eval_terminal_symbol(terminal_symbol, pos.clone(), all_of_the_span)
                }
                E::V(rhs_of_fc_v) => {
                    self.eval(&left_cst.span.hi(), rules, &rhs_of_fc_v, all_of_the_span)
                }
            };

            if let Ok(right_cst) = right_cst {
                let merged_span = Span::merge_lhs_and_rhs(&left_cst.span, &right_cst.span);

                // let output = OutputT::try_from((self, *variable, merged_span.clone(), Choice::first(left_cst, right_cst)));
                let output = OutputT::new(&self, *variable, merged_span.clone());

                return Ok(CST::from_internal_node(
                    InternalNode::from_first((*variable, output), left_cst, right_cst),
                    merged_span,
                ));

                // return Ok(CST::from_internal_node(
                //     InternalNode::from_first((*variable, output.ok()), , right_cst),
                //     merged_span,
                // ));

                // if let Ok(output) = output {
                //     return Ok(CST::from_internal_node(
                //         InternalNode::from_first((*variable, Some(output)), left_cst, right_cst),
                //         merged_span,
                //     ));
                // } else {
                //     return Ok(CST::from_internal_node(
                //         InternalNode::from_first((*variable, None), left_cst, right_cst),
                //         merged_span,
                //     ));
                // }
            }
        }

        // Second choice
        match &right_rule.second.0 {
            E::T(terminal_symbol) => {
                self.eval_terminal_symbol(terminal_symbol, pos.clone(), all_of_the_span)
            }
            E::V(sc_v) => {
                let cst = self.eval(pos, rules, &sc_v, all_of_the_span)?;
                let span = cst.span.clone();

                let output = OutputT::new(self, *variable, span.clone());

                Ok(CST::from_internal_node(
                    InternalNode::from_second((*variable, output), cst),
                    span,
                ))
            }
        }
    }
}
