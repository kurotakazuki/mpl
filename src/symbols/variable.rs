use std::hash::Hash;

pub trait Variable: Copy + Eq + Hash {}

#[derive(Clone, Debug, PartialEq)]
pub struct VAndE<V, E> {
    pub value: V,
    pub equal: E,
}

impl<V, E> VAndE<V, E> {
    pub fn new(value: V, equal: E) -> Self {
        Self { value, equal }
    }
}
