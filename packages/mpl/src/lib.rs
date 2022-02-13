//! # Minimal Parsing Language (MPL)
//! 
//! [![Crate](https://img.shields.io/crates/v/mpl.svg)](https://crates.io/crates/mpl)
//! [![API](https://docs.rs/mpl/badge.svg)](https://docs.rs/mpl)
//! 
//! This is minimal parser combinator of Minimal Parsing Language (MPL) like Top-Down Parsing Language (TDPL). It creates a abstract syntax tree (AST) for each input.
//! 
//! ## Getting Started
//! 1. implement `Variable`
//! 2. insert each rule into `HashMap`
//! 3. `minimal_parse()`
//! 
//! - Optional
//!     - implement `Input`
//!         - supports `[T]` and `str` by default
//!     - implement `Position`
//!         - supports `u*`, `i*`, and `f*` by default
//!     - implement `Span`
//!         - supports `StartAndLenSpan` by default
//!     - implement `Terminal`
//!         - supports `SliceTerminal`, `StrTerminal`, and `U8SliceTerminal` by default
//!     - implement `Output`
//!         - supports `()` by default
//!     - implement `Rules`
//!         - supports `HashMap` by default
//!     - implement `Parse`
//!         - supports `[T]`, `str`, and `[u8]` by default
//! 
//! ### Example
//! ```rust
//! use crate::ParenthesesVariable::*;
//! use mpl::parser::Parser;
//! use mpl::rules::{RightRule, RightRuleKind::*, Rules};
//! use mpl::span::{StartAndLenSpan, Start, Len};
//! use mpl::output::Output;
//! use mpl::symbols::{StrTerminal, StrTerminal::*, Variable};
//! use mpl::trees::AST;
//! use std::collections::HashMap;
//! 
//! #[derive(Clone, Debug, Hash, Eq, PartialEq)]
//! enum ParenthesesVariable {
//!     Open,
//!     Parentheses,
//!     Close,
//! }
//! 
//! impl Variable for ParenthesesVariable {}
//! 
//! struct ParenthesesParser;
//! 
//! impl<'i, V, P, L, R, O> Parser<'i, str, StrTerminal<'i>, V, StartAndLenSpan<P, L>, P, R, O>
//!     for ParenthesesParser
//! where
//!     V: Variable,
//!     P: Start<str, L>,
//!     L: Len<str, P>,
//!     R: Rules<StrTerminal<'i>, V>,
//!     O: Output<'i, str, V, StartAndLenSpan<P, L>>,
//! {
//! }
//! 
//! /// ```
//! /// Open = '(' Parentheses / ()
//! /// Parentheses = Open Close / f
//! /// Close = ")" Open / f
//! /// ```
//! fn main() {
//!     let parser = ParenthesesParser;
//!     let mut rules = HashMap::new();
//! 
//!     rules.insert(
//!         Open,
//!         RightRule::from_right_rule_kind((T(Char('(')), V(Parentheses)), Empty),
//!     );
//!     rules.insert(
//!         Parentheses,
//!         RightRule::from_right_rule_kind((V(Open), V(Close)), Failure),
//!     );
//!     rules.insert(
//!         Close,
//!         RightRule::from_right_rule_kind((T(Str(")")), V(Open)), Failure),
//!     );
//! 
//!     let input = "(()(()))";
//! 
//!     // all of the span
//!     let all_of_the_span = StartAndLenSpan::<u32, u16>::from_start_len(0, input.len() as u16);
//! 
//!     let result: Result<
//!         AST<ParenthesesVariable, StartAndLenSpan<u32, u16>, ()>,
//!         AST<ParenthesesVariable, StartAndLenSpan<u32, u16>, ()>,
//!     > = parser.parse(input, &rules, &Open, &all_of_the_span);
//! 
//!     if let Ok(ast) = result {
//!         println!("{}", ast);
//!     }
//! }
//! ```
//! 
//! ### Test Examples
//! - [Number](tests/number.rs)
//! - [Parentheses](tests/parentheses.rs)
//! - [Wav Riff](tests/wav_riff.rs)
//! 
//! ### Parsers written with MPL
//! - [WAV AST](https://github.com/kurotakazuki/wav_ast) : RIFF waveform Audio Format
//! 
//! ## MPL
//! ### Definition of MPL grammar
//! A MPL grammar `G` is a tuple `G = (V, Σ, R, S)` in which:
//! - `V` is a finite set of variables.
//! - `Σ` is a finite set of original terminal symbols.
//! - `T` is an union of `Σ` or `M` (Σ &cup; M) (`M` (= {(), f}) is a finite set of metasymbols).
//! - `R` is a finite set of rules of the form
//!     - `A = B C / D`  
//!     A in V (A &isin; V),  
//!     B, C, D in E (E = T &cup; V) (T &cap; V = &empty;) (B, C, D &isin; E).  
//!     For any variable A there is exactly one rule with A to the left of `=`.
//! - S in V (S &isin; V) is the start variable.
//! 
//! #### Empty
//! `()` is a metasymbol that always succeeds without consuming input.
//! 
//! ```rust ignore
//! Empty = () () / ()
//! ```
//! 
//! #### Failure
//! `f` is a metasymbol that always fails without consuming input.
//! 
//! ```rust ignore
//! Failure = f f / f
//! ```
//! 
//! ### Extended MPL
//! Since one of the goals of MPL is to create an AST, it also supports two features in terms of ease of use and speed.
//! 
//! #### Any
//! `?` is a metasymbol representing any single input like wildcard character. This succeeds if there is any input left, and fails if there is no input left.
//! 
//! ```rust ignore
//! Any = ? () / f
//! ```
//! 
//! To extend the difinition of MPL grammar, let ? &isin; M.
//! 
//! #### All
//! `*` is a metasymbol representing All remaining input like wildcard character. This will succeed even if the remaining inputs are zero.
//! 
//! ```rust ignore
//! All = * () / f
//! ```
//! 
//! Same as `All = ? All / ()`.
//! 
//! To extend the difinition of MPL grammar, let * &isin; M.

pub mod choices;
pub mod input;
pub mod output;
pub mod parser;
pub mod position;
pub mod rules;
pub mod span;
pub mod symbols;
pub mod trees;
