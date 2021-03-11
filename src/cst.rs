use crate::span::Spanned;
use crate::symbols::TerminalSymbol;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CSTKind<T, V, S, L> {
    T(TerminalSymbol<T>),
    // This includes Variable, Left Child Node and Right Child Node.
    FirstChoice { v: V, lcn: Box<CST<T, V, S, L>>, rcn: Box<CST<T, V, S, L>> },
    // This includes Variable and Child Node.
    SecondChoice { v: V, cn: Box<CST<T, V, S, L>> },
}

type CST<T, V, S, L> = Spanned<CSTKind<T, V, S, L>, S, L>;

// pub struct NodeKind<T, V, S, L> {
//     choice: Choice<T, V>,
//     node: Box<Node<T, V, S, L>>,
// }
// type Node<T, V, S, L> = Spanned<NodeKind<T, V, S, L>, S, L>;


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

        // impl DigitTerminal {
        //     fn into<T, I: From<T>>(&self) -> I {
        //         match *self {
        //             Self::Zero => "0".into(),
        //             Self::One => "1",
        //             Self::Two => "2",
        //             Self::Three => "3",
        //             Self::Four => "4",
        //             Self::Five => "5",
        //             Self::Six => "6",
        //             Self::Seven => "7",
        //             Self::Eight => "8",
        //             Self::Nine => "9",
        //         }
        //     }
        // }

        #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
        enum Variable {
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