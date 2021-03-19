#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Metasymbol {
    Epsilon,
    Failed,
    Any,
}

impl Metasymbol {
    pub fn epsilon() -> Self {
        Self::Epsilon
    }

    pub fn failed() -> Self {
        Self::Failed
    }

    pub fn any() -> Self {
        Self::Any
    }
}
