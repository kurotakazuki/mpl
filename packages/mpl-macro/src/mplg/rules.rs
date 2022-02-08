use crate::mplg::{MplgVariable, MplgVariable::*};

use mpl::choices::{First, Second};
use mpl::e_from;
use mpl::rules::{RightRule, Rules};
use mpl::symbols::{U8SliceTerminal, U8SliceTerminal::*};

pub struct MplgRules;

type MplgRightRule<'a> = RightRule<U8SliceTerminal<'a>, MplgVariable>;

macro_rules! mplg_rule {
    ($rule_ident:ident, $v:ty, $fl:tt, $fr:tt, $s:tt) => {
        /// $v = $fl $fr / $s
        const $rule_ident: MplgRightRule<'a> = RightRule {
            first: First {
                lhs: e_from!($fl),
                rhs: e_from!($fr),
            },
            second: Second(e_from!($s)),
        };
    };
}

impl<'a> MplgRules {
    mplg_rule!(MPLG_RULE, Mplg, ZeroOrMoreLines, (), f);
    mplg_rule!(
        ZERO_OR_MORE_LINES_RULE,
        ZeroOrMoreLines,
        Line,
        ZeroOrMoreLines,
        ()
    );

    mplg_rule!(LINE_RULE, Line, Line1, EndOfLine, f);
    mplg_rule!(LINE1_RULE, Line1, LineComment, (), Line2);
    mplg_rule!(LINE2_RULE, Line2, Rule, (), ());
    // Rule
    mplg_rule!(RULE_RULE, Rule, Variable, Rule1, f);
    mplg_rule!(RULE1_RULE, Rule1, { Str(" = ") }, Rule2, f);
    mplg_rule!(RULE2_RULE, Rule2, E, Rule3, f);
    mplg_rule!(RULE3_RULE, Rule3, Space, Rule4, f);
    mplg_rule!(RULE4_RULE, Rule4, E, Rule5, f);
    mplg_rule!(RULE5_RULE, Rule5, { Str(" / ") }, Rule6, f);
    mplg_rule!(RULE6_RULE, Rule6, E, (), f);
    mplg_rule!(E_RULE, E, TerminalSymbol, (), Variable);
    // Lexical syntax
    // Variable
    mplg_rule!(VARIABLE_RULE, Variable, Identifier, (), f);

    // Terminal symbol
    mplg_rule!(
        TERMINAL_SYMBOL_RULE,
        TerminalSymbol,
        MetasymbolLiteral,
        (),
        OriginalSymbolExpr
    );
    // Expr
    mplg_rule!(EXPR_RULE, Expr, ExprWithoutBlock, (), f);

    // Without Block
    mplg_rule!(
        EXPR_WITHOUT_BLOCK_RULE,
        ExprWithoutBlock,
        LiteralExpr,
        (),
        ExprWithoutBlock1
    );
    mplg_rule!(
        EXPR_WITHOUT_BLOCK1_RULE,
        ExprWithoutBlock1,
        StructExpr,
        (),
        f
    );

    // Struct
    mplg_rule!(
        STRUCT_EXPR_RULE,
        StructExpr,
        StructExprStruct,
        (),
        StructExpr1
    );
    mplg_rule!(
        STRUCT_EXPR1_RULE,
        StructExpr1,
        StructExprTuple,
        (),
        StructExprUnit
    );

    mplg_rule!(STRUCT_EXPR_STRUCT_RULE, StructExprStruct, f, f, f);

    mplg_rule!(
        STRUCT_EXPR_TUPLE_RULE,
        StructExprTuple,
        PathInExpr,
        StructExprTuple1,
        f
    );
    mplg_rule!(
        STRUCT_EXPR_TUPLE1_RULE,
        StructExprTuple1,
        { Char('(') },
        StructExprTuple2,
        f
    );
    mplg_rule!(
        STRUCT_EXPR_TUPLE2_RULE,
        StructExprTuple2,
        ZeroOrMoreExpr,
        { Char(')') },
        f
    );
    mplg_rule!(ZERO_OR_MORE_EXPR_RULE, ZeroOrMoreExpr, Expr, (), f);

    mplg_rule!(STRUCT_EXPR_UNIT_RULE, StructExprUnit, PathInExpr, (), f);

    // PathInExpr
    mplg_rule!(
        PATH_IN_EXPR_RULE,
        PathInExpr,
        ZeroOrOneDoubleColon,
        OneOrMorePathExprSegment,
        f
    );
    mplg_rule!(
        ZERO_OR_ONE_DOUBLE_COLON_RULE,
        ZeroOrOneDoubleColon,
        { Str("::") },
        (),
        ()
    );
    mplg_rule!(
        ONE_OR_MORE_PATH_EXPR_SEGMENT_RULE,
        OneOrMorePathExprSegment,
        PathExprSegment,
        (),
        f
    );

    mplg_rule!(
        PATH_EXPR_SEGMENT_RULE,
        PathExprSegment,
        PathIdentSegment,
        PathExprSegment1,
        f
    );
    mplg_rule!(
        PATH_EXPR_SEGMENT1_RULE,
        PathExprSegment1,
        { Str("::") },
        GenericArgs,
        ()
    );

    mplg_rule!(PATH_IDENT_SEGMENT_RULE, PathIdentSegment, Identifier, (), f);

    mplg_rule!(GENERIC_ARGS_RULE, GenericArgs, f, f, f);

    // Literal
    mplg_rule!(
        LITERAL_EXPR_RULE,
        LiteralExpr,
        StringLiteral,
        (),
        LiteralExpr1
    );
    mplg_rule!(LITERAL_EXPR1_RULE, LiteralExpr1, IntegerLiteral, (), f);

    // Metasymbol
    mplg_rule!(
        METASYMBOL_LITERAL_RULE,
        MetasymbolLiteral,
        EmptyLiteral,
        (),
        MetasymbolLiteral1
    );
    mplg_rule!(
        METASYMBOL_LITERAL1_RULE,
        MetasymbolLiteral1,
        FailureLiteral,
        (),
        MetasymbolLiteral2
    );
    mplg_rule!(
        METASYMBOL_LITERAL2_RULE,
        MetasymbolLiteral2,
        AnyLiteral,
        (),
        MetasymbolLiteral3
    );
    mplg_rule!(
        METASYMBOL_LITERAL3_RULE,
        MetasymbolLiteral3,
        AllLiteral,
        (),
        f
    );
    mplg_rule!(EMPTY_RULE, EmptyLiteral, { Str("()") }, (), f);
    mplg_rule!(FAILURE_LITERAL_RULE, FailureLiteral, { Char('f') }, (), f);
    mplg_rule!(
        ANY_LITERAL_RULE,
        AnyLiteral,
        { Char('?') },
        ZeroOrMoreAny,
        f
    );
    mplg_rule!(
        ZERO_OR_MORE_ANY_RULE,
        ZeroOrMoreAny,
        { Char('?') },
        ZeroOrMoreAny,
        ()
    );
    mplg_rule!(ALL_LITERAL_RULE, AllLiteral, { Char('*') }, (), f);

    // Original symbol
    mplg_rule!(
        ORIGINAL_SYMBOL_EXPR_RULE,
        OriginalSymbolExpr,
        { Str("{ ") },
        OriginalSymbolExpr1,
        f
    );
    mplg_rule!(
        ORIGINAL_SYMBOL_EXPR1_RULE,
        OriginalSymbolExpr1,
        ExprWithoutBlock,
        { Str(" }") },
        f
    );

    // String
    mplg_rule!(
        STRING_LITERAL_RULE,
        StringLiteral,
        { Char('"') },
        StringLiteral1,
        f
    );
    mplg_rule!(
        STRING_LITERAL1_RULE,
        StringLiteral1,
        InnerStringLiteral,
        { Char('"') },
        f
    );
    mplg_rule!(
        INNER_STRING_LITERAL_RULE,
        InnerStringLiteral,
        InnerStringLiteralLetter,
        InnerStringLiteral,
        ()
    );
    // InnerStringLiteralLetter
    mplg_rule!(
        INNER_STRING_LITERAL_LETTER_RULE,
        InnerStringLiteralLetter,
        NotStringLetter,
        InnerStringLiteral1Letter1,
        f
    );
    mplg_rule!(NOT_STRING_LETTER_RULE, NotStringLetter, { Char('"') }, *, ());
    mplg_rule!(INNER_STRING_LITERAL_LETTER1_RULE, InnerStringLiteral1Letter1, QuoteEscape, (), ?);

    // Integer
    mplg_rule!(INTEGER_LITERAL_RULE, IntegerLiteral, IntegerLiterals, (), f);
    mplg_rule!(INTEGER_LITERALS_RULE, IntegerLiterals, DecLiteral, (), f);
    mplg_rule!(
        DEC_LITERAL_RULE,
        DecLiteral,
        DecDigit,
        ZeroOrMoreDecDigit,
        f
    );
    mplg_rule!(
        ZERO_OR_MORE_DEC_DIGIT_RULE,
        ZeroOrMoreDecDigit,
        DecDigitOrUnderscore,
        ZeroOrMoreDecDigit,
        ()
    );
    mplg_rule!(
        DEC_DIGIT_OR_UNDERSCORE_RULE,
        DecDigitOrUnderscore,
        DecDigit,
        (),
        { Char('_') }
    );

    // IDENTIFIER
    mplg_rule!(
        IDENTIFIER_RULE,
        Identifier,
        Uppercase,
        ZeroOrMoreIdentifierContinue,
        f
    );
    mplg_rule!(
        ZERO_OR_MORE_IDENTIFIER_CONTINUE_RULE,
        ZeroOrMoreIdentifierContinue,
        IdentifierContinue,
        ZeroOrMoreIdentifierContinue,
        ()
    );
    mplg_rule!(
        IDENTIFIER_CONTINUE_RULE,
        IdentifierContinue,
        Alphabet,
        (),
        DecDigit
    );

    // Letters
    mplg_rule!(ALPHABET_RULE, Alphabet, Lowercase, (), Uppercase);
    // Lowercase
    mplg_rule!(LOWERCASE_A_TO_F_RULE, LowercaseAToF, LowercaseAToF1, (), f);
    mplg_rule!(
        LOWERCASE_A_TO_F1_RULE,
        LowercaseAToF1,
        { Char('a') },
        (),
        LowercaseAToF2
    );
    mplg_rule!(
        LOWERCASE_A_TO_F2_RULE,
        LowercaseAToF2,
        { Char('b') },
        (),
        LowercaseAToF3
    );
    mplg_rule!(
        LOWERCASE_A_TO_F3_RULE,
        LowercaseAToF3,
        { Char('c') },
        (),
        LowercaseAToF4
    );
    mplg_rule!(
        LOWERCASE_A_TO_F4_RULE,
        LowercaseAToF4,
        { Char('d') },
        (),
        LowercaseAToF5
    );
    mplg_rule!(
        LOWERCASE_A_TO_F5_RULE,
        LowercaseAToF5,
        { Char('e') },
        (),
        LowercaseAToF6
    );
    mplg_rule!(LOWERCASE_A_TO_F6_RULE, LowercaseAToF6, { Char('f') }, (), f);
    mplg_rule!(LOWERCASE_RULE, Lowercase, LowercaseAToF, (), Lowercase1);
    mplg_rule!(LOWERCASE1_RULE, Lowercase1, { Char('g') }, (), Lowercase2);
    mplg_rule!(LOWERCASE2_RULE, Lowercase2, { Char('h') }, (), Lowercase3);
    mplg_rule!(LOWERCASE3_RULE, Lowercase3, { Char('i') }, (), Lowercase4);
    mplg_rule!(LOWERCASE4_RULE, Lowercase4, { Char('j') }, (), Lowercase5);
    mplg_rule!(LOWERCASE5_RULE, Lowercase5, { Char('k') }, (), Lowercase6);
    mplg_rule!(LOWERCASE6_RULE, Lowercase6, { Char('l') }, (), Lowercase7);
    mplg_rule!(LOWERCASE7_RULE, Lowercase7, { Char('m') }, (), Lowercase8);
    mplg_rule!(LOWERCASE8_RULE, Lowercase8, { Char('n') }, (), Lowercase9);
    mplg_rule!(LOWERCASE9_RULE, Lowercase9, { Char('o') }, (), Lowercase10);
    mplg_rule!(
        LOWERCASE10_RULE,
        Lowercase10,
        { Char('p') },
        (),
        Lowercase11
    );
    mplg_rule!(
        LOWERCASE11_RULE,
        Lowercase11,
        { Char('q') },
        (),
        Lowercase12
    );
    mplg_rule!(
        LOWERCASE12_RULE,
        Lowercase12,
        { Char('r') },
        (),
        Lowercase13
    );
    mplg_rule!(
        LOWERCASE13_RULE,
        Lowercase13,
        { Char('s') },
        (),
        Lowercase14
    );
    mplg_rule!(
        LOWERCASE14_RULE,
        Lowercase14,
        { Char('t') },
        (),
        Lowercase15
    );
    mplg_rule!(
        LOWERCASE15_RULE,
        Lowercase15,
        { Char('u') },
        (),
        Lowercase16
    );
    mplg_rule!(
        LOWERCASE16_RULE,
        Lowercase16,
        { Char('v') },
        (),
        Lowercase17
    );
    mplg_rule!(
        LOWERCASE17_RULE,
        Lowercase17,
        { Char('w') },
        (),
        Lowercase18
    );
    mplg_rule!(
        LOWERCASE18_RULE,
        Lowercase18,
        { Char('x') },
        (),
        Lowercase19
    );
    mplg_rule!(
        LOWERCASE19_RULE,
        Lowercase19,
        { Char('y') },
        (),
        Lowercase20
    );
    mplg_rule!(LOWERCASE20_RULE, Lowercase20, { Char('z') }, (), f);
    // Uppercase
    mplg_rule!(UPPERCASE_A_TO_F_RULE, UppercaseAToF, UppercaseAToF1, (), f);
    mplg_rule!(
        UPPERCASE_A_TO_F1_RULE,
        UppercaseAToF1,
        { Char('A') },
        (),
        UppercaseAToF2
    );
    mplg_rule!(
        UPPERCASE_A_TO_F2_RULE,
        UppercaseAToF2,
        { Char('B') },
        (),
        UppercaseAToF3
    );
    mplg_rule!(
        UPPERCASE_A_TO_F3_RULE,
        UppercaseAToF3,
        { Char('C') },
        (),
        UppercaseAToF4
    );
    mplg_rule!(
        UPPERCASE_A_TO_F4_RULE,
        UppercaseAToF4,
        { Char('D') },
        (),
        UppercaseAToF5
    );
    mplg_rule!(
        UPPERCASE_A_TO_F5_RULE,
        UppercaseAToF5,
        { Char('E') },
        (),
        UppercaseAToF6
    );
    mplg_rule!(UPPERCASE_A_TO_F6_RULE, UppercaseAToF6, { Char('F') }, (), f);
    mplg_rule!(UPPERCASE_RULE, Uppercase, UppercaseAToF, (), Uppercase1);
    mplg_rule!(UPPERCASE1_RULE, Uppercase1, { Char('G') }, (), Uppercase2);
    mplg_rule!(UPPERCASE2_RULE, Uppercase2, { Char('H') }, (), Uppercase3);
    mplg_rule!(UPPERCASE3_RULE, Uppercase3, { Char('I') }, (), Uppercase4);
    mplg_rule!(UPPERCASE4_RULE, Uppercase4, { Char('J') }, (), Uppercase5);
    mplg_rule!(UPPERCASE5_RULE, Uppercase5, { Char('K') }, (), Uppercase6);
    mplg_rule!(UPPERCASE6_RULE, Uppercase6, { Char('L') }, (), Uppercase7);
    mplg_rule!(UPPERCASE7_RULE, Uppercase7, { Char('M') }, (), Uppercase8);
    mplg_rule!(UPPERCASE8_RULE, Uppercase8, { Char('N') }, (), Uppercase9);
    mplg_rule!(UPPERCASE9_RULE, Uppercase9, { Char('O') }, (), Uppercase10);
    mplg_rule!(
        UPPERCASE10_RULE,
        Uppercase10,
        { Char('P') },
        (),
        Uppercase11
    );
    mplg_rule!(
        UPPERCASE11_RULE,
        Uppercase11,
        { Char('Q') },
        (),
        Uppercase12
    );
    mplg_rule!(
        UPPERCASE12_RULE,
        Uppercase12,
        { Char('R') },
        (),
        Uppercase13
    );
    mplg_rule!(
        UPPERCASE13_RULE,
        Uppercase13,
        { Char('S') },
        (),
        Uppercase14
    );
    mplg_rule!(
        UPPERCASE14_RULE,
        Uppercase14,
        { Char('T') },
        (),
        Uppercase15
    );
    mplg_rule!(
        UPPERCASE15_RULE,
        Uppercase15,
        { Char('U') },
        (),
        Uppercase16
    );
    mplg_rule!(
        UPPERCASE16_RULE,
        Uppercase16,
        { Char('V') },
        (),
        Uppercase17
    );
    mplg_rule!(
        UPPERCASE17_RULE,
        Uppercase17,
        { Char('W') },
        (),
        Uppercase18
    );
    mplg_rule!(
        UPPERCASE18_RULE,
        Uppercase18,
        { Char('X') },
        (),
        Uppercase19
    );
    mplg_rule!(
        UPPERCASE19_RULE,
        Uppercase19,
        { Char('Y') },
        (),
        Uppercase20
    );
    mplg_rule!(UPPERCASE20_RULE, Uppercase20, { Char('Z') }, (), f);

    mplg_rule!(QUOTE_ESCAPE_RULE, QuoteEscape, { Str("\\'") }, (), {
        Str("\\\"")
    });
    mplg_rule!(END_OF_LINE_RULE, EndOfLine, { Char('\n') }, (), {
        Str("\r\n")
    });
    mplg_rule!(SPACE_RULE, Space, { Char(' ') }, (), f);

    // Digits
    mplg_rule!(BIN_DIGIT_RULE, BinDigit, { Char('0') }, (), { Char('1') });
    mplg_rule!(OCT_DIGIT_RULE, OctDigit, BinDigit, (), OctDigit1);
    mplg_rule!(OCT_DIGIT1_RULE, OctDigit1, { Char('2') }, (), OctDigit2);
    mplg_rule!(OCT_DIGIT2_RULE, OctDigit2, { Char('3') }, (), OctDigit3);
    mplg_rule!(OCT_DIGIT3_RULE, OctDigit3, { Char('4') }, (), OctDigit4);
    mplg_rule!(OCT_DIGIT4_RULE, OctDigit4, { Char('5') }, (), OctDigit5);
    mplg_rule!(OCT_DIGIT5_RULE, OctDigit5, { Char('6') }, (), OctDigit6);
    mplg_rule!(OCT_DIGIT6_RULE, OctDigit6, { Char('7') }, (), f);
    mplg_rule!(DEC_DIGIT_RULE, DecDigit, OctDigit, (), DecDigit1);
    mplg_rule!(DEC_DIGIT1_RULE, DecDigit1, { Char('8') }, (), DecDigit2);
    mplg_rule!(DEC_DIGIT2_RULE, DecDigit2, { Char('9') }, (), f);

    // Comment
    mplg_rule!(
        LINE_COMMENT_RULE,
        LineComment,
        { Str("//") },
        InnerLineComment,
        f
    );
    mplg_rule!(
        INNER_LINE_COMMENT_RULE,
        InnerLineComment,
        AnyExceptLF,
        InnerLineComment,
        ()
    );
    mplg_rule!(ANY_EXCEPT_L_F_RULE, AnyExceptLF, AnyExceptLF1, ?, f);
    mplg_rule!(ANY_EXCEPT_L_F1_RULE, AnyExceptLF1, EndOfLine, *, ());
}

