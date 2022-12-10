use std::{collections::HashMap, hash::Hash};

trait Point {}

#[derive(Debug)]
struct Space<P, T>(HashMap<P, T>)
where
    P: Point;

impl<P: Point, T> std::ops::Deref for Space<P, T> {
    type Target = HashMap<P, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<P: Point, T> std::ops::DerefMut for Space<P, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<P: Point, T> Space<P, T> {
    fn new() -> Self {
        Space(HashMap::new())
    }
}

impl Point for (i32, i32) {}

impl<K: Point, V> FromIterator<(K, V)> for Space<K, V>
where
    K: Eq + Hash,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Space<K, V> {
        let mut map = HashMap::with_hasher(Default::default());
        map.extend(iter);
        Self(map)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn create_grid() {
        let objects: Vec<((i32, i32), bool)> = vec![((1, 1), false)];
        let g: Space<(i32, i32), bool> = objects.into_iter().collect();
        dbg!(g);
    }
}
