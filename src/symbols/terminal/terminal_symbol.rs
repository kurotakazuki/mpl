use crate::symbols::Metasymbol;

/// Terminal symbol.
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

impl<T> TerminalSymbol<T> {
    pub fn as_original(&self) -> Option<&T> {
        match self {
            Self::Original(original) => Some(original),
            Self::Metasymbol(_) => None,
        }
    }

    pub fn as_metasymbol(&self) -> Option<&Metasymbol> {
        match self {
            Self::Original(_) => None,
            Self::Metasymbol(metasymbol) => Some(metasymbol),
        }
    }

    pub fn into_original(self) -> Option<T> {
        match self {
            Self::Original(original) => Some(original),
            Self::Metasymbol(_) => None,
        }
    }

    pub fn into_metasymbol(self) -> Option<Metasymbol> {
        match self {
            Self::Original(_) => None,
            Self::Metasymbol(metasymbol) => Some(metasymbol),
        }
    }
}
