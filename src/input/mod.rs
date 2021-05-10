mod slice;
mod str;

pub trait Input<S>
{
    /// This method represents the start and length of the input.
    /// Recommended span.start is 0.
    fn all_of_the_span(&self) -> S;
}
