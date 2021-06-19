use crate::tree::AST;

#[derive(Clone, Debug, PartialEq)]
pub struct First<E> {
    pub lhs: E,
    pub rhs: E,
}

impl<E> First<E> {
    pub fn new(lhs: E, rhs: E) -> Self {
        Self { lhs, rhs }
    }
}

impl<O, V, S> From<(AST<O, V, S>, AST<O, V, S>)> for First<AST<O, V, S>> {
    fn from(e: (AST<O, V, S>, AST<O, V, S>)) -> Self {
        Self::new(e.0, e.1)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Second<E>(pub E);

impl<O, V, S> From<AST<O, V, S>> for Second<AST<O, V, S>> {
    fn from(e: AST<O, V, S>) -> Self {
        Self::new(e)
    }
}

impl<E> Second<E> {
    pub fn new(e: E) -> Self {
        Self(e)
    }
}
#[derive(Clone, Debug, PartialEq)]
pub enum Choice<E> {
    First(First<E>),
    Second(Second<E>),
}

impl<E> From<First<E>> for Choice<E> {
    fn from(first: First<E>) -> Self {
        Self::First(first)
    }
}

impl<E> From<Second<E>> for Choice<E> {
    fn from(second: Second<E>) -> Self {
        Self::Second(second)
    }
}

impl<O, V, S> From<(AST<O, V, S>, AST<O, V, S>)> for Choice<AST<O, V, S>> {
    fn from(e: (AST<O, V, S>, AST<O, V, S>)) -> Self {
        First::from(e).into()
    }
}

impl<O, V, S> From<AST<O, V, S>> for Choice<AST<O, V, S>> {
    fn from(e: AST<O, V, S>) -> Self {
        Second::from(e).into()
    }
}

impl<E> Choice<E> {
    /// Returns true if Self::First
    pub fn is_first(&self) -> bool {
        match self {
            Self::First(_) => true,
            Self::Second(_) => false,
        }
    }

    /// Returns true if Self::Second
    pub fn is_second(&self) -> bool {
        match self {
            Self::First(_) => false,
            Self::Second(_) => true,
        }
    }

    pub fn as_first(&self) -> Option<&First<E>> {
        match self {
            Self::First(first) => Some(first),
            Self::Second(_) => None,
        }
    }

    pub fn as_second(&self) -> Option<&Second<E>> {
        match self {
            Self::First(_) => None,
            Self::Second(second) => Some(second),
        }
    }

    pub fn into_first(self) -> Option<First<E>> {
        match self {
            Self::First(first) => Some(first),
            Self::Second(_) => None,
        }
    }

    pub fn into_second(self) -> Option<Second<E>> {
        match self {
            Self::First(_) => None,
            Self::Second(second) => Some(second),
        }
    }
}
