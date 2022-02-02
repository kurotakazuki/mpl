mod mplg;

use crate::mplg::parse_mplg;
use proc_macro::TokenStream;
use quote::quote;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use syn::{parse_macro_input, Attribute, DeriveInput, Generics, Ident, Lit, Meta};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum GrammarData {
    Mplg(Vec<u8>),
    None,
}

#[proc_macro_derive(Parser, attributes(grammar))]
pub fn derive_parser(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (parser_ident, generics, grammar_data) = parse_derive(input);

    match grammar_data {
        Ok(grammar_data) => {
            match grammar_data {
                // mplg = \"...\"
                GrammarData::Mplg(data) => {
                    let variables = variables(&data);
                    todo!()
                }
                GrammarData::None => TokenStream::new(),
            }
        }
        Err(e) => e.into(),
    }
}

fn variables(data: &[u8]) -> proc_macro2::TokenStream {
    let ast = parse_mplg(&data).unwrap();

    let variables_enum = variables_enum();
    let variables_impl = variables_impl();

    quote! {
        #variables_enum
        #variables_impl
    }
}

fn variables_enum() -> proc_macro2::TokenStream {
    todo!()
}

fn variables_impl() -> proc_macro2::TokenStream {
    todo!()
}

fn parse_derive(
    input: DeriveInput,
) -> (
    Ident,
    Generics,
    Result<GrammarData, proc_macro2::TokenStream>,
) {
    let parser_ident = input.ident;
    let generics = input.generics;

    let attr = match input.attrs.len() {
        0 => Ok(GrammarData::None),
        1 => get_grammar_data(&input.attrs[0]),
        _ => unimplemented!(),
    };

    (parser_ident, generics, attr)
}

fn get_grammar_data(attr: &Attribute) -> Result<GrammarData, proc_macro2::TokenStream> {
    if let Ok(Meta::NameValue(name_value)) = attr.parse_meta() {
        match name_value.lit {
            Lit::Str(lit_str) => {
                if name_value.path.is_ident("mplg") {
                    let attr = lit_str.value();
                    let root = env!("CARGO_MANIFEST_DIR");
                    let full_path = Path::new(&root).join(&attr);
                    let data = match read_file(&full_path) {
                        Ok(data) => data,
                        Err(e) => panic!("{} ({:#?})", e, full_path),
                    };

                    return Ok(GrammarData::Mplg(data));
                } else {
                    // unimplemented!()
                }
            }
            _ => {
                // unimplemented!()
            }
        }
    }

    Err(syn::Error::new_spanned(attr, "expected `mplg = \"...\"`").to_compile_error())
}

fn read_file<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut v = Vec::new();
    file.read_to_end(&mut v)?;
    Ok(v)
}
