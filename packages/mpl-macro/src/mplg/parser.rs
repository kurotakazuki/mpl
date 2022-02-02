use crate::mplg::{MplgAST, MplgRules, MplgVariables};
use mpl::parse::Parse;
use mpl::span::StartAndLenSpan;

pub fn parse_mplg(input: &[u8]) -> Result<MplgAST, MplgAST> {
    let all_of_the_span = StartAndLenSpan::<u32, u32>::from_start_len(0, input.len() as u32);
    let rules = &MplgRules;
    input.minimal_parse(rules, &MplgVariables::Mplg, &all_of_the_span)
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
            // Strings
            (
                "A = \"string\" \"''\r\n\n\\\"\\\"\n\" / \"\"\n",
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
