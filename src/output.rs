pub trait Output<'input, I: ?Sized, V, S>: Sized {
    fn new(input: &'input I, variable: V, span: S) -> Option<Self>;
}
