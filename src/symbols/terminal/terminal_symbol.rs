use crate::symbols::Metasymbol;

#[derive(Clone, Debug, PartialEq)]
pub enum TerminalSymbol<T> {
    Original(T),
    M(Metasymbol),
}

impl<T> TerminalSymbol<T> {
    pub fn from_t(t: T) -> Self {
        Self::Original(t)
    }

    pub fn from_m(metasymbol: Metasymbol) -> Self {
        Self::M(metasymbol)
    }
}
