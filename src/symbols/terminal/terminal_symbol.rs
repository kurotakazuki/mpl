use crate::symbols::Metasymbol;

#[derive(Clone, Debug, PartialEq)]
pub enum TerminalSymbol<T> {
    Original(T),
    Metasymbol(Metasymbol),
}

impl<T> From<Metasymbol> for TerminalSymbol<T> {
    fn from(m: Metasymbol) -> Self {
        Self::Metasymbol(m)
    }
}

impl<T> TerminalSymbol<T> {
    pub fn from_original(t: T) -> Self {
        Self::Original(t)
    }
}
