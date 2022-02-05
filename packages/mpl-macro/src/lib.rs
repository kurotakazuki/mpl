mod generator;
mod mplg;

use proc_macro::TokenStream;

#[proc_macro_derive(Parser, attributes(mplg))]
pub fn derive_parser(input: TokenStream) -> TokenStream {
    generator::derive_parser(input.into()).into()
}
