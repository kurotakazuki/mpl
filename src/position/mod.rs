pub use self::byte_pos::BytePos;

// use crate::input::Input;
// use crate::span::Span;

mod byte_pos;

pub trait Position: Clone + PartialOrd {
    fn with_one_added(&self) -> Self;
}


// pub trait Position: Clone + PartialOrd {
//     fn add_with_input<I: Input>(&self, rhs: usize, input: &I) -> Self;
// }
