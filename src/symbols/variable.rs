use std::hash::Hash;

pub trait Variable: Clone + Eq + Hash {}
