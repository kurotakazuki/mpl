mod generator;
mod mplg;

use proc_macro::TokenStream;

/// This macro creates
/// let ident = parser_ident.replace("Parser", "");
/// `{ident}Variable` enum,
/// `{ident}Rules` const {variable_i_ident}_RULE for each rule,
/// and impl Parser for `{parser_ident}`.
///
/// # Examples
///
/// ``` ignore
/// use mpl_macro::Parse;
///
/// #[derive(Parse)]
/// #[mplg = "{your path}/my.mplg"]
/// pub struct MyParser;
/// ```
#[proc_macro_derive(Parse, attributes(mplg))]
pub fn derive_parse(input: TokenStream) -> TokenStream {
    generator::derive_parser(input.into()).into()
}
