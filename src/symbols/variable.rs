use std::hash::Hash;

pub trait Variable: Clone + Eq + Hash {}

#[derive(Clone, Debug, PartialEq)]
pub struct Equivalence<V, E> {
    pub value: V,
    pub equal: E,
}

impl<V, E> Equivalence<V, E> {
    pub fn new(value: V, equal: E) -> Self {
        Self { value, equal }
    }
}
