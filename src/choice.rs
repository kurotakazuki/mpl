use crate::tree::AST;

#[derive(Clone, Debug, PartialEq)]
pub struct First<E> {
    pub lhs: E,
    pub rhs: E,
}

impl<O, V, S> From<(AST<O, V, S>, AST<O, V, S>)> for First<AST<O, V, S>> {
    fn from(e: (AST<O, V, S>, AST<O, V, S>)) -> Self {
        let (lhs, rhs) = e;
        Self { lhs, rhs }
    }
}

impl<E> First<E> {
    pub fn new(lhs: E, rhs: E) -> Self {
        Self { lhs, rhs }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Second<E>(pub E);

impl<O, V, S> From<AST<O, V, S>> for Second<AST<O, V, S>> {
    fn from(e: AST<O, V, S>) -> Self {
        Self(e)
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

impl<O, V, S> From<(AST<O, V, S>, AST<O, V, S>)> for Choice<AST<O, V, S>> {
    fn from(e: (AST<O, V, S>, AST<O, V, S>)) -> Self {
        e.into()
    }
}

impl<E> From<Second<E>> for Choice<E> {
    fn from(second: Second<E>) -> Self {
        Self::Second(second)
    }
}

impl<O, V, S> From<AST<O, V, S>> for Choice<AST<O, V, S>> {
    fn from(e: AST<O, V, S>) -> Self {
        e.into()
    }
}

impl<E> Choice<E> {
    // pub fn first(lhs: E, rhs: E) -> Self {
    //     Self::First(First::new(lhs, rhs))
    // }

    // pub fn second(e: E) -> Self {
    //     Self::Second(Second::new(e))
    // }

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
}
