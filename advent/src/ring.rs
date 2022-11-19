// modular rings? there's something to generalize here
// check out 2016 day 15

pub trait Ring {
    fn activated(&self) -> bool;
}

pub trait Rings {
    type Item;
}
