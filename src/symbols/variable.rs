/// Variable types.
/// 
/// ```
/// #[derive(Debug)]
/// enum Paren {
///     Open,
///     Close,
/// }
/// 
/// impl Variable for Paren {}
/// ```
pub trait Variable: Clone {}
