use crate::cst::CST;
use crate::span::Span;
use crate::rules::Rules;
use std::collections::HashMap;
// use crate::rules::Rule;

// pub fn parse_from<T, V, S, L, I>(input: I, rules: <Rule<T, V>>, start: V) -> Result<CST<T, V, S>, ()> {
    
//     // expr(input, )

//     unimplemented!()
// }

// fn expr<T, V, S, L, I>(v: V, input: I) -> Result<CST<T, V, S, L>, ()> {
    
// }

/// The type that implements this trait is used to check if the input and terminal symbol match.
pub trait CheckingSlice {}

/// This is terminal.
/// I is input.
pub trait Terminal<I> where I: CheckingSlice {
    fn into_checking_slice(&self) -> I;
}

pub trait ParseInput {
    
}

/// T is (enum of) Terminal type.
/// V is (enum of) Variable.
/// S is span.start.
/// L is span.len.
pub trait Parse<T, V, S, L> {
    fn parse(&self, rules: Rules<T, V>, start: Option<V>) -> Result<CST<T, V, S, L>, ()> {
        

        Err(())
    }
    fn eval(&self);
}

pub trait ParseSlice<S> {
    type Slice;
    type Span;

    fn parse_slice(&self, span: S) -> Self::Slice;
}



