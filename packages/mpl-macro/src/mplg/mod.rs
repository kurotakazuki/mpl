use mpl::span::StartAndLenSpan;
use mpl::trees::AST;

pub use self::output::MplgOutput;
pub use self::parser::parse_mplg;
pub use self::rules::MplgRules;
pub use self::variables::MplgVariables;

mod output;
mod parser;
mod rules;
mod variables;

pub type MplgAST<'input> = AST<MplgVariables, StartAndLenSpan<u32, u32>, MplgOutput<'input>>;
