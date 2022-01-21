use proc_macro::TokenStream;

#[proc_macro_derive(Builder)]
pub fn derive_parser(input: TokenStream) -> TokenStream {
    let _ = input;
    unimplemented!()
}
