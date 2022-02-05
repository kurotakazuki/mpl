use mpl::output::Output;
use mpl::parser::Parser;
use mpl::rules::{RightRule, RightRuleKind, Rules};
use mpl::span::{Len, Span, Start, StartAndLenSpan};
use mpl::symbols::{SliceTerminal, Variable};
use mpl::trees::{AST, CST};
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum WavRiffVariable {
    // RIFF chunk
    Riff,
    FileSize,
    Wave,
    U32,
}

#[derive(Debug)]
enum U16OrU32 {
    // U16(u16),
    U32(u32),
}

impl Variable for WavRiffVariable {}

impl<'i> Output<'i, [u8], WavRiffVariable, StartAndLenSpan<u32, u16>> for U16OrU32 {
    fn output_ast(
        input: &'i [u8],
        cst: CST<WavRiffVariable, StartAndLenSpan<u32, u16>, Self>,
    ) -> AST<WavRiffVariable, StartAndLenSpan<u32, u16>, Self> {
        match cst.node.value {
            WavRiffVariable::U32 => {
                let lo = cst.span.start as usize;
                let hi = cst.span.hi(input) as usize;

                let n = u32::from_le_bytes(input[lo..hi].try_into().unwrap());

                AST::from_cst_and_output(cst, Some(U16OrU32::U32(n)))
            }
            _ => AST::from_cst(cst),
        }
    }
}

struct WavRiffParser;

/// T represents the element type.
impl<'i, T, V, P, L, R, O> Parser<'i, [T], SliceTerminal<'i, T>, V, StartAndLenSpan<P, L>, P, R, O>
    for WavRiffParser
where
    T: PartialEq,
    V: Variable,
    P: Start<[T], L>,
    L: Len<[T], P>,
    R: Rules<SliceTerminal<'i, T>, V>,
    O: Output<'i, [T], V, StartAndLenSpan<P, L>>,
{
}

/// ```
/// Riff = b"RIFF" FileSize / f
/// FileSize = U32 Wave / f
/// Wave = b"WAVE" () / f
///
/// // U16 = ?? () / f
/// U32 = ???? () / f
/// ```
#[test]
fn wav_riff() {
    let parser = WavRiffParser;
    let mut rules = HashMap::new();

    rules.insert(
        WavRiffVariable::Riff,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(SliceTerminal::<u8>::Slice(b"RIFF")),
                RightRuleKind::V(WavRiffVariable::FileSize),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavRiffVariable::FileSize,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(WavRiffVariable::U32),
                RightRuleKind::V(WavRiffVariable::Wave),
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavRiffVariable::Wave,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(SliceTerminal::<u8>::Slice(b"WAVE")),
                RightRuleKind::Empty,
            ),
            RightRuleKind::Failure,
        ),
    );
    rules.insert(
        WavRiffVariable::U32,
        RightRule::from_right_rule_kind(
            (RightRuleKind::Any(4), RightRuleKind::Empty),
            RightRuleKind::Failure,
        ),
    );

    let input: &[u8] = &[
        0x52, 0x49, 0x46, 0x46, 0x04, 0x00, 0x00, 0x00, 0x57, 0x41, 0x56, 0x45,
    ][..];
    // all of the span
    let all_of_the_span = StartAndLenSpan::<u32, u16>::from_start_len(0, input.len() as u16);

    let result: Result<
        AST<WavRiffVariable, StartAndLenSpan<u32, u16>, U16OrU32>,
        AST<WavRiffVariable, StartAndLenSpan<u32, u16>, U16OrU32>,
    > = parser.parse(input, &rules, &WavRiffVariable::Riff, &all_of_the_span);

    assert!(result.is_ok());

    let input: &[u8] = &[
        0x52, 0x43, 0x46, 0x46, 0x04, 0x00, 0x00, 0x00, 0x57, 0x41, 0x56, 0x45,
    ][..];
    // all of the span
    let all_of_the_span = StartAndLenSpan::<u32, u16>::from_start_len(0, input.len() as u16);

    let result: Result<
        AST<WavRiffVariable, StartAndLenSpan<u32, u16>, U16OrU32>,
        AST<WavRiffVariable, StartAndLenSpan<u32, u16>, U16OrU32>,
    > = parser.parse(input, &rules, &WavRiffVariable::Riff, &all_of_the_span);

    assert!(result.is_err());
}
