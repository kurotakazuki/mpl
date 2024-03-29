use crate::mplg::{MplgAST, MplgRules, MplgVariable};
use mpl::output::Output;
use mpl::parser::Parser;
use mpl::span::{Len, Start, StartAndLenSpan};
use mpl::symbols::U8SliceTerminal;

pub struct MplgParser;

impl<'i, P, L, O>
    Parser<'i, [u8], U8SliceTerminal<'i>, MplgVariable, StartAndLenSpan<P, L>, P, MplgRules, O>
    for MplgParser
where
    P: Start<[u8], L>,
    L: Len<[u8], P>,
    O: Output<'i, [u8], MplgVariable, StartAndLenSpan<P, L>>,
{
}

pub fn parse_mplg(input: &[u8]) -> Result<MplgAST, MplgAST> {
    let parser = MplgParser;
    let all_of_the_span = StartAndLenSpan::<u32, u32>::from_start_len(0, input.len() as u32);
    let rules = &MplgRules;
    parser.parse(input, rules, &MplgVariable::Mplg, &all_of_the_span)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mplg() {
        enum ParseResult {
            Ok,
            Err,
        }
        let inputs = [
            // Ok
            ("", ParseResult::Ok),
            ("\n", ParseResult::Ok),
            ("\r\n", ParseResult::Ok),
            ("\r\n\n\r\n", ParseResult::Ok),
            // Rule
            ("A = B C / D\n", ParseResult::Ok),
            ("Abc1 = B1 CC / D0d\n", ParseResult::Ok),
            // Rules
            (
                "A = B C / D\r\nE = F G / H\nI = J K / L\r\nM = N O / P\n",
                ParseResult::Ok,
            ),
            // LineComment
            ("//\n", ParseResult::Ok),
            ("//\r\n", ParseResult::Ok),
            ("// \n", ParseResult::Ok),
            ("// Hello\n", ParseResult::Ok),
            ("//A = B C / D\n", ParseResult::Ok),
            ("// \"こんにちは!\"\n", ParseResult::Ok),
            // LineComments
            ("//\r\n// a\r\n//b\n", ParseResult::Ok),
            // Metasymbols
            ("A = () () / ()\n", ParseResult::Ok),
            ("A = f f / f\n", ParseResult::Ok),
            ("A = ? ?? / ???\n", ParseResult::Ok),
            ("A = * * / *\n", ParseResult::Ok),
            // Chars
            ("A = { 'b' } { 'c' } / { 'd' }\n", ParseResult::Ok),
            // ("A = { '\'' } { '\'' } / { '\'' }\n", ParseResult::Ok),
            // Strings
            (
                "A = { \"string\" } { \"''\r\n\n\\\"\\\"\n\" } / { \"\" }\n",
                ParseResult::Ok,
            ),
            // Integers
            (
                "A = { 1234567890 } { 1_2__3 } / { 1_____ }\n",
                ParseResult::Ok,
            ),
            // Struct
            (
                "A = { Str(\"b\") } { Null(1) } / { A(2) }\n",
                ParseResult::Ok,
            ),
            // Mplg
            (
                "// Mplg = Line Mplg / ()\nMplg = Line Mplg / ()\r\n\n",
                ParseResult::Ok,
            ),
            // Err
            ("A = B\n", ParseResult::Err),
            ("A = B C / D", ParseResult::Err),
            ("A = B / C / D\n", ParseResult::Err),
            ("a = B C / D\n", ParseResult::Err),
            ("A = b c / d\n", ParseResult::Err),
            ("() = B C / D\n", ParseResult::Err),
            ("A = { 'bb' } { 'c' } / { 'd' }\n", ParseResult::Err),
        ];
        for input in inputs {
            let result = parse_mplg(input.0.as_bytes());
            match input.1 {
                ParseResult::Ok => assert!(result.is_ok()),
                ParseResult::Err => assert!(result.is_err()),
            }
        }
    }
}
