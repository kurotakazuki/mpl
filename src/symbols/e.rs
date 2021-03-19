use crate::symbols::TerminalSymbol;

#[derive(Clone, Debug, PartialEq)]
pub enum E<T, V> {
    T(TerminalSymbol<T>),
    V(V),
}

impl<T, V> E<T, V> {
    pub fn t(t: TerminalSymbol<T>) -> Self {
        Self::T(t)
    }

    pub fn v(v: V) -> Self {
        Self::V(v)
    }
}