impl<'a> Rules<U8SliceTerminal<'a>, MplgVariable> for MplgRules {
    fn get(&self, variable: &MplgVariable) -> Option<&MplgRightRule<'a>> {
        Some(match variable {
            Mplg => &Self::MPLG_RULE,
            ZeroOrMoreLines => &Self::ZERO_OR_MORE_LINES_RULE,
            Line => &Self::LINE_RULE,
            Line1 => &Self::LINE1_RULE,
            Line2 => &Self::LINE2_RULE,
            // Rule
            Rule => &Self::RULE_RULE,
            Rule1 => &Self::RULE1_RULE,
            Rule2 => &Self::RULE2_RULE,
            Rule3 => &Self::RULE3_RULE,
            Rule4 => &Self::RULE4_RULE,
            Rule5 => &Self::RULE5_RULE,
            Rule6 => &Self::RULE6_RULE,
            E => &Self::E_RULE,
            // Lexical syntax
            // Variable
            Variable => &Self::VARIABLE_RULE,

            // Terminal symbol
            TerminalSymbol => &Self::TERMINAL_SYMBOL_RULE,
            // Expr
            Expr => &Self::EXPR_RULE,

            // Without Block
            ExprWithoutBlock => &Self::EXPR_WITHOUT_BLOCK_RULE,
            ExprWithoutBlock1 => &Self::EXPR_WITHOUT_BLOCK1_RULE,

            // Struct
            StructExpr => &Self::STRUCT_EXPR_RULE,
            StructExpr1 => &Self::STRUCT_EXPR1_RULE,

            StructExprStruct => &Self::STRUCT_EXPR_STRUCT_RULE,

            StructExprTuple => &Self::STRUCT_EXPR_TUPLE_RULE,
            StructExprTuple1 => &Self::STRUCT_EXPR_TUPLE1_RULE,
            StructExprTuple2 => &Self::STRUCT_EXPR_TUPLE2_RULE,
            ZeroOrMoreExpr => &Self::ZERO_OR_MORE_EXPR_RULE,

            StructExprUnit => &Self::STRUCT_EXPR_UNIT_RULE,

            // PathInExpr
            PathInExpr => &Self::PATH_IN_EXPR_RULE,
            ZeroOrOneDoubleColon => &Self::ZERO_OR_ONE_DOUBLE_COLON_RULE,
            OneOrMorePathExprSegment => &Self::ONE_OR_MORE_PATH_EXPR_SEGMENT_RULE,

            PathExprSegment => &Self::PATH_EXPR_SEGMENT_RULE,
            PathExprSegment1 => &Self::PATH_EXPR_SEGMENT1_RULE,

            PathIdentSegment => &Self::PATH_IDENT_SEGMENT_RULE,

            GenericArgs => &Self::GENERIC_ARGS_RULE,

            // Literal
            LiteralExpr => &Self::LITERAL_EXPR_RULE,
            LiteralExpr1 => &Self::LITERAL_EXPR1_RULE,

            // Metasymbol
            MetasymbolLiteral => &Self::METASYMBOL_LITERAL_RULE,
            MetasymbolLiteral1 => &Self::METASYMBOL_LITERAL1_RULE,
            MetasymbolLiteral2 => &Self::METASYMBOL_LITERAL2_RULE,
            MetasymbolLiteral3 => &Self::METASYMBOL_LITERAL3_RULE,
            EmptyLiteral => &Self::EMPTY_RULE,
            FailureLiteral => &Self::FAILURE_LITERAL_RULE,
            AnyLiteral => &Self::ANY_LITERAL_RULE,
            ZeroOrMoreAny => &Self::ZERO_OR_MORE_ANY_RULE,
            AllLiteral => &Self::ALL_LITERAL_RULE,

            // Original symbol
            OriginalSymbolExpr => &Self::ORIGINAL_SYMBOL_EXPR_RULE,
            OriginalSymbolExpr1 => &Self::ORIGINAL_SYMBOL_EXPR1_RULE,

            // String
            StringLiteral => &Self::STRING_LITERAL_RULE,
            StringLiteral1 => &Self::STRING_LITERAL1_RULE,
            InnerStringLiteral => &Self::INNER_STRING_LITERAL_RULE,
            // InnerStringLiteralLetter
            InnerStringLiteralLetter => &Self::INNER_STRING_LITERAL_LETTER_RULE,
            NotStringLetter => &Self::NOT_STRING_LETTER_RULE,
            InnerStringLiteral1Letter1 => &Self::INNER_STRING_LITERAL_LETTER1_RULE,

            // Integer
            IntegerLiteral => &Self::INTEGER_LITERAL_RULE,
            IntegerLiterals => &Self::INTEGER_LITERALS_RULE,
            DecLiteral => &Self::DEC_LITERAL_RULE,
            ZeroOrMoreDecDigit => &Self::ZERO_OR_MORE_DEC_DIGIT_RULE,
            DecDigitOrUnderscore => &Self::DEC_DIGIT_OR_UNDERSCORE_RULE,

            // IDENTIFIER
            Identifier => &Self::IDENTIFIER_RULE,
            ZeroOrMoreIdentifierContinue => &Self::ZERO_OR_MORE_IDENTIFIER_CONTINUE_RULE,
            IdentifierContinue => &Self::IDENTIFIER_CONTINUE_RULE,

            // Letters
            Alphabet => &Self::ALPHABET_RULE,
            // Lowercase
            LowercaseAToF => &Self::LOWERCASE_A_TO_F_RULE,
            LowercaseAToF1 => &Self::LOWERCASE_A_TO_F1_RULE,
            LowercaseAToF2 => &Self::LOWERCASE_A_TO_F2_RULE,
            LowercaseAToF3 => &Self::LOWERCASE_A_TO_F3_RULE,
            LowercaseAToF4 => &Self::LOWERCASE_A_TO_F4_RULE,
            LowercaseAToF5 => &Self::LOWERCASE_A_TO_F5_RULE,
            LowercaseAToF6 => &Self::LOWERCASE_A_TO_F6_RULE,
            Lowercase => &Self::LOWERCASE_RULE,
            Lowercase1 => &Self::LOWERCASE1_RULE,
            Lowercase2 => &Self::LOWERCASE2_RULE,
            Lowercase3 => &Self::LOWERCASE3_RULE,
            Lowercase4 => &Self::LOWERCASE4_RULE,
            Lowercase5 => &Self::LOWERCASE5_RULE,
            Lowercase6 => &Self::LOWERCASE6_RULE,
            Lowercase7 => &Self::LOWERCASE7_RULE,
            Lowercase8 => &Self::LOWERCASE8_RULE,
            Lowercase9 => &Self::LOWERCASE9_RULE,
            Lowercase10 => &Self::LOWERCASE10_RULE,
            Lowercase11 => &Self::LOWERCASE11_RULE,
            Lowercase12 => &Self::LOWERCASE12_RULE,
            Lowercase13 => &Self::LOWERCASE13_RULE,
            Lowercase14 => &Self::LOWERCASE14_RULE,
            Lowercase15 => &Self::LOWERCASE15_RULE,
            Lowercase16 => &Self::LOWERCASE16_RULE,
            Lowercase17 => &Self::LOWERCASE17_RULE,
            Lowercase18 => &Self::LOWERCASE18_RULE,
            Lowercase19 => &Self::LOWERCASE19_RULE,
            Lowercase20 => &Self::LOWERCASE20_RULE,
            // Uppercase
            UppercaseAToF => &Self::UPPERCASE_A_TO_F_RULE,
            UppercaseAToF1 => &Self::UPPERCASE_A_TO_F1_RULE,
            UppercaseAToF2 => &Self::UPPERCASE_A_TO_F2_RULE,
            UppercaseAToF3 => &Self::UPPERCASE_A_TO_F3_RULE,
            UppercaseAToF4 => &Self::UPPERCASE_A_TO_F4_RULE,
            UppercaseAToF5 => &Self::UPPERCASE_A_TO_F5_RULE,
            UppercaseAToF6 => &Self::UPPERCASE_A_TO_F6_RULE,
            Uppercase => &Self::UPPERCASE_RULE,
            Uppercase1 => &Self::UPPERCASE1_RULE,
            Uppercase2 => &Self::UPPERCASE2_RULE,
            Uppercase3 => &Self::UPPERCASE3_RULE,
            Uppercase4 => &Self::UPPERCASE4_RULE,
            Uppercase5 => &Self::UPPERCASE5_RULE,
            Uppercase6 => &Self::UPPERCASE6_RULE,
            Uppercase7 => &Self::UPPERCASE7_RULE,
            Uppercase8 => &Self::UPPERCASE8_RULE,
            Uppercase9 => &Self::UPPERCASE9_RULE,
            Uppercase10 => &Self::UPPERCASE10_RULE,
            Uppercase11 => &Self::UPPERCASE11_RULE,
            Uppercase12 => &Self::UPPERCASE12_RULE,
            Uppercase13 => &Self::UPPERCASE13_RULE,
            Uppercase14 => &Self::UPPERCASE14_RULE,
            Uppercase15 => &Self::UPPERCASE15_RULE,
            Uppercase16 => &Self::UPPERCASE16_RULE,
            Uppercase17 => &Self::UPPERCASE17_RULE,
            Uppercase18 => &Self::UPPERCASE18_RULE,
            Uppercase19 => &Self::UPPERCASE19_RULE,
            Uppercase20 => &Self::UPPERCASE20_RULE,

            QuoteEscape => &Self::QUOTE_ESCAPE_RULE,
            EndOfLine => &Self::END_OF_LINE_RULE,
            Space => &Self::SPACE_RULE,

            // Digits
            BinDigit => &Self::BIN_DIGIT_RULE,
            OctDigit => &Self::OCT_DIGIT_RULE,
            OctDigit1 => &Self::OCT_DIGIT1_RULE,
            OctDigit2 => &Self::OCT_DIGIT2_RULE,
            OctDigit3 => &Self::OCT_DIGIT3_RULE,
            OctDigit4 => &Self::OCT_DIGIT4_RULE,
            OctDigit5 => &Self::OCT_DIGIT5_RULE,
            OctDigit6 => &Self::OCT_DIGIT6_RULE,
            DecDigit => &Self::DEC_DIGIT_RULE,
            DecDigit1 => &Self::DEC_DIGIT1_RULE,
            DecDigit2 => &Self::DEC_DIGIT2_RULE,

            // Comment
            LineComment => &Self::LINE_COMMENT_RULE,
            InnerLineComment => &Self::INNER_LINE_COMMENT_RULE,
            AnyExceptLF => &Self::ANY_EXCEPT_L_F_RULE,
            AnyExceptLF1 => &Self::ANY_EXCEPT_L_F1_RULE,
        })
    }
}
