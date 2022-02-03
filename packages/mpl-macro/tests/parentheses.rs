use mpl_macro::Parser;

#[test]
fn derive_grammar_from_file() {
    #[derive(Parser)]
    #[mplg = "tests/parentheses.mplg"]
    pub struct MyParser;
}
