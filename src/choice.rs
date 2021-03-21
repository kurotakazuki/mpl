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

#[derive(Clone, Debug, PartialEq)]
pub struct Second<E>(pub E);

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

impl<E> Choice<E> {
    pub fn first(lhs: E, rhs: E) -> Self {
        Self::First(First::new(lhs, rhs))
    }

    pub fn second(e: E) -> Self {
        Self::Second(Second::new(e))
    }

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
