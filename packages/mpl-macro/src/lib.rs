use proc_macro::TokenStream;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use syn::{parse_macro_input, Attribute, DeriveInput, Generics, Ident, Lit, Meta};

#[proc_macro_derive(Parser, attributes(grammar))]
pub fn derive_parser(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (parser_ident, generics, attr) = parse_derive(input);

    match attr {
        Ok(attr) => {
            let data = match attr {
                // grammar = \"...\"
                Some(attr) => {
                    let root = env!("CARGO_MANIFEST_DIR");
                    let full_path = Path::new(&root).join(&attr);
                    match read_file(&full_path) {
                        Ok(data) => data,
                        Err(e) => panic!("{}{:#?}", e, full_path),
                    };
                }
                None => unimplemented!(),
            };
            // TODO Generate tokens.
            unimplemented!()
        }
        Err(e) => e.into(),
    }
}

fn parse_derive(
    input: DeriveInput,
) -> (
    Ident,
    Generics,
    Result<Option<String>, proc_macro2::TokenStream>,
) {
    let parser_ident = input.ident;
    let generics = input.generics;

    let attr = match input.attrs.len() {
        0 => Ok(None),
        1 => get_grammar_attr(&input.attrs[0]).map(|a| Some(a)),
        _ => unimplemented!(),
    };

    (parser_ident, generics, attr)
}

fn get_grammar_attr(attr: &Attribute) -> Result<String, proc_macro2::TokenStream> {
    if let Ok(Meta::NameValue(name_value)) = attr.parse_meta() {
        match name_value.lit {
            Lit::Str(lit_str) => {
                if name_value.path.is_ident("grammar") {
                    return Ok(lit_str.value());
                } else {
                    // unimplemented!()
                }
            }
            _ => {
                // unimplemented!()
            }
        }
    }

    Err(syn::Error::new_spanned(attr, "expected `grammar = \"...\"`").to_compile_error())
}

fn read_file<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}
