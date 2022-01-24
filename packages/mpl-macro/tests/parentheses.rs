use mpl_macro::Parser;

#[test]
fn derive_grammar_from_file() {
    #[derive(Parser)]
    #[grammar = "tests/parentheses.mplg"]
    pub struct MyParser;
}
