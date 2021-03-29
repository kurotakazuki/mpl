pub use self::byte_pos::BytePos;

mod byte_pos;

pub trait Position: Clone + PartialOrd {
    fn with_one_added(&self) -> Self;
}
