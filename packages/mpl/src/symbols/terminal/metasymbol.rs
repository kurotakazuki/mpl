/// Metasymbol.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Metasymbol {
    /// `()` is a metasymbol that always succeeds without consuming input.
    ///
    /// `Empty = () () / ()`
    Empty,
    /// `f` is a metasymbol that always fails without consuming input.
    ///
    /// `Failure = f f / f`
    Failure,
    /// `?` is a metasymbol representing any single input like wildcard character. This succeeds if there is any input left, and fails if there is no input left.
    ///
    /// `Any = ? () / f`
    Any(usize),
    /// `*` is a metasymbol representing All remaining input like wildcard character. This will succeed even if the remaining inputs are zero.
    ///
    /// `All = * () / f`
    ///
    /// Same as `All = ? All / ()`.
    All,
    /// `Omit` means that some information has been omitted.
    Omit,
}
