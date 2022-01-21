use crate::symbols::{Metasymbol, TerminalSymbol};

/// Terminal symbol or Variable.
///
/// { e | e &isin; E, E = T &cup; V }
#[derive(Clone, Debug, PartialEq)]
pub enum E<T, V> {
    T(TerminalSymbol<T>),
    V(V),
}

impl<T, V> From<TerminalSymbol<T>> for E<T, V> {
    fn from(t: TerminalSymbol<T>) -> Self {
        Self::T(t)
    }
}

impl<T, V> From<Metasymbol> for E<T, V> {
    fn from(m: Metasymbol) -> Self {
        Self::T(m.into())
    }
}

impl<T, V> E<T, V> {
    pub fn from_v(v: V) -> Self {
        Self::V(v)
    }
}
