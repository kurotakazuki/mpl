# Minimal Parser Generator
This is minimal parser generator that generates a parser from grammar like Top-Down Parsing Language (TDPL). The generated parser creates a concrete syntax tree (CST) for each input.

## MPG
### Definition of MPG grammar
A MPG grammar `G` is a tuple `G = (V, T, R, S, $)` in which:
- `V` is a finite set of variables.
- `T` is a finite set of terminal symbols containing `M` ( = {(), f}) (is a finite set of metasymbols).
- `R` is a finite set of rules of the form
    - `A = B C / D`  
    A in V (A &isin; V),  
    B, C, D in E (E = T &cup; V) (T &cap; V = &empty;) (B, C, D &isin; E).  
    For any variable A there is at most one rule with A to the left of the `=`.
- S in V (S &isin; V) is the start variable.
- $ not in E ($ &notin; E) is the end symbol.

### Extended MPG
Since one of the goals of MPG is to create an CST, it also supports two features in terms of ease of use and speed.

#### Any
`?` is a metasymbol representing any single input like wildcard character. This succeeds if there is any input left, and fails if there is no input left.

```
Integer = ???? () / f
```

To extend the difinition of MPG grammar, change `M` = {(), f} to `M` = {?, (), f}.

<!---
#### Variable type
Variables can have a type.

If the variable contains a type, it will include the value of that type, such as a token, when the CST is created. Therefore rules decomposed from variable including rule has a role like lexical analysis. The following syntax is a lexical syntax for numbers.

```
Number: String = digit numeral / f
Numeral = digit numeral / ()
Digit = zero () / f
Zero = "0" () / one
One = "1" () / two
// ...
Nine = "9" () / f
```

An error (TODO: maybe failure would be better) will occur if the input cannot be converted to the variable type.

To extend the difinition of MPG grammar, change `A = B C / D` to `A = B C / D` or `A: TYPE = B C / D`, or seperate definition of `V` by including type or not.
--->


#### Terminal symbol type
Terminal symbols supports several types.

```
A = 'A' "abc" / [0, 0, 0]
```

Supports `char, str, [u8]` at this moment.

## Difference between TDPL and MPG
The biggest difference between the two grammars is the rule form. There are two rule forms in TDPL.

> `A..BC/D`, A,B,C,D in V.  
> `A..a`, a in &sum; &cup; {&epsilon;, f}, f is a metasymbol not in &sum; and &epsilon; is the null string.

MPG, on the other hand, has one rule form.

## MPGG (MPG Grammar) syntax
### In PEG like grammar
```rust
// Hierarchical syntax
MPGG = (Line)*
Line = (Rule / LineComment / ()) EndOfLine
Rule = Variable '=' E Space E Space '/' Space E
E = Variable / TerminalSymbol

// Lexical syntax
// Variable
Variable = Uppercase (Alphabet / DecDigit)*

// Terminal symbol
TerminalSymbol = Expression

// Expression
Expression = LiteralExpression / ArrayExpression

// Array
ArrayExpression = '[' ArrayElements? ']'
ArrayElements = Expression (',' Expression)* ','? / Expression ';' Expression

// Literal
LiteralExpression = CharLiteral / StringLiteral / IntegerLiteral / FloatLiteral

// String
StringLiteral = '\"' ((!('\"' / '\\' / IsolatedCR) . / QuoteEscape / ASCIIEscape / UnicodeEscape / StringContinue)* '\"'
StringContinue = '\\' &'\n' 

// Char
CharLiteral = '\'' (!('\'' / '\n' / '\r' / '\t') . / QuoteEscape / ASCIIEscape / UnicodeEscape) '\''
QuoteEscape = "\\'" / "\\\""
ASCIIEscape = "\\x" OctDigit HexDigit / "\\n" / "\\r" / "\\t" / "\\\\" / "\\0"
UnicodeEscape = "\\u{" (HexDigit '_'*)^1..6 '}'

// Integer
IntegerLiteral = (DecLiteral / HexLiteral) IntegerSuffix?
DecLiteral = DecDigit (DecDigit / '_')*
HexLiteral =  "0x" (HexDigit / '_')* HexDigit (HexDigit / '_')*

IntegerSuffix = "u8" / "u16" / "u32" / "u64" / "u128" / "usize" / "i8" / "i16" / "i32" / "i64" / "i128" / "isize"

// Float
FloatLiteral = DecLiteral '.' ()

FloatExponent = ('e' / 'E') ('+' / '-')? (DecDigit / '_')* DecDigit (DecDigit / '_')*
FloatSuffix = "f32" / "f64"


// Letters
UppercaseAToF = 'A' / 'B' / 'C' / 'D' / 'E' / 'F'
LowercaseAToF = 'a' / 'b' / 'c' / 'd' / 'e' / 'f'
Uppercase = UppercaseAToF / 'G' / 'H' / 'I' / 'J' / 'K' / 'L' / 'M' / 'N' / 'O' / 'P' / 'Q' / 'R' / 'S' / 'T' / 'U' / 'V' / 'W' / 'X' / 'Y' / 'Z'
Lowercase = LowercaseAToF / 'g' / 'h' / 'i' / 'j' / 'k' / 'l' / 'm' / 'n' / 'o' / 'p' / 'q' / 'r' / 's' / 't' / 'u' / 'v' / 'w' / 'x' / 'y' / 'z'
Alphabet = Uppercase / Lowercase

// Digits
BinDigit = '0' / '1'
OctDigit = BinDigit / '2' / '3' / '4' / '5' / '6' / '7'
DecDigit = OctDigit / '8' / '9'
HexDigit = DecDigit / UppercaseAToF / LowercaseAToF

// Comment
LineComment = "//" (!('\n') .)*

IsolatedCR = '\r' !'\n' ()
// TODO: Maybe need EOF
EndOfLine = '\n' / "\r\n"
Space = ' '
```

### In MPG grammar


## References
These are references that I read. (I may have misunderstood the content because I haven't read some reference completely or have no reading comprehension. Please let me know if there are any mistakes.)

- Alexander Birman. The TMG Recognition Schema. PhD thesis, Princeton University, February 1970
- Alfred V. Aho and Jeffrey D. Ullman. The Theory of Parsing, Translation and Compiling - Vol. I: Parsing. Prentice Hall, Englewood Cliffs, N.J., 1972.
- Bryan Ford. 2002. Packrat parsing: a practical linear-time algorithm with backtracking. Ph.D. Dissertation. Massachusetts Institute of Technology.
- Bryan Ford. 2004. Parsing expression grammars: a recognition-based syntactic foundation. In Proceedings of the 31st ACM SIGPLAN-SIGACT symposium on Principles of programming languages. 111–122.
- Hutchison, Luke AD. "Pika parsing: reformulating packrat parsing as a dynamic programming algorithm solves the left recursion and error recovery problems." arXiv preprint arXiv:2005.06444 (2020).
