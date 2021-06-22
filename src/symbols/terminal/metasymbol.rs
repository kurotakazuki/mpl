#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Metasymbol {
    Empty,
    Failure,
    Any(usize),
    All,
    Omit,
}
