use crate::mplg::MplgOutput;
use mpl::symbols::{Metasymbol, TerminalSymbol, E};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::str::FromStr;
use syn::Ident;

pub fn generate_e<'a>(e: &E<&'a str, &'a str>, variable_ident: &Ident) -> TokenStream {
    match e {
        E::T(t) => match t {
            TerminalSymbol::Metasymbol(m) => match m {
                Metasymbol::Failure => quote! {
                    ::mpl::symbols::E::<::mpl::symbols::U8SliceTerminal, #variable_ident>::T(::mpl::symbols::TerminalSymbol::Metasymbol(
                        ::mpl::symbols::Metasymbol::Failure
                    ))
                },
                Metasymbol::Empty => quote! {
                    ::mpl::symbols::E::<::mpl::symbols::U8SliceTerminal, #variable_ident>::T(::mpl::symbols::TerminalSymbol::Metasymbol(
                        ::mpl::symbols::Metasymbol::Empty
                    ))
                },
                Metasymbol::All => quote! {
                    ::mpl::symbols::E::<::mpl::symbols::U8SliceTerminal, #variable_ident>::T(::mpl::symbols::TerminalSymbol::Metasymbol(
                        ::mpl::symbols::Metasymbol::All
                    ))
                },
                Metasymbol::Any(n) => quote! {
                    ::mpl::symbols::E::<::mpl::symbols::U8SliceTerminal, #variable_ident>::T(::mpl::symbols::TerminalSymbol::Metasymbol(
                        ::mpl::symbols::Metasymbol::Any(#n)
                    ))
                },
                _ => unreachable!(),
            },
            TerminalSymbol::Original(o) => {
                // eprintln!("fadsj;lkkalcmondai{}", o);
                // let o = format_ident!("{}", o);
                let o = TokenStream::from_str(o).unwrap();
                quote! {
                    ::mpl::symbols::E::<::mpl::symbols::U8SliceTerminal, #variable_ident>::T(::mpl::symbols::TerminalSymbol::Original(
                        ::mpl::symbols::U8SliceTerminal::#o
                    ))
                }
            }
        },
        E::V(v) => {
            let ident = format_ident!("{}", v);
            quote! {
                ::mpl::symbols::E::<::mpl::symbols::U8SliceTerminal, #variable_ident>::V(#variable_ident::#ident)
            }
        }
    }
}

pub fn generate_rules(
    rules_ident: &Ident,
    variable_ident: &Ident,
    lines: &[MplgOutput],
) -> TokenStream {
    let const_rules = lines
        .iter()
        .filter(|line| matches!(line, &&MplgOutput::Rule(_)))
        .map(|line| match line {
            MplgOutput::Rule(rule) => {
                let variable = format_ident!("{}", rule.value);
                let const_ident = format_ident!("{}_RULE", variable);
                let fl = generate_e(&rule.equal.first.lhs, variable_ident);
                let fr = generate_e(&rule.equal.first.rhs, variable_ident);
                let s = generate_e(&rule.equal.second.0, variable_ident);

                quote! {
                    const #const_ident: ::mpl::rules::RightRule<::mpl::symbols::U8SliceTerminal<'a>, #variable_ident> = ::mpl::rules::RightRule {
                        first: ::mpl::choices::First {
                            lhs: #fl,
                            rhs: #fr,
                        },
                        second: ::mpl::choices::Second(#s),
                    };
                }
            }
            _ => unreachable!(),
        });

    // TODO BUG
    // let match_rules = lines
    //     .iter()
    //     .filter(|line| matches!(line, &&MplgOutput::Rule(_)))
    //     .map(|line| match line {
    //         MplgOutput::Rule(rule) => {
    //             dbg!(&rule);
    //             let variable = format_ident!("{}", rule.value);
    //             let const_ident = format_ident!("{}_RULE", variable);
    //             quote! {
    //                 #variable => {
    //                     &Self::#const_ident
    //                 }
    //             }
    //         }
    //         _ => unreachable!(),
    //     });

    quote! {
        pub struct #rules_ident;

        impl<'a> #rules_ident {
            #(#const_rules)*
        }

        // TODO BUG
        // impl<'a> ::mpl::rules::Rules<::mpl::symbols::U8SliceTerminal<'a>, #variable_ident> for #rules_ident {
        //     fn get(&self, variable: &#variable_ident) -> Option<&::mpl::rules::RightRule<::mpl::symbols::U8SliceTerminal<'a>, #variable_ident>> {
        //         eprintln!("Get Variable: {:?}", variable);
        //         let a = Some(match variable {
        //             #(#match_rules)*
        //         });
        //         eprintln!("{:#?}", a);
        //         a
        //     }
        // }
    }
}
