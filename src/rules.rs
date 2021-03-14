// use crate::symbols::E;

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Choice<E> {
    First(E, E),
    Second(E),
}

impl<E> Choice<E> {
    pub fn first(left: E, right: E) -> Self {
        Self::First(left, right)
    }

    pub fn second(e: E) -> Self {
        Self::Second(e)
    }

    /// Returns true if Self::First
    pub fn is_first(&self) -> bool {
        match self {
            Self::First(_, _) => true,
            Self::Second(_) => false,
        }
    }

    /// Returns true if Self::Second
    pub fn is_second(&self) -> bool {
        match self {
            Self::First(_, _) => false,
            Self::Second(_) => true,
        }
    }
}

// #[derive(Clone, Debug, PartialEq)]
// pub struct RightRule<T, V> {
//     first: (E<T, V>, E<T, V>),
//     second: E<T, V>,
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number() {
        #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
        enum DigitTerminal {
            Zero,
            One,
            Two,
            Three,
            Four,
            Five,
            Six,
            Seven,
            Eight,
            Nine,
        }
    }
}