/// Variable types.
///
/// # Examples
///
/// ```
/// use mpl::symbols::Variable;
///
/// #[derive(Clone, Debug, Hash, Eq, PartialEq)]
/// enum ParenthesesVariable {
///     Open,
///     Parentheses,
///     Close,
/// }
///
/// impl Variable for ParenthesesVariable {}
/// ```
pub trait Variable: Clone {}
