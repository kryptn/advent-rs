fn step(position: usize, length: usize, ring: &mut Vec<usize>) {
    // dbg!(position, length, &ring);
    ring.rotate_left(position);

    let left = &mut ring[0..length];
    left.reverse();

    ring.rotate_right(position);
}

pub struct KnotHasher {
    pub ring: Vec<usize>,
    pub lengths: Vec<usize>,
    pub position: usize,
    pub skip: usize,
}

impl KnotHasher {
    pub fn new(ring_size: usize, lengths: Vec<usize>) -> Self {
        let ring = (0..ring_size).into_iter().collect();

        Self {
            ring,
            lengths,
            position: 0,
            skip: 0,
        }
    }
}

impl Iterator for KnotHasher {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        for length in self.lengths.iter() {
            step(self.position, *length, &mut self.ring);
            self.position = (self.position + length + self.skip) % self.ring.len();
            self.skip += 1;
        }
        Some(self.ring.clone())
    }
}
