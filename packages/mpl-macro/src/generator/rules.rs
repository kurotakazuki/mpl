use crate::mplg::MplgOutput;
use mpl::symbols::{TerminalSymbol, U8SliceTerminal, E};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

pub fn generate_e<'a>(e: &E<U8SliceTerminal<'a>, &'a str>, variable_ident: &Ident) -> TokenStream {
    match e {
        E::T(t) => match t {
            TerminalSymbol::Metasymbol(m) => {
                let m = format!("{:?}", m);
                let ident = format_ident!("{}", m);
                quote!(#ident)
            }
            TerminalSymbol::Original(o) => match o {
                U8SliceTerminal::Str(s) => {
                    let o = format!("{}", s);
                    quote!({ ::mpl::symbols::U8SliceTerminal::Str(#o) })
                }
                _ => unimplemented!(),
            },
        },
        E::V(v) => {
            let ident = format_ident!("{}", format!("{}", v));
            quote!(#variable_ident::#ident)
        }
    }
}

pub fn generate_rules(
    rules_ident: &Ident,
    variable_ident: &Ident,
    lines: &[MplgOutput],
) -> TokenStream {
    let rules = lines
        .iter()
        .filter(|line| matches!(line, MplgOutput::Rule(_)))
        .map(|line| match line {
            MplgOutput::Rule(rule) => {
                dbg!(rule);
                let variable = format_ident!("{}", rule.value);
                let fl = generate_e(&rule.equal.first.lhs, variable_ident);
                let fr = generate_e(&rule.equal.first.rhs, variable_ident);
                let s = generate_e(&rule.equal.second.0, variable_ident);
                dbg!(&fl);
                quote! {
                    #variable =>{ dbg!("This is", variable, #fl); &::mpl::rules::RightRule {
                        first: ::mpl::choices::First {
                            lhs: ::mpl::e_from!(#fl),
                            rhs: ::mpl::e_from!(#fr),
                        },
                        second: ::mpl::choices::Second(::mpl::e_from!(#s)),
                    } }
                }
            }
            _ => unreachable!(),
        });

    quote! {
        pub struct #rules_ident;

        impl<'a> ::mpl::rules::Rules<::mpl::symbols::U8SliceTerminal<'a>, #variable_ident> for #rules_ident {
            fn get(&self, variable: &#variable_ident) -> Option<&::mpl::rules::RightRule<::mpl::symbols::U8SliceTerminal<'a>, #variable_ident>> {
                eprintln!("Get Variable: {:?}", &variable);
                let a = Some(match variable {
                    #(#rules),*
                });
                eprintln!("{:#?}", a);
                a
            }
        }
    }
}
