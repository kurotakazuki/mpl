//! Input

mod slice;
mod str;

/// Language's input types.
///
/// # Examples
///
/// ```
/// use mpl::input::Input;
///
/// struct ExtStr(String);
///
/// impl Input for ExtStr {}
/// ```
pub trait Input {}
