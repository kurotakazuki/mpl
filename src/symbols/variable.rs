#[derive(Clone, Debug, PartialEq)]
pub struct Variable<V, E> {
    pub value: V,
    pub equal: E,
}

impl<V, E> Variable<V, E> {
    pub fn new(value: V, equal: E) -> Self {
        Self { value, equal }
    }
}
