//! Choices

use crate::trees::AST;

/// First choice of right rule.
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

impl<V, S, O> From<(AST<V, S, O>, AST<V, S, O>)> for First<AST<V, S, O>> {
    fn from(e: (AST<V, S, O>, AST<V, S, O>)) -> Self {
        Self::new(e.0, e.1)
    }
}

/// Second choice of right rule.
#[derive(Clone, Debug, PartialEq)]
pub struct Second<E>(pub E);

impl<V, S, O> From<AST<V, S, O>> for Second<AST<V, S, O>> {
    fn from(e: AST<V, S, O>) -> Self {
        Self::new(e)
    }
}

impl<E> Second<E> {
    pub fn new(e: E) -> Self {
        Self(e)
    }
}

/// `Choice` is either `First` or `Second`.
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

impl<V, S, O> From<(AST<V, S, O>, AST<V, S, O>)> for Choice<AST<V, S, O>> {
    fn from(e: (AST<V, S, O>, AST<V, S, O>)) -> Self {
        First::from(e).into()
    }
}

impl<V, S, O> From<AST<V, S, O>> for Choice<AST<V, S, O>> {
    fn from(e: AST<V, S, O>) -> Self {
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
