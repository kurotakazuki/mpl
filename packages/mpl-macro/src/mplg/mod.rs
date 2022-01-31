use mpl::span::StartAndLenSpan;
use mpl::trees::AST;

pub use self::output::MplgOutput;
pub use self::rules::MplgRules;
pub use self::variables::MplgVariables;

mod output;
mod parse;
mod rules;
mod variables;

pub type MplgAST<'input> = AST<MplgVariables, StartAndLenSpan<u32, u32>, MplgOutput<'input>>;
