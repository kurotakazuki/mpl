// use mpl_macro::Parse;
// use mpl::parse::Parser;
// use mpl::span::StartAndLenSpan;
// use mpl::trees::AST;

// #[derive(Parse, Debug)]
// #[mplg = "../../examples/paren/parentheses.mplg"] // TODO Use release mpl_macro
// pub struct ParenParser;

// enum ParseResult {
//     Ok,
//     Err,
// }

// const INPUTS: [(&str, ParseResult); 8] = [
//     // Ok
//     ("()", ParseResult::Ok),
//     ("()(())", ParseResult::Ok),
//     ("(()(()))", ParseResult::Ok),
//     // Err
//     ("", ParseResult::Ok),
//     ("(", ParseResult::Err),
//     (")", ParseResult::Err),
//     ("()())", ParseResult::Err),
//     ("(()(())))", ParseResult::Err),
// ];

fn main() {
    // let parser = ParenParser;
    // println!("{:#?}", &parser);
    // for input in INPUTS {
    //     let input_data = input.0.as_bytes();
    //     let parse_result = input.1;
    //     // all of the span
    //     let all_of_the_span = StartAndLenSpan::<u32, u16>::from_start_len(0, input_data.len() as u16);
    //     let result: Result<
    //         AST<ParenVariable, StartAndLenSpan<u32, u16>, ()>,
    //         AST<ParenVariable, StartAndLenSpan<u32, u16>, ()>,
    //     > = input_data.minimal_parse(&ParenRules, &ParenVariable::Open, &all_of_the_span);
    //     match parse_result {
    //         ParseResult::Ok => assert!(result.is_ok()),
    //         ParseResult::Err => assert!(result.is_err()),
    //     }
    // }
}
