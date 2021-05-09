#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Metasymbol {
    Epsilon,
    Failure,
    Any(usize),
    All,
    Omit,
}
