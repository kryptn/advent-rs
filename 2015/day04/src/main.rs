use advent::fetch;

fn mine(input: String, pat: &str) -> Option<i32> {
    let mut x: i32 = 1;
    loop {
        let digest = md5::compute(format!("{}{}", &input, x));
        let digest_str = format!("{:x}", digest);
        if digest_str.starts_with(pat) {
            return Some(x);
        }
        x += 1;
    }

    None
}

fn main() {
    let input = fetch::get_input(2015, 4);
    let input = input.trim().to_string();

    println!("part 1 = {}", mine(input.clone(), "00000").unwrap());
    println!("part 2 = {}", mine(input.clone(), "000000").unwrap());
    // println!("part 2 = {} houses", deliver(2, input.clone()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given() {
        assert_eq!(mine(String::from("abcdef"), "00000"), Some(609043));
        assert_eq!(mine(String::from("pqrstuv"), "00000"), Some(1048970));
    }
}
