mod str;

pub trait Input<'input, S>
where
    Self: 'input,
{
    /// This method represents the start and length of the input.
    /// Recommended span.start is 0.
    fn all_of_the_span(&'input self) -> S;
}
