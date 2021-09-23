use advent::fetch;
use fancy_regex::Regex;

fn nice(given: &str) -> bool {
    let double = Regex::new(r"(.)\1").unwrap();
    let three_vowels = Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap();

    let forbidden = Regex::new(r"(ab|cd|pq|xy)").unwrap();

    double.is_match(given).unwrap()
        && three_vowels.is_match(given).unwrap()
        && !forbidden.is_match(given).unwrap()
}

fn new_nice(given: &str) -> bool {
    let double_pair = Regex::new(r"(.)(.).*\1\2").expect("double_pair should be ok");
    let straddles = Regex::new(r"(.).\1").expect("straddles should be ok");

    double_pair.is_match(given).unwrap() && straddles.is_match(given).unwrap()
}

fn main() {
    let input = fetch::get_input(2015, 5);
    let input = input.trim().to_string();

    let nice_strings: Vec<&str> = input.lines().filter(|l| nice(l)).collect();
    println!("part 1 = {}", nice_strings.len());

    let nicer_strings: Vec<&str> = input.lines().filter(|l| new_nice(l)).collect();
    println!("part 2 = {}", nicer_strings.len());

    // println!("part 2 = {} houses", deliver(2, input.clone()));
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn nice_strings() {
        assert_eq!(nice("ugknbfddgicrmopn"), true);
        assert_eq!(nice("aaa"), true);
    }

    #[test]
    fn naughty_strings() {
        assert_eq!(nice("jchzalrnumimnmhp"), false);
        assert_eq!(nice("haegwjzuvuyypxyu"), false);
        assert_eq!(nice("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn new_nice_strings() {
        assert_eq!(new_nice("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(new_nice("xxyxx"), true);
    }

    #[test]
    fn new_naughty_strings() {
        assert_eq!(new_nice("uurcxstgmygtbstg"), false);
        assert_eq!(new_nice("ieodomkazucvgmuy"), false);
    }
}
