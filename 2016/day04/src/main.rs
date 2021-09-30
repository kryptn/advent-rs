use advent::input_store;
use itertools::Itertools;
use nom::character::complete as ch;
use nom::{multi, sequence, IResult};

fn name_segment(input: &str) -> IResult<&str, &str> {
    let (input, letters) = ch::alpha1(input)?;
    let (input, _) = ch::char('-')(input)?;

    Ok((input, letters))
}

fn build_checksum(segments: Vec<&str>) -> String {
    segments
        .join("")
        .chars()
        .sorted()
        .dedup_with_count()
        .sorted_by(|left, right| {
            if left.0 == right.0 {
                left.1.cmp(&right.1)
            } else {
                right.0.cmp(&left.0)
            }
        })
        .map(|(_, letter)| letter)
        .join("")
}

fn parse_room(input: &str) -> IResult<&str, Room> {
    let (input, (letter_sets, sector)) = multi::many_till(name_segment, ch::digit1)(input)?;
    let (input, checksum) = sequence::delimited(ch::char('['), ch::alpha1, ch::char(']'))(input)?;

    let sector_id = sector.parse().unwrap();
    let distinct = build_checksum(letter_sets.clone());

    Ok((
        input,
        Room {
            letter_sets,
            sector_id,
            checksum,
            distinct,
        },
    ))
}

#[derive(PartialEq, Debug)]
struct Room<'a> {
    letter_sets: Vec<&'a str>,
    sector_id: i32,
    checksum: &'a str,
    distinct: String,
}

impl<'a> From<&'a str> for Room<'a> {
    fn from(input: &'a str) -> Self {
        let (_, room) = parse_room(input).unwrap();
        room
    }
}

impl<'a> Room<'a> {
    fn is_real(&self) -> bool {
        self.distinct.starts_with(self.checksum)
    }

    fn decrypt(&self) -> String {
        let mut out = String::new();

        for segment in self.letter_sets.clone() {
            for chr in segment.bytes() {
                let offset = (chr - 97) as i32;
                let rotated = (offset + self.sector_id) % 26;
                let reset = (rotated + 97) as u8;

                out.push(reset as char)
            }
            out.push(' ');
        }

        out
    }
}

fn main() {
    let input = input_store::get_input(2016, 4);
    let rooms: Vec<Room> = input.lines().map(|line| line.into()).collect();

    let part_1: i32 = rooms
        .iter()
        .filter(|r| r.is_real())
        .map(|r| r.sector_id)
        .sum();
    println!("part 1 => {}", part_1);

    for room in rooms {
        // println!("{} -> {}", room.decrypt(), room.sector_id);
        if room.decrypt() == "northpole object storage " {
            println!("part 2 => {}", room.sector_id);
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[test]
    fn parser_test() {
        let input = "aaaaa-bbb-z-y-x-123[abxyz]";

        let (_, room) = parse_room(input).unwrap();

        let expected = Room {
            letter_sets: vec!["aaaaa", "bbb", "z", "y", "x"],
            sector_id: 123,
            checksum: "abxyz",
            distinct: build_checksum(vec!["aaaaa", "bbb", "z", "y", "x"]),
        };

        assert_eq!(expected, room);
    }

    #[test]
    fn validity() {
        let room: Room = "aaaaa-bbb-z-y-x-123[abxyz]".into();
        assert_eq!(room.is_real(), true);

        let room: Room = "a-b-c-d-e-f-g-h-987[abcde]".into();
        assert_eq!(room.is_real(), true);

        let room: Room = "not-a-real-room-404[oarel]".into();
        assert_eq!(room.is_real(), true);

        let room: Room = "totally-real-room-200[decoy]".into();
        assert_eq!(room.is_real(), false);
    }

    #[test]
    fn build_checksum_test() {
        let ls = vec!["aaa", "bbb", "ccc", "abcd"];
        assert_eq!(build_checksum(ls), "abcd");

        let ls = vec!["aaa", "bb", "cc", "d", "ee"];
        assert_eq!(build_checksum(ls), "abced");
    }

    #[test]

    fn decryptor() {
        let room: Room = "qzmt-zixmtkozy-ivhz-343[abcde]".into();

        assert_eq!(room.decrypt(), "very encrypted name ");
    }

    #[test]
    fn p1_tests() {}

    #[test]
    fn p2_tests() {}
}
