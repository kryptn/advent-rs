pub fn factors(input: i64) -> Vec<i64> {
    let mut out = vec![1];

    for factor in 2..=input {
        if input % factor == 0 {
            out.push(factor)
        }
    }

    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(factors(10), vec![1, 2, 5, 10]);
        assert_eq!(factors(16), vec![1, 2, 4, 8, 16]);
    }
}
