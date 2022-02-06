# Minimal Parsing Language (MPL)

[![Crate](https://img.shields.io/crates/v/mpl.svg)](https://crates.io/crates/mpl)
[![API](https://docs.rs/mpl/badge.svg)](https://docs.rs/mpl)

This is minimal parser combinator of Minimal Parsing Language (MPL) like Top-Down Parsing Language (TDPL). It creates a abstract syntax tree (AST) for each input.

## Getting Started
1. implement `Variable`
2. insert each rule into `HashMap`
3. `minimal_parse()`

- Optional
    - implement `Input`
        - supports `[T]` and `str` by default
    - implement `Position`
        - supports `u*`, `i*`, and `f*` by default
    - implement `Span`
        - supports `StartAndLenSpan` by default
    - implement `Terminal`
        - supports `SliceTerminal`, `StrTerminal`, and `U8SliceTerminal` by default
    - implement `Output`
        - supports `()` by default
    - implement `Rules`
        - supports `HashMap` by default
    - implement `Parse`
        - supports `[T]`, `str`, and `[u8]` by default

### Example
```rust
use crate::ParenthesesVariable::*;
use mpl::parser::Parser;
use mpl::rules::{RightRule, RightRuleKind::*, Rules};
use mpl::span::{StartAndLenSpan, Start, Len};
use mpl::output::Output;
use mpl::symbols::{StrTerminal, StrTerminal::*, Variable};
use mpl::trees::AST;
use std::collections::HashMap;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum ParenthesesVariable {
    Open,
    Parentheses,
    Close,
}

impl Variable for ParenthesesVariable {}

struct ParenthesesParser;

impl<'i, V, P, L, R, O> Parser<'i, str, StrTerminal<'i>, V, StartAndLenSpan<P, L>, P, R, O>
    for ParenthesesParser
where
    V: Variable,
    P: Start<str, L>,
    L: Len<str, P>,
    R: Rules<StrTerminal<'i>, V>,
    O: Output<'i, str, V, StartAndLenSpan<P, L>>,
{
}

/// ```
/// Open = '(' Parentheses / ()
/// Parentheses = Open Close / f
/// Close = ")" Open / f
/// ```
fn main() {
    let parser = ParenthesesParser;
    let mut rules = HashMap::new();

    rules.insert(
        Open,
        RightRule::from_right_rule_kind((T(Char('(')), V(Parentheses)), Empty),
    );
    rules.insert(
        Parentheses,
        RightRule::from_right_rule_kind((V(Open), V(Close)), Failure),
    );
    rules.insert(
        Close,
        RightRule::from_right_rule_kind((T(Str(")")), V(Open)), Failure),
    );

    let input = "(()(()))";

    // all of the span
    let all_of_the_span = StartAndLenSpan::<u32, u16>::from_start_len(0, input.len() as u16);

    let result: Result<
        AST<ParenthesesVariable, StartAndLenSpan<u32, u16>, ()>,
        AST<ParenthesesVariable, StartAndLenSpan<u32, u16>, ()>,
    > = parser.parse(input, &rules, &Open, &all_of_the_span);

    if let Ok(ast) = result {
        println!("{}", ast);
    }
}
```

### Test Examples
- [Number](tests/number.rs)
- [Parentheses](tests/parentheses.rs)
- [Wav Riff](tests/wav_riff.rs)

### Parsers written with MPL
- [WAV AST](https://github.com/kurotakazuki/wav_ast) : RIFF waveform Audio Format

## MPL
### Definition of MPL grammar
A MPL grammar `G` is a tuple `G = (V, Σ, R, S)` in which:
- `V` is a finite set of variables.
- `Σ` is a finite set of original terminal symbols.
- `T` is an union of `Σ` or `M` (Σ &cup; M) (`M` (= {(), f}) is a finite set of metasymbols).
- `R` is a finite set of rules of the form
    - `A = B C / D`  
    A in V (A &isin; V),  
    B, C, D in E (E = T &cup; V) (T &cap; V = &empty;) (B, C, D &isin; E).  
    For any variable A there is exactly one rule with A to the left of `=`.
- S in V (S &isin; V) is the start variable.

#### Empty
`()` is a metasymbol that always succeeds without consuming input.

```rust ignore
Empty = () () / ()
```

#### Failure
`f` is a metasymbol that always fails without consuming input.

```rust ignore
Failure = f f / f
```

### Extended MPL
Since one of the goals of MPL is to create an AST, it also supports two features in terms of ease of use and speed.

#### Any
`?` is a metasymbol representing any single input like wildcard character. This succeeds if there is any input left, and fails if there is no input left.

```rust ignore
Any = ? () / f
```

To extend the difinition of MPL grammar, let ? &isin; M.

#### All
`*` is a metasymbol representing All remaining input like wildcard character. This will succeed even if the remaining inputs are zero.

```rust ignore
All = * () / f
```

Same as `All = ? All / ()`.

To extend the difinition of MPL grammar, let * &isin; M.

<!---
#### Variable type
Variables can have a type.

If the variable contains a type, it will include the value of that type, such as a token, when the AST is created. Therefore rules decomposed from variable including rule has a role like lexical analysis. The following syntax is a lexical syntax for numbers.

```
Number: String = Digit Numeral / f
Numeral = Digit Numeral / ()
Digit = Zero () / f
Zero = "0" () / One
One = "1" () / Two
// ...
Nine = "9" () / f
```

An error (TODO: maybe failure would be better) will occur if the input cannot be converted to the variable type.

To extend the difinition of MPL grammar, change `A = B C / D` to `A = B C / D` or `A: TYPE = B C / D`, or seperate definition of `V` by including type or not.
--->


<!-- #### Terminal symbol type
Terminal symbols supports several types.

```
A = "A" "abc" / [0, 0, 0]
```

Supports `&str, &[u8]` at this moment. -->

## Difference between TDPL and MPL
The biggest difference between the two grammars is the rule form. There are two rule forms in TDPL.

> `A..BC/D`, A,B,C,D in V.  
> `A..a`, a in &sum; &cup; {&epsilon;, f}, f is a metasymbol not in &sum; and &epsilon; is the null string.

MPL, on the other hand, has one rule form.


## MPLG (MPL Grammar) syntax
### In PEG like grammar
```rust ignore
// Hierarchical syntax
MPLG = (Line)*
Line = (LineComment / Rule / ()) EndOfLine
Rule = Variable " = " E Space E " / " E
E = TerminalSymbol / Variable

// Lexical syntax
// Variable
Variable = Uppercase (Alphabet / DecDigit)*

// Terminal symbol
TerminalSymbol = Expr

// Expr
Expr = LiteralExpr

// Literal
LiteralExpr = MetasymbolLiteral / StringLiteral

// Metasymbol
MetasymbolLiteral = EmptyLiteral / FailureLiteral / AnyLiteral / AllLiteral
EmptyLiteral = "()" () / f
FailureLiteral = 'f' () / f
AnyLiteral = '?' () / f
AllLiteral = '*' () / f

// String
StringLiteral = "\"" (NotStringLetter / QuoteEscape / ?)* "\""
NotStringLetter = !("\"")

// Letters
Alphabet = Lowercase / Uppercase
UppercaseAToF = "A" / "B" / "C" / "D" / "E" / "F"
LowercaseAToF = "a" / "b" / "c" / "d" / "e" / "f"
Uppercase = UppercaseAToF / "G" / "H" / "I" / "J" / "K" / "L" / "M" / "N" / "O" / "P" / "Q" / "R" / "S" / "T" / "U" / "V" / "W" / "X" / "Y" / "Z"
Lowercase = LowercaseAToF / "g" / "h" / "i" / "j" / "k" / "l" / "m" / "n" / "o" / "p" / "q" / "r" / "s" / "t" / "u" / "v" / "w" / "x" / "y" / "z"

QuoteEscape = "\\'" / "\\\""
EndOfLine = "\r\n" / '\n'
Space = " "

// Digits
BinDigit = "0" / "1"
OctDigit = BinDigit / "2" / "3" / "4" / "5" / "6" / "7"
DecDigit = OctDigit / "8" / "9"
HexDigit = DecDigit / UppercaseAToF / LowercaseAToF

// Comment
LineComment = "//" (!(EndOfLine) ?)*
```

### In MPL grammar
```rust ignore
// Hierarchical syntax
Mplg = ZeroOrMoreLines () / f
ZeroOrMoreLines = Line ZeroOrMoreLines / ()

Line = Line1 EndOfLine / f
Line1 = LineComment () / Line2
Line2 = Rule () / ()

Rule = Variable Rule1 / f
Rule1 = " = " Rule2 / f
Rule2 = E Rule3 / f
Rule3 = Space Rule4 / f
Rule4 = E Rule5 / f
Rule5 = " / " Rule6 / f
Rule6 = E () / f
E = TerminalSymbol () / Variable


// Lexical syntax
// Variable
Variable = Uppercase ZeroOrMoreVariableContinue / f
ZeroOrMoreVariableContinue =  VariableContinue ZeroOrMoreVariableContinue / ()
VariableContinue =  Alphabet () / DecDigit


// Terminal symbol
TerminalSymbol = Expr () / f

// Expr
Expr = LiteralExpr () / f

// Literal
LiteralExpr = MetasymbolLiteral () / StringLiteral

// Metasymbol
MetasymbolLiteral = EmptyLiteral () / MetasymbolLiteral1
MetasymbolLiteral1 = FailureLiteral () / MetasymbolLiteral2
MetasymbolLiteral2 = AnyLiteral () / MetasymbolLiteral3
MetasymbolLiteral3 = AllLiteral () / f
EmptyLiteral = "()" () / f
FailureLiteral = 'f' () / f
AnyLiteral = '?' ZeroOrMoreAny / f
ZeroOrMoreAny = '?' ZeroOrMoreAny / ()
AllLiteral = '*' () / f

// Original symbol

// String
StringLiteral = '"' StringLiteral1 / f
StringLiteral1 = InnerStringLiteral '"' / f
InnerStringLiteral = InnerStringLiteralLetter InnerStringLiteral / ()
// InnerStringLiteralLetter
InnerStringLiteralLetter = NotStringLetter InnerStringLiteral1Letter1 / f
NotStringLetter = '"' * / ()
InnerStringLiteral1Letter1 = QuoteEscape () / ?

// Letters
Alphabet = Lowercase () / Uppercase
// Lowercase
LowercaseAToF = LowercaseAToF1 () / f
LowercaseAToF1 = 'a' () / LowercaseAToF2
LowercaseAToF2 = 'b' () / LowercaseAToF3
LowercaseAToF3 = 'c' () / LowercaseAToF4
LowercaseAToF4 = 'd' () / LowercaseAToF5
LowercaseAToF5 = 'e' () / LowercaseAToF6
LowercaseAToF6 = 'f' () / f
Lowercase = LowercaseAToF () / Lowercase1
Lowercase1 = 'g' () / Lowercase2
Lowercase2 = 'h' () / Lowercase3
Lowercase3 = 'i' () / Lowercase4
Lowercase4 = 'j' () / Lowercase5
Lowercase5 = 'k' () / Lowercase6
Lowercase6 = 'l' () / Lowercase7
Lowercase7 = 'm' () / Lowercase8
Lowercase8 = 'n' () / Lowercase9
Lowercase9 = 'o' () / Lowercase10
Lowercase10 = 'p' () / Lowercase11
Lowercase11 = 'q' () / Lowercase12
Lowercase12 = 'r' () / Lowercase13
Lowercase13 = 's' () / Lowercase14
Lowercase14 = 't' () / Lowercase15
Lowercase15 = 'u' () / Lowercase16
Lowercase16 = 'v' () / Lowercase17
Lowercase17 = 'w' () / Lowercase18
Lowercase18 = 'x' () / Lowercase19
Lowercase19 = 'y' () / Lowercase20
Lowercase20 = 'z' () / f
// Uppercase
UppercaseAToF = UppercaseAToF1 () / f
UppercaseAToF1 = 'A' () / UppercaseAToF2
UppercaseAToF2 = 'B' () / UppercaseAToF3
UppercaseAToF3 = 'C' () / UppercaseAToF4
UppercaseAToF4 = 'D' () / UppercaseAToF5
UppercaseAToF5 = 'E' () / UppercaseAToF6
UppercaseAToF6 = 'F' () / f
Uppercase = UppercaseAToF () / Uppercase1
Uppercase1 = 'G' () / Uppercase2
Uppercase2 = 'H' () / Uppercase3
Uppercase3 = 'I' () / Uppercase4
Uppercase4 = 'J' () / Uppercase5
Uppercase5 = 'K' () / Uppercase6
Uppercase6 = 'L' () / Uppercase7
Uppercase7 = 'M' () / Uppercase8
Uppercase8 = 'N' () / Uppercase9
Uppercase9 = 'O' () / Uppercase10
Uppercase10 = 'P' () / Uppercase11
Uppercase11 = 'Q' () / Uppercase12
Uppercase12 = 'R' () / Uppercase13
Uppercase13 = 'S' () / Uppercase14
Uppercase14 = 'T' () / Uppercase15
Uppercase15 = 'U' () / Uppercase16
Uppercase16 = 'V' () / Uppercase17
Uppercase17 = 'W' () / Uppercase18
Uppercase18 = 'X' () / Uppercase19
Uppercase19 = 'Y' () / Uppercase20
Uppercase20 = 'Z' () / f

QuoteEscape = "\\'" () / "\\\""
EndOfLine = "\r\n" () / '\n'
Space = ' ' () / f

// Digits
BinDigit = "0" () / "1"
OctDigit = BinDigit () / OctDigit1
OctDigit1 = "2" () / OctDigit2
OctDigit2 = "3" () / OctDigit3
OctDigit3 = "4" () / OctDigit4
OctDigit4 = "5" () / OctDigit5
OctDigit5 = "6" () / OctDigit6
OctDigit6 = "7" () / f
DecDigit = OctDigit () / DecDigit1
DecDigit1 = "8" () / DecDigit2
DecDigit2 = "9" () / f

// Comment
LineComment = "//" InnerLineComment / f
InnerLineComment = AnyExceptLF InnerLineComment / ()
AnyExceptLF = AnyExceptLF1 ? / f
AnyExceptLF1 = EndOfLine * / ()
```

<!---
```rust ignore
// Hierarchical syntax
MPLG = (Line)*
Line = (Rule / LineComment / ()) EndOfLine
Rule = Variable " = " E Space E " / " E
E = TerminalSymbol / Variable

// Lexical syntax
// Variable
Variable = Uppercase (Alphabet / DecDigit)*

// Terminal symbol
TerminalSymbol = Expr

// Expr
Expr = LiteralExpr / ArrayExpr

// Array
ArrayExpr = "[" ArrayElements? "]"
ArrayElements = Expr ("," Expr)* ","? / Expr ";" Expr

// Literal
LiteralExpr = MetasymbolLiteral / StringLiteral / IntegerLiteral
// LiteralExpr = MetasymbolLiteral / CharLiteral / StringLiteral / IntegerLiteral / FloatLiteral

// Metasymbol
MetasymbolLiteral = EmptyLiteral / FailureLiteral / AnyLiteral / AllLiteral
EmptyLiteral = "()" () / f
FailureLiteral = 'f' () / f
AnyLiteral = '?' () / f
AllLiteral = '*' () / f

// String
// TODO Multibyte character may not work.
StringLiteral = "\"" (!("\"" / "\\" / IsolatedCR) . / QuoteEscape / ASCIIEscape / UnicodeEscape / StringContinue)* "\""
StringContinue = "\\" &"\n" 

// Char
// CharLiteral = "\'" (!("\'" / "\n" / "\r" / "\t") . / QuoteEscape / ASCIIEscape / UnicodeEscape) '\''
QuoteEscape = "\\'" / "\\\""
ASCIIEscape = "\\x" OctDigit HexDigit / "\\n" / "\\r" / "\\t" / "\\\\" / "\\0"
UnicodeEscape = "\\u{" (HexDigit "_"*)^1..6 "}"

// Integer
IntegerLiteral = (DecLiteral / HexLiteral) IntegerSuffix?
DecLiteral = DecDigit (DecDigit / "_")*
HexLiteral =  "0x" (HexDigit / "_")* HexDigit (HexDigit / "_")*

IntegerSuffix = "u8" / "u16" / "u32" / "u64" / "u128" / "usize" / "i8" / "i16" / "i32" / "i64" / "i128" / "isize"

// Float
FloatLiteral = DecLiteral "." ()

FloatExponent = ("e" / "E") ("+" / "-")? (DecDigit / "_")* DecDigit (DecDigit / "_")*
FloatSuffix = "f32" / "f64"


// Letters
UppercaseAToF = "A" / "B" / "C" / "D" / "E" / "F"
LowercaseAToF = "a" / "b" / "c" / "d" / "e" / "f"
Uppercase = UppercaseAToF / "G" / "H" / "I" / "J" / "K" / "L" / "M" / "N" / "O" / "P" / "Q" / "R" / "S" / "T" / "U" / "V" / "W" / "X" / "Y" / "Z"
Lowercase = LowercaseAToF / "g" / "h" / "i" / "j" / "k" / "l" / "m" / "n" / "o" / "p" / "q" / "r" / "s" / "t" / "u" / "v" / "w" / "x" / "y" / "z"
Alphabet = Uppercase / Lowercase

// Digits
BinDigit = "0" / "1"
OctDigit = BinDigit / "2" / "3" / "4" / "5" / "6" / "7"
DecDigit = OctDigit / "8" / "9"
HexDigit = DecDigit / UppercaseAToF / LowercaseAToF

// Comment
LineComment = "//" (!("\n") .)*

IsolatedCR = "\r" !"\n" ()
// TODO: Maybe need EOF
EndOfLine = "\n" / "\r\n"
Space = " "
```

--->

## TODO

### Tasks
- into_first() in CST

### 
- Add { Original } in mplg
- Add functions that easy to get Variable from AST
- Add RowColSpan
- Create parser from MPLG file.
- Error Handling
- Packrat Parsing
- Left Recursion

## Next implementation
- [ ] Add functions that easy to get Variable from AST
- [ ] Can be Variable in Leaf Node
- [ ] Error Handling

## Practice
### Sequence
`A <- e1 e2`
```rust ignore
A = e1 e2 / f
```

### Choice
`A <- e1 / e2`
```rust ignore
A = e1 () / e2
```

### Zero or more
`A <- e*`
```rust ignore
A = e A / ()
```

### Not predicate
`A <- !e ?`
```rust ignore
A = B ? / f
B = e * / ()
```

## References
- Alexander Birman. The TMG Recognition Schema. PhD thesis, Princeton University, February 1970
- Alfred V. Aho and Jeffrey D. Ullman. The Theory of Parsing, Translation and Compiling - Vol. I: Parsing. Prentice Hall, Englewood Cliffs, N.J., 1972.
- Bryan Ford. 2002. Packrat parsing: a practical linear-time algorithm with backtracking. Ph.D. Dissertation. Massachusetts Institute of Technology.
- Bryan Ford. 2004. Parsing expression grammars: a recognition-based syntactic foundation. In Proceedings of the 31st ACM SIGPLAN-SIGACT symposium on Principles of programming languages. 111–122.
- Hutchison, Luke AD. "Pika parsing: reformulating packrat parsing as a dynamic programming algorithm solves the left recursion and error recovery problems." arXiv preprint arXiv:2005.06444 (2020).
