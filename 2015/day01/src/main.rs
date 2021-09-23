use advent::fetch;

fn main() {
    let input = fetch::get_input(2015, 1);

    let mut floor = 0;
    let mut hit_basement = false;

    for (i, chr) in input.chars().enumerate() {
        match chr {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => unreachable!(),
        };

        if floor < 0 && !hit_basement {
            println!("hit basement on floor {}", i + 1);
            hit_basement = true;
        }
    }

    println!("ended on floor {}", floor);
}
