use crate::mplg::MplgOutput;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

pub fn generate_variable(ident: &Ident, lines: &[MplgOutput]) -> TokenStream {
    let variables = lines
        .iter()
        .filter(|line| matches!(line, MplgOutput::Rule(_)))
        .map(|line| match line {
            MplgOutput::Rule(rule) => {
                let variable = format_ident!("{}", rule.value);
                quote!(#variable)
            }
            _ => unreachable!(),
        });

    quote! {
        impl ::mpl::symbols::Variable for #ident {}

        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        pub enum #ident {
            #(#variables),*
        }
    }
}
