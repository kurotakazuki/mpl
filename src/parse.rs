use crate::input::Input;
use crate::output::Output;
use crate::position::Position;
use crate::rules::Rules;
use crate::span::{Len, Span, Start, StartAndLenSpan};
use crate::symbols::{
    Equivalence, Metasymbol, SliceTerminal, StrTerminal, Terminal, TerminalSymbol, U8SliceTerminal,
    Variable, E,
};
use crate::tree::{AST, CST};

/// Types that can be parsed.
///
/// T is terminal symbols.
/// O is output type.
/// V is (enum of) Variables.
/// S is Span.
/// P is position.
// TODO: Create Error types
pub trait Parse<'input, T, V, S, P, R, O = ()>: Input
where
    T: Terminal<'input, Self, V, S, P, O>,
    V: Variable,
    S: Span<Self, P>,
    P: Position,
    R: Rules<T, V>,
    O: Output<'input, Self, V, S>,
{
    // all_of_the_span.unwarp().hi() < input.len()
    fn minimal_parse(
        &'input self,
        rules: &R,
        start_variable: &V,
        all_of_the_span: &S,
    ) -> Result<AST<V, S, O>, AST<V, S, O>> {
        let ast = self.eval(
            &all_of_the_span.lo(self),
            rules,
            start_variable,
            &all_of_the_span.hi(self),
        )?;

        if &ast.span == all_of_the_span {
            Ok(ast)
        } else {
            Err(ast)
        }
    }

    fn to_epsilon_ast(&'input self, pos: P) -> Result<AST<V, S, O>, AST<V, S, O>> {
        Ok(AST::from_leaf_node(
            Metasymbol::Empty.into(),
            Span::from_lo_hi(pos.clone(), pos, self),
        ))
    }

    fn to_failed_ast(&'input self, pos: P) -> Result<AST<V, S, O>, AST<V, S, O>> {
        Err(AST::from_leaf_node(
            Metasymbol::Failure.into(),
            Span::from_lo_hi(pos.clone(), pos, self),
        ))
    }

    // TODO: Decide return Any or Failure
    fn to_any_ast(
        &'input self,
        pos: P,
        max_pos: &P,
        n: usize,
    ) -> Result<AST<V, S, O>, AST<V, S, O>> {
        let span_with_len_added = S::from_lo_len(pos, n, self);
        let hi = span_with_len_added.hi(self);
        let ast = AST::from_leaf_node(Metasymbol::Any(n).into(), span_with_len_added);
        if &hi <= max_pos {
            Ok(ast)
        } else {
            Err(ast)
        }
    }

    fn to_all_ast(&'input self, pos: P, max_pos: P) -> Result<AST<V, S, O>, AST<V, S, O>> {
        Ok(AST::from_leaf_node(
            Metasymbol::All.into(),
            Span::from_lo_hi(pos, max_pos, self),
        ))
    }

    fn eval_terminal_symbol(
        &'input self,
        terminal_symbol: &TerminalSymbol<T>,
        pos: P,
        max_pos: &P,
    ) -> Result<AST<V, S, O>, AST<V, S, O>> {
        match terminal_symbol {
            TerminalSymbol::Original(t) => t.eval(self, pos, max_pos),
            TerminalSymbol::Metasymbol(metasymbol) => match metasymbol {
                Metasymbol::Empty => self.to_epsilon_ast(pos),
                Metasymbol::Failure => self.to_failed_ast(pos),
                Metasymbol::Any(n) => self.to_any_ast(pos, max_pos, *n),
                Metasymbol::All => self.to_all_ast(pos, max_pos.clone()),
                Metasymbol::Omit => unimplemented!(),
            },
        }
    }

    fn eval(
        &'input self,
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
                self.eval_terminal_symbol(terminal_symbol, pos.clone(), max_pos)
            }
            E::V(lhs_of_fc_v) => self.eval(pos, rules, lhs_of_fc_v, max_pos),
        };

        if let Ok(left_ast) = left_ast {
            // right-hand side of first choice
            let right_ast: Result<AST<V, S, O>, AST<V, S, O>> = match &right_rule.first.rhs {
                E::T(terminal_symbol) => {
                    self.eval_terminal_symbol(terminal_symbol, left_ast.span.hi(self), max_pos)
                }
                E::V(rhs_of_fc_v) => {
                    self.eval(&left_ast.span.hi(self), rules, rhs_of_fc_v, max_pos)
                }
            };

            if let Ok(right_ast) = right_ast {
                let merged_span = Span::merge_lhs_and_rhs(&left_ast.span, &right_ast.span, self);

                let variable_and_choice =
                    Equivalence::new(variable.clone(), (left_ast, right_ast).into());

                let cst = CST::new(variable_and_choice, merged_span);

                let output_ast = O::output_ast(self, cst);

                return Ok(output_ast);
            }
        }

        // Second choice
        match &right_rule.second.0 {
            E::T(terminal_symbol) => {
                self.eval_terminal_symbol(terminal_symbol, pos.clone(), max_pos)
            }
            E::V(sc_v) => {
                let ast = self.eval(pos, rules, sc_v, max_pos)?;
                let span = ast.span.clone();

                let variable_and_choice = Equivalence::new(variable.clone(), ast.into());

                let cst = CST::new(variable_and_choice, span);

                let output_ast = O::output_ast(self, cst);

                Ok(output_ast)
            }
        }
    }
}

/// T represents the element type.
impl<'input, T, V, P, L, R, O>
    Parse<'input, SliceTerminal<'input, T>, V, StartAndLenSpan<P, L>, P, R, O> for [T]
where
    T: PartialEq,
    V: Variable,
    P: Start<Self, L>,
    L: Len<Self, P>,
    R: Rules<SliceTerminal<'input, T>, V>,
    O: Output<'input, Self, V, StartAndLenSpan<P, L>>,
{
}

impl<'input, V, P, L, R, O>
    Parse<'input, U8SliceTerminal<'input>, V, StartAndLenSpan<P, L>, P, R, O> for [u8]
where
    V: Variable,
    P: Start<Self, L>,
    L: Len<Self, P>,
    R: Rules<U8SliceTerminal<'input>, V>,
    O: Output<'input, Self, V, StartAndLenSpan<P, L>>,
{
}

impl<'input, V, P, L, R, O> Parse<'input, StrTerminal<'input>, V, StartAndLenSpan<P, L>, P, R, O>
    for str
where
    V: Variable,
    P: Start<Self, L>,
    L: Len<Self, P>,
    R: Rules<StrTerminal<'input>, V>,
    O: Output<'input, Self, V, StartAndLenSpan<P, L>>,
{
}
