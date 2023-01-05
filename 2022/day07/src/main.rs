use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use advent::input_store;
use itertools::Itertools;

#[derive(Clone, Debug)]
enum Object {
    File { name: String, size: usize },
    Directory { name: String },
}

#[derive(Clone, Debug)]
struct Filesystem {
    files: HashMap<String, Object>,
    cursor: String,
}

fn parent(path: &str) -> String {
    let path = Path::new(&path);
    path.parent().unwrap().to_str().unwrap().to_string()
}

fn down(path: &str, dir: &str) -> String {
    let mut pb = PathBuf::from(&path);
    pb.push(dir);
    pb.as_path().to_str().unwrap().to_string()
}

impl Filesystem {
    fn new() -> Self {
        let mut files = HashMap::new();
        let cursor = "".to_string();

        files.insert(
            "/".to_string(),
            Object::Directory {
                name: "/".to_string(),
            },
        );

        Self { files, cursor }
    }

    fn traverse(&mut self, dir: &str) {
        if dir == "/" {
            self.cursor = "/".to_string();
        } else if dir == ".." {
            self.cursor = parent(&self.cursor);
        } else {
            self.cursor = down(&self.cursor, dir);
        }
    }

    fn with_name(&self, name: &String) -> String {
        let mut pb = PathBuf::from(self.cursor.clone());
        pb.push(name);
        pb.as_path().to_str().unwrap().to_string()
    }

    fn reapply(&mut self, log: &LogLine) {
        match log {
            LogLine::Cd { directory } => {
                self.traverse(directory);
            }
            LogLine::Ls => {}
            LogLine::Object(obj) => {
                let name = match obj {
                    Object::File { name, size: _ } => name,
                    Object::Directory { name } => name,
                };

                let filename = self.with_name(name);
                self.files.insert(filename, obj.clone());
            }
        }
    }

    fn du(&self, prefix: String) -> usize {
        self.files
            .iter()
            .filter(|(pf, _)| {
                let candidate = if prefix == "/" {
                    prefix.clone()
                } else {
                    format!("{}/", prefix)
                };

                pf.starts_with(&candidate)
            })
            .map(|(_, obj)| {
                // println!("checked {:?} with {:?}", pf, prefix);
                match obj {
                    Object::File { name: _, size } => size.clone(),
                    _ => 0,
                }
            })
            .sum::<usize>()
    }

    fn show(&self) {
        for item in self.files.iter().sorted_by(|lhs, rhs| lhs.0.cmp(rhs.0)) {
            let slashes = item.0.chars().filter(|c| *c == '/').count();
            let indent = {
                let mut out = "".to_string();
                if item.0.len() > 1 {
                    for _ in 0..slashes {
                        out += "|  ";
                    }
                }
                out
            };

            match item.1 {
                Object::File { name, size } => println!("f   {}{} {}", indent, name, size),
                Object::Directory { name } => {
                    let du = self.du(item.0.clone());
                    let arrow = if du <= 100000 {
                        "     <------------"
                    } else {
                        ""
                    };

                    println!("d   {}{} ({}) {}", indent, name, du, arrow)
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
enum LogLine {
    Cd { directory: String },
    Ls,
    Object(Object),
}

impl From<&str> for LogLine {
    fn from(line: &str) -> Self {
        let split: Vec<&str> = line.trim().split_whitespace().collect();

        if split[0] == "$" {
            if split[1] == "cd" {
                Self::Cd {
                    directory: split[2].to_string(),
                }
            } else {
                Self::Ls
            }
        } else if split[0] == "dir" {
            LogLine::Object(Object::Directory {
                name: split[1].to_string(),
            })
        } else {
            let size = split[0].parse().unwrap();
            let name = split[1].to_string();
            Self::Object(Object::File { name, size })
        }
    }
}

fn main() {
    let input = input_store::get_input(2022, 07);
    // let input = r#"$ cd /
    // $ ls
    // dir a
    // 14848514 b.txt
    // 8504156 c.dat
    // dir d
    // $ cd a
    // $ ls
    // dir e
    // 29116 f
    // 2557 g
    // 62596 h.lst
    // $ cd e
    // $ ls
    // 584 i
    // $ cd ..
    // $ cd ..
    // $ cd d
    // $ ls
    // 4060174 j
    // 8033020 d.log
    // 5626152 d.ext
    // 7214296 k"#;

    let logs: Vec<LogLine> = input.trim().lines().map(|line| line.into()).collect();

    let mut fs = Filesystem::new();

    for log in logs {
        fs.reapply(&log);
    }

    // dbg!(&fs);

    fs.show();

    let dir_sizes: Vec<(String, usize)> = fs
        .files
        .iter()
        .filter_map(|(pf, obj)| match obj {
            Object::Directory { name: _ } => Some((pf.clone(), fs.du(pf.clone()))),
            _ => None,
        })
        .sorted_by(|lhs, rhs| lhs.1.cmp(&rhs.1))
        .collect();

    let part_1: usize = dir_sizes
        .iter()
        .filter(|(_, size)| *size <= 100000)
        .map(|a| a.1)
        .sum();

    println!("part_1 => {}", part_1);

    let total = 70000000;
    let needed = 30000000;
    let usage = dir_sizes[dir_sizes.len() - 1].1;
    let available = total - usage;

    let cleanup: Vec<_> = dir_sizes
        .iter()
        .filter(|(_, size)| available + size > needed)
        .collect();
    println!("part_2 => {}", cleanup.first().unwrap().1);
}

#[cfg(test)]
mod test {

    use rstest::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[rstest]
    #[case("ADVENT", "ADVENT")]
    fn p1_tests(#[case] given: &str, #[case] expected: &str) {
        assert_eq!(given, expected);
    }

    #[rstest]
    #[case("ADVENT", "ADVENT")]
    fn p2_tests(#[case] given: &str, #[case] expected: &str) {
        assert_eq!(given, expected);
    }
}
