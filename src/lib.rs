//! # Minimal Parsing Language (MPL)
//! This is minimal parser combinator of Minimal Parsing Language (MPL) like Top-Down Parsing Language (TDPL). It creates a abstract syntax tree (AST) for each input.
//!
//! ## Getting Started
//! 1. implement [`symbols::Variable`]
//! 2. insert each rule into `HashMap`
//! 3. [`parse::Parse::minimal_parse()`]
//!
//! - Optional
//!     - implement [`input::Input`]
//!         - supports `[T]` and `str` by default
//!     - implement [`position::Position`]
//!         - supports `u*`, `i*`, and `f*` by default
//!     - implement [`span::Span`]
//!         - supports `StartAndLenSpan` by default
//!     - implement [`symbols::Terminal`]
//!         - supports `SliceTerminal`, `StrTerminal`, and `U8SliceTerminal` by default
//!     - implement [`output::Output`]
//!         - supports `()` by default
//!     - implement [`rules::Rules`]
//!         - supports `HashMap` by default
//!     - implement [`parse::Parse`]
//!         - supports `[T]`, `str`, and `[u8]` by default
//!
//! ### Example
//! ```rust
//! use crate::ParenthesesVariable::*;
//! use mpl::parse::Parse;
//! use mpl::rules::{RightRule, RightRuleKind::*};
//! use mpl::span::StartAndLenSpan;
//! use mpl::symbols::{StrTerminal::*, Variable};
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
//! /// ```
//! /// Open = '(' Parentheses / ()
//! /// Parentheses = Open Close / f
//! /// Close = ")" Open / f
//! /// ```
//! fn main() {
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
//!     > = input.minimal_parse(&rules, &Open, &all_of_the_span);
//!
//!     println!("{:#?}", result);
//! }
//! ```

pub mod choices;
pub mod input;
pub mod output;
pub mod parse;
pub mod position;
pub mod rules;
pub mod span;
pub mod symbols;
pub mod trees;
