#![allow(unused_variables)]

trait Axis {}

trait Coordinate {
    // fn cardinals(&self) -> Vec<Self> where Self: Sized;
    // fn neighbors(&self) -> Vec<Self> where Self: Sized;
    fn rotate(&self, by: Self) -> Self;
}

struct D2<T> {
    x: T,
    y: T,
}

impl<T> Coordinate for D2<T> {
    fn rotate(&self, by: Self) -> Self {
        todo!()
    }
}

struct D3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Coordinate for D3<T> {
    fn rotate(&self, by: Self) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }
}
