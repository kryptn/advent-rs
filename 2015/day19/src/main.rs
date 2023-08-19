use advent::fetch;

use itertools::Itertools;

use rayon::prelude::*;

fn replacements_for(molecule: &String, replacements: &Vec<(&str, &str)>) -> Vec<String> {
    let mut out = vec![];

    for (source, dest) in replacements {
        if source.len() > molecule.len() {
            continue;
        }
        for idx in 0..(molecule.len() - source.len()) + 1 {
            if molecule[idx..idx + source.len()] == **source {
                let mut m = molecule.clone();
                m.replace_range(idx..idx + source.len(), dest);
                out.push(m);
            }
        }
    }
    out.into_iter().collect()
}

fn find_target(molecule: &String, target: &String, replacements: &Vec<(&str, &str)>) -> usize {
    let mut candidates = vec![molecule.clone()];
    let mut steps = 0;
    loop {
        steps += 1;
        candidates = candidates
            .into_iter()
            .map(|m| replacements_for(&m, replacements))
            .flatten()
            .unique()
            .sorted_by(|a, b| a.len().cmp(&b.len()))
            // arbitrary, worked for my input. may not work for yours
            .take(200)
            .collect();

        if candidates.contains(target) {
            break;
        }
    }

    steps
}

fn main() {
    let input = fetch::get_input(2015, 19);

    //     let input = r#"H => HO
    // H => OH
    // O => HH

    // HOH"#;

    // let input = r#"e => O
    // e => H
    // H => HO
    // H => OH
    // O => HH

    // HOHOHO"#;

    let mut replacements = vec![];
    let lines = input.lines().map(|l| l.trim()).collect_vec();
    let starting = lines[lines.len() - 1].to_string();
    for line in lines {
        if line.len() > 0 {
            let parts = line.split(" ").collect_vec();
            replacements.push((parts[0], parts[2]))
        } else {
            break;
        }
    }

    let result: Vec<_> = replacements_for(&starting.clone(), &replacements)
        .into_iter()
        .unique()
        .collect();
    println!("part 1 -> {}", result.len());

    let replacements: Vec<_> = replacements
        .into_iter()
        .map(|(a, b)| (b, a))
        .sorted_by(|a, b| b.0.len().cmp(&a.0.len()))
        .collect();

    // dbg!(&path);
    println!(
        "part 2 => {}",
        find_target(&starting, &"e".to_string(), &replacements)
    );
}

#[cfg(test)]
mod test {

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn p1_tests() {}

    #[test]
    fn p2_tests() {}
}
