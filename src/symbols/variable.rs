#[derive(Clone, Debug, PartialEq)]
pub struct Variable<V, E> {
    value: V,
    equal: E,
}

impl<V, E> Variable<V, E> {
    pub fn new(value: V, equal: E) -> Self {
        Self { value, equal }
    }
}
