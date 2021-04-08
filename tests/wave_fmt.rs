use std::convert::TryInto;
use mpg::tree::{AST, CST};
use mpg::parse::Parse;
use mpg::rules::{RightRule, RightRuleKind, Rule, Rules};
use mpg::span::{ByteSpan, Span};
use mpg::symbols::{SliceTerminal, Variable};

use mpg::output::Output;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum WaveFmtVariable {
    // RIFF chunk
    Riff,
    FileSize,
    Wave,

    // Lexical analysis
    // U16,
    // RawU16,
    U32,
    RawU32One,
    RawU32Two,
}

#[derive(Debug)]
enum U16OrU32 {
    // U16(u16),
    U32(u32),
}

impl Variable for WaveFmtVariable {}

impl<'input> Output<'input, [u8], WaveFmtVariable, ByteSpan> for U16OrU32 {
    fn output_ast(input: &'input [u8], cst: CST<Self, WaveFmtVariable, ByteSpan>) -> AST<Self, WaveFmtVariable, ByteSpan> {
        match cst.node.value {
            WaveFmtVariable::U32 => {
                let lo = cst.span.start.0 as usize;
                let hi = cst.span.hi().0 as usize;
                
                let n = u32::from_le_bytes(input[lo..hi].try_into().unwrap());

                AST::from_cst_and_output(cst, Some(U16OrU32::U32(n)))
        },
            _ => AST::from_cst(cst),
        }
    }
}

/// ```
/// Riff = b"RIFF" FileSize / f
/// FileSize = U32 Wave / f
/// Wave = b"WAVE" () / f
/// 
/// // U16 = RawU16 () / f
/// // RawU16 = ? ? / f
/// U32 = ? U32One / f
/// RawU32One = ? RawU32Two / f
/// RawU32Two = ? ? / f
/// 
/// ```
#[test]
fn wave_fmt() {
    let riff_rule: Rule<SliceTerminal::<u8>, WaveFmtVariable> = Rule::new(
        WaveFmtVariable::Riff,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(SliceTerminal::<u8>::Slice(b"RIFF")),
                RightRuleKind::V(WaveFmtVariable::FileSize),
            ),
            RightRuleKind::Failure,
        ),
    );
    let file_size_rule: Rule<SliceTerminal::<u8>, WaveFmtVariable> = Rule::new(
        WaveFmtVariable::FileSize,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(WaveFmtVariable::U32),
                RightRuleKind::V(WaveFmtVariable::Wave),
            ),
            RightRuleKind::Failure,
        ),
    );
    let wave_rule: Rule<SliceTerminal::<u8>, WaveFmtVariable> = Rule::new(
        WaveFmtVariable::Wave,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(SliceTerminal::<u8>::Slice(b"WAVE")),
                RightRuleKind::Epsilon,
            ),
            RightRuleKind::Failure,
        ),
    );


    let u32_rule: Rule<SliceTerminal::<u8>, WaveFmtVariable> = Rule::new(
        WaveFmtVariable::U32,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::Any,
                RightRuleKind::V(WaveFmtVariable::RawU32One),
            ),
            RightRuleKind::Failure,
        ),
    );
    let raw_u32_one_rule: Rule<SliceTerminal::<u8>, WaveFmtVariable> = Rule::new(
        WaveFmtVariable::RawU32One,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::Any,
                RightRuleKind::V(WaveFmtVariable::RawU32Two),
            ),
            RightRuleKind::Failure,
        ),
    );
    let raw_u32_two_rule: Rule<SliceTerminal::<u8>, WaveFmtVariable> = Rule::new(
        WaveFmtVariable::RawU32Two,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::Any,
                RightRuleKind::Any,
            ),
            RightRuleKind::Failure,
        ),
    );

    let mut rules = Rules::new();

    rules.insert_rule(riff_rule);
    rules.insert_rule(file_size_rule);
    rules.insert_rule(wave_rule);

    rules.insert_rule(u32_rule);
    rules.insert_rule(raw_u32_one_rule);
    rules.insert_rule(raw_u32_two_rule);

    let input: &[u8] = &[0x52, 0x49, 0x46, 0x46, 0x04, 0x00, 0x00, 0x00, 0x57, 0x41, 0x56, 0x45][..];
    let result: Result<AST<U16OrU32, WaveFmtVariable, ByteSpan>, ()> =
        input.mpg_parse(&rules, &WaveFmtVariable::Riff, None);

        assert!(result.is_ok());

        let input: &[u8] = &[0x52, 0x43, 0x46, 0x46, 0x04, 0x00, 0x00, 0x00, 0x57, 0x41, 0x56, 0x45][..];
        let result: Result<AST<U16OrU32, WaveFmtVariable, ByteSpan>, ()> =
            input.mpg_parse(&rules, &WaveFmtVariable::Riff, None);
    
            assert!(result.is_err());

}
