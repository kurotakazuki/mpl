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

#[macro_export]
macro_rules! e_from {
    (f) => {
        $crate::symbols::E::T($crate::symbols::TerminalSymbol::Metasymbol(
            $crate::symbols::Metasymbol::Failure,
        ))
    };
    (()) => {
        $crate::symbols::E::T($crate::symbols::TerminalSymbol::Metasymbol(
            $crate::symbols::Metasymbol::Empty,
        ))
    };
    (*) => {
        $crate::symbols::E::T($crate::symbols::TerminalSymbol::Metasymbol(
            $crate::symbols::Metasymbol::All,
        ))
    };
    (?) => {
        $crate::symbols::E::T($crate::symbols::TerminalSymbol::Metasymbol(
            $crate::symbols::Metasymbol::Any(1),
        ))
    };
    (Failure) => {
        $crate::symbols::E::T($crate::symbols::TerminalSymbol::Metasymbol(
            $crate::symbols::Metasymbol::Failure,
        ))
    };
    (Empty) => {
        $crate::symbols::E::T($crate::symbols::TerminalSymbol::Metasymbol(
            $crate::symbols::Metasymbol::Empty,
        ))
    };
    (All) => {
        $crate::symbols::E::T($crate::symbols::TerminalSymbol::Metasymbol(
            $crate::symbols::Metasymbol::All,
        ))
    };
    (Any($len:literal)) => {
        $crate::symbols::E::T($crate::symbols::TerminalSymbol::Metasymbol(
            $crate::symbols::Metasymbol::Any($len),
        ))
    };
    ($v:path) => {
        $crate::symbols::E::V($v)
    };
    ($o:block) => {
        $crate::symbols::E::T($crate::symbols::TerminalSymbol::Original($o))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbols::{Metasymbol::*, U8SliceTerminal, U8SliceTerminal::*, E};

    #[test]
    fn e_from() {
        #[derive(Debug, Eq, PartialEq)]
        enum TestVariables {
            Test,
        }
        use TestVariables::Test;
        assert_eq!(
            e_from!(f),
            E::<U8SliceTerminal, TestVariables>::T(TerminalSymbol::Metasymbol(Failure))
        );
        assert_eq!(
            e_from!(()),
            E::<U8SliceTerminal, TestVariables>::T(TerminalSymbol::Metasymbol(Empty))
        );
        assert_eq!(
            e_from!(*),
            E::<U8SliceTerminal, TestVariables>::T(TerminalSymbol::Metasymbol(All))
        );
        assert_eq!(
            e_from!(?),
            E::<U8SliceTerminal, TestVariables>::T(TerminalSymbol::Metasymbol(Any(1)))
        );
        assert_eq!(
            e_from!(Any(1)),
            E::<U8SliceTerminal, TestVariables>::T(TerminalSymbol::Metasymbol(Any(1)))
        );
        assert_eq!(
            e_from!(Any(3)),
            E::<U8SliceTerminal, TestVariables>::T(TerminalSymbol::Metasymbol(Any(3)))
        );
        // Variable
        assert_eq!(
            e_from!(Test),
            E::<U8SliceTerminal, TestVariables>::V(TestVariables::Test)
        );
        assert_eq!(
            e_from!(TestVariables::Test),
            E::<U8SliceTerminal, TestVariables>::V(TestVariables::Test)
        );
        // Original
        assert_eq!(
            e_from!({ U8SliceTerminal::Str("hello") }),
            E::<U8SliceTerminal, TestVariables>::T(TerminalSymbol::Original(U8SliceTerminal::Str(
                "hello"
            )))
        );
        assert_eq!(
            e_from!({ Str("hello") }),
            E::<U8SliceTerminal, TestVariables>::T(TerminalSymbol::Original(U8SliceTerminal::Str(
                "hello"
            )))
        );
        assert_eq!(
            e_from!({ "hello" }),
            E::<&str, TestVariables>::T(TerminalSymbol::Original("hello"))
        );
    }
}
