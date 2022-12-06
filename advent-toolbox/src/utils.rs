pub fn neighbors((x, y): (u32, u32)) -> [(u32, u32); 8] {
    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        /*(x, y),*/ (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}

pub fn adjacent((x, y): (u32, u32)) -> [(u32, u32); 4] {
    [(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)]
}
