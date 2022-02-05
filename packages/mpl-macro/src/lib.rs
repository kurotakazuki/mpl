mod generator;
mod mplg;

use proc_macro::TokenStream;

#[proc_macro_derive(Parse, attributes(mplg))]
pub fn derive_parse(input: TokenStream) -> TokenStream {
    generator::derive_parser(input.into()).into()
}
