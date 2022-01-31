use crate::mplg::{MplgAST, MplgOutput, MplgRules, MplgVariables};
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
        let inputs = [
            "",
            "\n",
            "\r\n",
            "\r\n\n\r\n",
            // Rule
            "A = B C / D\n",
            "Abc1 = B1 CC / D0d\n",
            // Rules
            "A = B C / D\r\nE = F G / H\nI = J K / L\r\nM = N O / P\n",
            // LineComment
            "//\n",
            "//\r\n",
            "// \n",
            "// Hello\n",
            "//A = B C / D\n",
            "// \"こんにちは!\"\n",
            // LineComments
            "//\r\n// a\r\n//b\n",
            // Strings
            "A = \"string\" \"''\r\n\n\\\"\\\"\n\" / \"\"\n",
            // Mplg
            // "// Mplg = Line Mplg / ()\nMplg = Line Mplg / ()\r\n\n",
        ];
        for input in inputs {
            let result = parse_mplg(input.as_bytes());
            assert!(result.is_ok());
        }
    }
}
