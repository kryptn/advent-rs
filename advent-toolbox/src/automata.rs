use crate::spatial::{Point, Space};

pub trait Automata {
    type State;

    fn next_state(&self) -> Self::State;
}

impl<P, T> Automata for Space<P, T>
where
    P: Point,
{
    type State = Self;

    fn next_state(&self) -> Self::State {
        todo!()
    }
}
