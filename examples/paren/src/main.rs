use mpl_macro::Parser;

fn main() {
    #[derive(Parser)]
    #[mplg = "../../examples/paren/parentheses.mplg"] // TODO Use release mpl_macro
    pub struct MyParser;
}
