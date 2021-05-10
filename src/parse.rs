use crate::choice::Choice;
use crate::input::Input;
use crate::output::Output;
use crate::position::Position;
use crate::rules::Rules;
use crate::span::Span;
use crate::symbols::{Metasymbol, Terminal, TerminalSymbol, VAndE, Variable, E};
use crate::tree::{AST, CST};

/// T is terminal symbols.
/// OutputT is output type.
/// V is (enum of) Variables.
/// S is Span.
/// P is position.
pub trait Parse<'input, T, OutputT, V, S, P>: Input<S>
where
    T: Terminal<'input, Self, OutputT, V, S, P>,
    //TODO
    // OutputT: TryFrom<(&'input Self, V, S, Choice<AST<OutputT, V, S>>)>,
    OutputT: Output<'input, Self, V, S>,
    V: Variable,
    S: Span<P>,
    P: Position,
{
    // all_of_the_span.unwarp().hi() < input.len()
    fn minimal_parse(
        &'input self,
        rules: &'input Rules<T, V>,
        start_variable: &V,
        all_of_the_span: Option<S>,
    ) -> Result<AST<OutputT, V, S>, ()> {
        let all_of_the_span = match all_of_the_span {
            Some(all_of_the_span) => all_of_the_span,
            None => self.all_of_the_span(),
        };
        let ast = self.eval(
            &all_of_the_span.lo(),
            &rules,
            &start_variable,
            &self.all_of_the_span().hi(),
        )?;

        if ast.span == all_of_the_span {
            Ok(ast)
        } else {
            Err(())
        }
    }

    fn into_epsilon_ast(&'input self, pos: P) -> Result<AST<OutputT, V, S>, ()> {
        Ok(AST::from_leaf_node(
            TerminalSymbol::M(Metasymbol::Epsilon),
            Span::from_lo_hi(pos.clone(), pos),
        ))
    }

    fn into_failed_ast(&'input self, _pos: P) -> Result<AST<OutputT, V, S>, ()> {
        // Ok(AST::from_leaf_node(
        //     TerminalSymbol::M(Metasymbol::Failure),
        //     Span::from_lo_hi(pos.clone(), pos)
        // ))
        Err(())
    }

    fn into_any_ast(&'input self, pos: P, max_pos: &P, n: usize) -> Result<AST<OutputT, V, S>, ()> {
        let mut pos_with_one_added = pos.with_one_added();
        for _ in 1..n {
            pos_with_one_added = pos_with_one_added.with_one_added();
        }
        if &pos_with_one_added <= max_pos {
            Ok(AST::from_leaf_node(
                TerminalSymbol::M(Metasymbol::Any(n)),
                Span::from_lo_hi(pos, pos_with_one_added),
            ))
        } else {
            Err(())
        }
    }

    fn into_all_ast(&'input self, pos: P, max_pos: P) -> Result<AST<OutputT, V, S>, ()> {
        Ok(AST::from_leaf_node(
            TerminalSymbol::M(Metasymbol::All),
            Span::from_lo_hi(pos, max_pos),
        ))
    }

    fn eval_terminal_symbol(
        &'input self,
        terminal_symbol: &'input TerminalSymbol<T>,
        pos: P,
        max_pos: &P,
    ) -> Result<AST<OutputT, V, S>, ()> {
        match terminal_symbol {
            TerminalSymbol::Original(t) => t.eval(self, pos, max_pos),
            TerminalSymbol::M(metasymbol) => match metasymbol {
                Metasymbol::Epsilon => self.into_epsilon_ast(pos),
                Metasymbol::Failure => self.into_failed_ast(pos),
                Metasymbol::Any(n) => self.into_any_ast(pos, max_pos, *n),
                Metasymbol::All => self.into_all_ast(pos, max_pos.clone()),
                Metasymbol::Omit => unimplemented!(),
            },
        }
    }

    fn eval(
        &'input self,
        pos: &P,
        rules: &'input Rules<T, V>,
        variable: &V,
        max_pos: &P,
    ) -> Result<AST<OutputT, V, S>, ()> {
        let right_rule = rules
            .0
            .get(variable)
            .expect("Get the right_rule from a variable");

        // First choice
        // left-hand side of first choice
        let left_ast: Result<AST<OutputT, V, S>, ()> = match &right_rule.first.lhs {
            E::T(terminal_symbol) => {
                self.eval_terminal_symbol(terminal_symbol, pos.clone(), max_pos)
            }
            E::V(lhs_of_fc_v) => self.eval(pos, rules, &lhs_of_fc_v, max_pos),
        };

        if let Ok(left_ast) = left_ast {
            // right-hand side of first choice
            let right_ast: Result<AST<OutputT, V, S>, ()> = match &right_rule.first.rhs {
                E::T(terminal_symbol) => {
                    self.eval_terminal_symbol(terminal_symbol, pos.clone(), max_pos)
                }
                E::V(rhs_of_fc_v) => self.eval(&left_ast.span.hi(), rules, &rhs_of_fc_v, max_pos),
            };

            if let Ok(right_ast) = right_ast {
                let merged_span = Span::merge_lhs_and_rhs(&left_ast.span, &right_ast.span);

                let variable_and_choice =
                    VAndE::new(variable.clone(), Choice::first(left_ast, right_ast));

                // let output = OutputT::try_from((self, *variable, merged_span.clone(), Choice::first(left_ast, right_ast)));

                let cst = CST::new(variable_and_choice, merged_span);

                let output_ast = OutputT::output_ast(&self, cst);

                return Ok(output_ast);

                // return Ok(AST::from_internal_node(
                //     InternalNode::from_first((*variable, output), left_ast, right_ast),
                //     merged_span,
                // ));
            }
        }

        // Second choice
        match &right_rule.second.0 {
            E::T(terminal_symbol) => {
                self.eval_terminal_symbol(terminal_symbol, pos.clone(), max_pos)
            }
            E::V(sc_v) => {
                let ast = self.eval(pos, rules, &sc_v, max_pos)?;
                let span = ast.span.clone();

                let variable_and_choice = VAndE::new(variable.clone(), Choice::second(ast));

                let cst = CST::new(variable_and_choice, span);

                let output_ast = OutputT::output_ast(self, cst);

                Ok(output_ast)
                // Ok(AST::from_internal_node(
                //     InternalNode::from_second((*variable, output), ast),
                //     span,
                // ))
            }
        }
    }
}
