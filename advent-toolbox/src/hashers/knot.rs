use itertools::Itertools;

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

    pub fn round(&mut self) {
        for _ in 0..64 {
            self.next();
        }
    }

    pub fn new_from_str(ring_size: usize, input: &str) -> Self {
        let mut lengths: Vec<usize> = input
            .trim()
            .as_bytes()
            .iter()
            .cloned()
            .map(|b| b.into())
            .collect();
        lengths.extend(vec![17, 31, 73, 47, 23]);
        Self::new(ring_size, lengths)
    }

    pub fn dense_hash(&self) -> Vec<usize> {
        self.ring
            .chunks(16)
            .map(|ch| ch.iter().cloned().reduce(|a, b| a ^ b).unwrap())
            .collect()
    }

    pub fn as_hex_str(&self) -> String {
        self.dense_hash()
            .iter()
            .map(|ch| format!("{:02x}", ch))
            .join("")
    }

    pub fn as_bin_str(&self) -> String {
        convert_to_binary_from_hex(&self.as_hex_str())
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

fn convert_to_binary_from_hex(hex: &str) -> String {
    hex.chars().map(to_binary).collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' | 'a' => "1010",
        'B' | 'b' => "1011",
        'C' | 'c' => "1100",
        'D' | 'd' => "1101",
        'E' | 'e' => "1110",
        'F' | 'f' => "1111",
        _ => "",
    }
}
