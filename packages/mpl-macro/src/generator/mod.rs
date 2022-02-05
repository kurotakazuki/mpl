use crate::mplg::parse_mplg;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use syn::{parse2, Attribute, DeriveInput, Generics, Ident, Lit, Meta};

pub use self::rules::generate_rules;
pub use self::variable::generate_variable;

mod rules;
mod variable;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum GrammarData {
    Mplg(Vec<u8>),
    None,
}

pub fn derive_parser(input: TokenStream) -> TokenStream {
    let input = parse2(input).unwrap();
    let (parser_ident, _generics, grammar_data) = parse_derive(input);
    let ident = parser_ident.to_string().replace("Parser", "");
    let rules_ident = &format_ident!("{}Rules", ident);
    let variable_ident = &format_ident!("{}Variables", ident);

    match grammar_data {
        Ok(grammar_data) => {
            match grammar_data {
                // mplg = \"...\"
                GrammarData::Mplg(data) => {
                    let lines = parse_mplg(&data)
                        .unwrap()
                        .into_original()
                        .expect("Lines")
                        .to_lines();
                    let rules = generate_rules(rules_ident, variable_ident, &lines);
                    let variable = generate_variable(variable_ident, &lines);

                    quote! {
                        #variable
                        #rules
                    }
                }
                GrammarData::None => TokenStream::new(),
            }
        }
        Err(e) => e,
    }
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
