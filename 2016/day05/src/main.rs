use advent::input_store;

struct BruteForce {
    door_id: String,
    salt: i32,
}

impl BruteForce {
    fn new(door_id: String) -> Self {
        Self { door_id, salt: 0 }
    }
}

impl Iterator for BruteForce {
    type Item = (char, char);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let input = format!("{}{}", self.door_id, self.salt);
            let digest = md5::compute(input);
            let hash = format!("{:x}", digest);
            self.salt += 1;
            if hash.starts_with("00000") {
                return Some((hash.chars().nth(5).unwrap(), hash.chars().nth(6).unwrap()));
            }
        }
    }
}

fn find_pw(bf: BruteForce) -> String {
    bf.into_iter()
        .take(8)
        .map(|(chr, _)| chr)
        .collect::<String>()
}

fn find_pw_pt2(mut bf: BruteForce) -> String {
    let mut pw: [Option<char>; 8] = [None, None, None, None, None, None, None, None];

    while pw.iter().any(|&f| f == None) {
        let (index, chr) = bf.next().unwrap();

        if let Some(idx) = index.to_digit(10) {
            if idx < 8 {
                let value = pw.get_mut(idx as usize).unwrap();
                if *value == None {
                    //println!("on salt {} -> idx: {}, chr: {}", bf.salt, idx, chr);
                    *value = Some(chr);
                }

                // unsafe {
                //     let value = pw.get_unchecked_mut(idx as usize);
                //     if *value == None {
                //         *value = Some(chr);
                //     }
                // }
            }
        }
    }

    let pw: String = pw.iter().map(|&cho| cho.unwrap()).collect();
    pw
}

fn main() {
    let input = input_store::get_input(2016, 5).trim().to_string();
    dbg!(&input);

    let bf = BruteForce::new(input.clone());
    let pw = find_pw(bf);
    println!("part 1 => {}", pw);

    let bf = BruteForce::new(input);
    let pw = find_pw_pt2(bf);
    println!("part 2 => {}", pw);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn p1_tests() {
        let bf = BruteForce::new(String::from("abc"));
        let pw = find_pw(bf);

        assert_eq!(pw, String::from("18f47a30"));
    }

    #[test]
    fn p2_tests() {
        let bf = BruteForce::new(String::from("abc"));
        let pw = find_pw_pt2(bf);

        assert_eq!(pw, String::from("05ace8e3"));
    }
}
