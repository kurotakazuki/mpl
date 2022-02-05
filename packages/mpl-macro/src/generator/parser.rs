use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn generate_parser(
    parser_ident: &Ident,
    rules_ident: &Ident,
    variable_ident: &Ident,
) -> TokenStream {
    quote! {
        impl<'i, P, L, O>
            ::mpl::parser::Parser<'i, [u8], ::mpl::symbols::U8SliceTerminal<'i>, #variable_ident, ::mpl::span::StartAndLenSpan<P, L>, P, #rules_ident, O> for #parser_ident
        where
            P: ::mpl::span::Start<[u8], L>,
            L: ::mpl::span::Len<[u8], P>,
            O: ::mpl::output::Output<'i, [u8], #variable_ident, ::mpl::span::StartAndLenSpan<P, L>>,
        {
        }
    }
}
