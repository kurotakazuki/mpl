use crate::symbols::Metasymbol;

#[derive(Clone, Debug, PartialEq)]
pub enum TerminalSymbol<T> {
    Original(T),
    M(Metasymbol),
}

// impl<T> From<Metasymbol> for TerminalSymbol<T> {
//     fn from(metasymbol: Metasymbol) -> Self {
//         Self::M(metasymbol)
//     }
// }

// impl<T> From<T> for TerminalSymbol<T> {
//     fn from(t: T) -> Self {
//         Self::Original(t)
//     }
// }

impl<T> TerminalSymbol<T> {
    pub fn from_t(t: T) -> Self {
        Self::Original(t)
    }

    pub fn from_m(metasymbol: Metasymbol) -> Self {
        Self::M(metasymbol)
    }
}
