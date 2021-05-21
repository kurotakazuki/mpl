pub trait Position: Clone + PartialOrd {}

macro_rules! position_impl {
    ( $( $t:ty ),* ) => ($(
        impl Position for $t {}
    )*)
}

position_impl!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
