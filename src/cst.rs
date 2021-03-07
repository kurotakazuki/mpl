use crate::span::Spanned;
use crate::symbols::TerminalSymbol;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CstKind<T, V, S, L> {
    Item(TerminalSymbol<T>),
    FirstChoice { variable: V, left: Box<Cst<T, V, S, L>>, right: Box<Cst<T, V, S, L>> },
    SecondChoice { variable: V, e: Box<Cst<T, V, S, L>> },
}

type Cst<T, V, S, L> = Spanned<CstKind<T, V, S, L>, S, L>;

#[cfg(test)]
mod tests {
    use crate::span::ByteSpan;
    use super::*;

    #[test]
    fn cst() {
        #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
        enum DigitTerminal {
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

        impl DigitTerminal {
            fn into<I: From<String>>(&self) -> I {
                match *self {
                    Self::Zero => "0".to_string().into(),
                    Self::One => "1",
                    Self::Two => "2",
                    Self::Three => "3",
                    Self::Four => "4",
                    Self::Five => "5",
                    Self::Six => "6",
                    Self::Seven => "7",
                    Self::Eight => "8",
                    Self::Nine => "9",
                }
            }
        }

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