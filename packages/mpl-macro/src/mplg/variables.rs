#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MplgVariables {
    // Hierarchical syntax
    Mplg,
    // Line
    Line,
    Line1,
    Line2,
    //Rule
    Rule,
    Rule1,
    Rule2,
    Rule3,
    Rule4,
    Rule5,
    Rule6,
    Rule7,
    E,

    // Lexical syntax
    // Variable
    Variable,
    ZeroOrMoreVariableContinue,
    VariableContinue,

    // Terminal symbol
    TerminalSymbol,

    // Expr
    Expr,

    // Literal
    LiteralExpr,

    // String
    StringLiteral,
    StringLiteral1,
    InnerStringLiteral,
    InnerInnerStringLiteral,

    // Letters
    Alphabet,
    // Lowercase
    LowercaseAToF,
    LowercaseAToF1,
    LowercaseAToF2,
    LowercaseAToF3,
    LowercaseAToF4,
    LowercaseAToF5,
    LowercaseAToF6,
    Lowercase,
    Lowercase1,
    Lowercase2,
    Lowercase3,
    Lowercase4,
    Lowercase5,
    Lowercase6,
    Lowercase7,
    Lowercase8,
    Lowercase9,
    Lowercase10,
    Lowercase11,
    Lowercase12,
    Lowercase13,
    Lowercase14,
    Lowercase15,
    Lowercase16,
    Lowercase17,
    Lowercase18,
    Lowercase19,
    Lowercase20,
    // Uppercase
    UppercaseAToF,
    UppercaseAToF1,
    UppercaseAToF2,
    UppercaseAToF3,
    UppercaseAToF4,
    UppercaseAToF5,
    UppercaseAToF6,
    Uppercase,
    Uppercase1,
    Uppercase2,
    Uppercase3,
    Uppercase4,
    Uppercase5,
    Uppercase6,
    Uppercase7,
    Uppercase8,
    Uppercase9,
    Uppercase10,
    Uppercase11,
    Uppercase12,
    Uppercase13,
    Uppercase14,
    Uppercase15,
    Uppercase16,
    Uppercase17,
    Uppercase18,
    Uppercase19,
    Uppercase20,

    QuoteEscape,
    EndOfLine,
    Space,

    // Digits
    BinDigit,
    OctDigit,
    OctDigit1,
    OctDigit2,
    OctDigit3,
    OctDigit4,
    OctDigit5,
    OctDigit6,
    DecDigit,
    DecDigit1,
    DecDigit2,

    // Comment
    LineComment,
    InnerLineComment,
    AnyExceptLF,
    AnyExceptLF1,
}
