use crate::rules::Choice;
use crate::span::{Span, Spanned};
use crate::symbols::{TerminalSymbol, MPGGTerminalType, Variable};

type LeafNode<T> = TerminalSymbol<T>;
type InternalNode<T, V, S, L> = Variable<V, Box<Choice<CST<T, V, S, L>>>>;

#[derive(Clone, Debug, PartialEq)]
pub enum CSTKind<T, V, S, L> {
    /// Terminal symbol
    LeafNode(LeafNode<T>),
    /// Viriable
    InternalNode(InternalNode<T, V, S, L>),
    // InternalNode { variable: V, choice: Box<Choice<CST<T, V, S, L>>> },
}

type CST<T, V, S, L> = Spanned<CSTKind<T, V, S, L>, S, L>;

impl<T, V, S, L> CST<T, V, S, L> {
    pub fn leaf_node(leaf_node: LeafNode<T>, span: Span<S, L>) -> Self {
        Self::new(CSTKind::LeafNode(leaf_node), span)
    }

    pub fn internal_node(internal_node: InternalNode<T, V, S, L>, span: Span<S, L>) -> Self {
        Self::new(CSTKind::InternalNode(internal_node), span)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The following syntax is a lexical syntax for numbers.
    /// ```
    /// Number: String = digit numeral / f
    /// Numeral = digit numeral / ()
    /// Digit = zero () / f
    /// Zero = "0" () / one
    /// One = "1" () / two
    /// // ...
    /// Nine = "9" () / f
    /// ```
    #[test]
    fn number_cst() {
        #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
        enum NumberTerminal {
            ZeroFC0,
            OneFC0,
            TwoFC0,
            ThreeFC0,
            FourFC0,
            FiveFC0,
            SixFC0,
            SevenFC0,
            EightFC0,
            NineFC0,
        }

        // trait TerminalAndInput<L, I> {
        //     fn into_input(&self) -> (L, I);
        // }

        impl NumberTerminal {
            fn into_terminal_type(&self) -> MPGGTerminalType {
                match *self {
                    Self::ZeroFC0 => "0".into(),
                    Self::OneFC0 => "1".into(),
                    Self::TwoFC0 => '2'.into(),
                    Self::ThreeFC0 => "3".into(),
                    Self::FourFC0 => "4".into(),
                    Self::FiveFC0 => "5".into(),
                    Self::SixFC0 => "6".into(),
                    Self::SevenFC0 => "7".into(),
                    Self::EightFC0 => "8".into(),
                    Self::NineFC0 => [96, 1][..].into(),
                }
            }
        }

        #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
        enum NumberVariable {
            Number,
            Numeral,
            Digit,
            Zero,
            One,
            Two,
            Three,
            Four,
            Five,
            Six,
            Seven,
            Eight,
            Nine,
        }
    }
}