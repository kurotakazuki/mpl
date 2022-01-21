//! Symbols

pub use self::e::E;
pub use self::terminal::{
    metasymbol::Metasymbol, slice_terminal::SliceTerminal, str_terminal::StrTerminal,
    terminal_symbol::TerminalSymbol, u8slice_terminal::U8SliceTerminal, Terminal,
};
pub use self::variable::Variable;

mod e;
mod terminal;
mod variable;

#[derive(Clone, Debug, PartialEq)]
pub struct Equivalence<V, E> {
    pub value: V,
    pub equal: E,
}

impl<V, E> Equivalence<V, E> {
    pub fn new(value: V, equal: E) -> Self {
        Self { value, equal }
    }
}
