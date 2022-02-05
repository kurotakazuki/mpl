//! Parse

use crate::input::Input;
use crate::output::Output;
use crate::position::Position;
use crate::rules::Rules;
use crate::span::Span;
use crate::symbols::{Equivalence, Metasymbol, Terminal, TerminalSymbol, Variable, E};
use crate::trees::{AST, CST};

/// Types that can be parsed.
///
/// I is Input.
/// T is terminal symbols.
/// V is (enum of) Variables.
/// S is Span.
/// P is position.
/// R is Rules.
/// O is output type.
// TODO: Create Error types
pub trait Parser<'i, I, T, V, S, P, R, O = ()>
where
    I: Input + ?Sized,
    T: Terminal<'i, I, V, S, P, O>,
    V: Variable,
    S: Span<I, P>,
    P: Position,
    R: Rules<T, V>,
    O: Output<'i, I, V, S>,
{
    /// Minimal parse.
    ///
    /// # Warning
    ///
    /// `all_of_the_span.hi(self)` must be smaller than its length.
    fn parse(
        &self,
        input: &'i I,
        rules: &R,
        start_variable: &V,
        all_of_the_span: &S,
    ) -> Result<AST<V, S, O>, AST<V, S, O>> {
        let ast = self.eval(
            input,
            &all_of_the_span.lo(input),
            rules,
            start_variable,
            &all_of_the_span.hi(input),
        )?;

        if &ast.span == all_of_the_span {
            Ok(ast)
        } else {
            Err(ast)
        }
    }

    fn to_empty_ast(&self, input: &'i I, pos: P) -> Result<AST<V, S, O>, AST<V, S, O>> {
        Ok(AST::from_leaf(
            Metasymbol::Empty.into(),
            Span::from_lo_hi(pos.clone(), pos, input),
        ))
    }

    fn to_failure_ast(&self, input: &'i I, pos: P) -> Result<AST<V, S, O>, AST<V, S, O>> {
        Err(AST::from_leaf(
            Metasymbol::Failure.into(),
            Span::from_lo_hi(pos.clone(), pos, input),
        ))
    }

    // TODO: Decide return Any or Failure
    fn to_any_ast(
        &self,
        input: &'i I,
        pos: P,
        max_pos: &P,
        n: usize,
    ) -> Result<AST<V, S, O>, AST<V, S, O>> {
        let span_with_len_added = S::from_lo_len(pos, n, input);
        let hi = span_with_len_added.hi(input);
        let ast = AST::from_leaf(Metasymbol::Any(n).into(), span_with_len_added);
        if &hi <= max_pos {
            Ok(ast)
        } else {
            Err(ast)
        }
    }

    fn to_all_ast(&self, input: &'i I, pos: P, max_pos: P) -> Result<AST<V, S, O>, AST<V, S, O>> {
        Ok(AST::from_leaf(
            Metasymbol::All.into(),
            Span::from_lo_hi(pos, max_pos, input),
        ))
    }

    fn eval_terminal_symbol(
        &self,
        input: &'i I,
        terminal_symbol: &TerminalSymbol<T>,
        pos: P,
        max_pos: &P,
    ) -> Result<AST<V, S, O>, AST<V, S, O>> {
        match terminal_symbol {
            TerminalSymbol::Original(t) => t.eval(input, pos, max_pos),
            TerminalSymbol::Metasymbol(metasymbol) => match metasymbol {
                Metasymbol::Empty => self.to_empty_ast(input, pos),
                Metasymbol::Failure => self.to_failure_ast(input, pos),
                Metasymbol::Any(n) => self.to_any_ast(input, pos, max_pos, *n),
                Metasymbol::All => self.to_all_ast(input, pos, max_pos.clone()),
                Metasymbol::Omit => unimplemented!(),
            },
        }
    }

    fn eval(
        &self,
        input: &'i I,
        pos: &P,
        rules: &R,
        variable: &V,
        max_pos: &P,
    ) -> Result<AST<V, S, O>, AST<V, S, O>> {
        let right_rule = rules.get(variable).expect("right_rule from a variable");

        // First choice
        // left-hand side of first choice
        let left_ast: Result<AST<V, S, O>, AST<V, S, O>> = match &right_rule.first.lhs {
            E::T(terminal_symbol) => {
                self.eval_terminal_symbol(input, terminal_symbol, pos.clone(), max_pos)
            }
            E::V(lhs_of_fc_v) => self.eval(input, pos, rules, lhs_of_fc_v, max_pos),
        };

        if let Ok(left_ast) = left_ast {
            // right-hand side of first choice
            let right_ast: Result<AST<V, S, O>, AST<V, S, O>> = match &right_rule.first.rhs {
                E::T(terminal_symbol) => self.eval_terminal_symbol(
                    input,
                    terminal_symbol,
                    left_ast.span.hi(input),
                    max_pos,
                ),
                E::V(rhs_of_fc_v) => {
                    self.eval(input, &left_ast.span.hi(input), rules, rhs_of_fc_v, max_pos)
                }
            };

            if let Ok(right_ast) = right_ast {
                let merged_span = Span::merge_lhs_and_rhs(&left_ast.span, &right_ast.span, input);

                let variable_and_choice =
                    Equivalence::new(variable.clone(), (left_ast, right_ast).into());

                let cst = CST::new(variable_and_choice, merged_span);

                let output_ast = O::output_ast(input, cst);

                return Ok(output_ast);
            }
        }

        // Second choice
        match &right_rule.second.0 {
            E::T(terminal_symbol) => {
                self.eval_terminal_symbol(input, terminal_symbol, pos.clone(), max_pos)
            }
            E::V(sc_v) => {
                let ast = self.eval(input, pos, rules, sc_v, max_pos)?;
                let span = ast.span.clone();

                let variable_and_choice = Equivalence::new(variable.clone(), ast.into());

                let cst = CST::new(variable_and_choice, span);

                let output_ast = O::output_ast(input, cst);

                Ok(output_ast)
            }
        }
    }
}
