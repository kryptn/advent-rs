use advent::input_store;
use itertools::Itertools;
use nom::{bytes::complete::take, multi::many1, IResult};

#[derive(Debug)]
enum PacketType {
    Literal(usize),
    Operator(usize, Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: usize,
    packet: PacketType,
}

impl Packet {
    fn sum_versions(&self) -> usize {
        match &self.packet {
            PacketType::Literal(_) => self.version,
            PacketType::Operator(_, packets) => {
                packets.iter().map(|p| p.sum_versions()).sum::<usize>() + self.version
            }
        }
    }

    fn resolve(&self) -> usize {
        match &self.packet {
            PacketType::Literal(value) => value.clone(),
            PacketType::Operator(operator_type, packets) => {
                let packets = packets.iter().map(|p| p.resolve());
                match operator_type {
                    0 => packets.sum::<usize>(),
                    1 => packets.fold(1, |a, b| a * b),
                    2 => packets.min().unwrap(),
                    3 => packets.max().unwrap(),
                    5 => {
                        let mut packets = packets;
                        let first = packets.next().unwrap();
                        let second = packets.next().unwrap();
                        if first > second {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        let mut packets = packets;
                        let first = packets.next().unwrap();
                        let second = packets.next().unwrap();
                        if first < second {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        let mut packets = packets;
                        let first = packets.next().unwrap();
                        let second = packets.next().unwrap();
                        if first == second {
                            1
                        } else {
                            0
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

fn parse_segment(input: &str) -> IResult<&str, (bool, usize)> {
    let (input, cont) = take(1usize)(input)?;
    let cont = cont == "1";
    let (input, value) = take(4usize)(input)?;
    let value = usize::from_str_radix(value, 2).expect("known values");

    Ok((input, (cont, value)))
}

fn parse_literal(version: usize, input: &str) -> IResult<&str, Packet> {
    let mut collector = 0;

    let mut input = input;

    loop {
        let (new_input, (cont, value)) = parse_segment(input)?;

        input = new_input;
        collector = collector << 4;
        collector += value;

        if !cont {
            break;
        }
    }

    let packet_type = PacketType::Literal(collector);

    let out = Packet {
        version,
        packet: packet_type,
    };

    Ok((input, out))
}

#[derive(Debug)]
enum PacketLengthType {
    Bits(usize),
    Packets(usize),
}

fn parse_sub_packet_size(input: &str) -> IResult<&str, PacketLengthType> {
    let (input, value) = take(1usize)(input)?;

    if value == "1" {
        let (input, size) = take(11usize)(input)?;
        let size = usize::from_str_radix(size, 2).expect("known values");
        Ok((input, PacketLengthType::Packets(size)))
    } else {
        let (input, size) = take(15usize)(input)?;
        let size = usize::from_str_radix(size, 2).expect("known values");
        Ok((input, PacketLengthType::Bits(size)))
    }
}

fn parse_operator(version: usize, type_id: usize, input: &str) -> IResult<&str, Packet> {
    let (input, sub_pkg_size) = parse_sub_packet_size(input)?;

    let (input, packets) = match sub_pkg_size {
        PacketLengthType::Bits(n) => {
            let (input, sub) = take(n)(input)?;
            let (_, packets) = parse_packets(sub)?;
            (input, packets)
        }
        PacketLengthType::Packets(n) => {
            let mut input = input;
            let mut packets = Vec::new();
            for _ in 0..n {
                let (i, packet) = parse_packet(input)?;
                input = i;
                packets.push(packet);
            }
            (input, packets)
        }
    };

    let packet = PacketType::Operator(type_id, packets);
    let packet = Packet { version, packet };

    Ok((input, packet))
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    let (input, packet_version) = take(3usize)(input)?;
    let packet_version = usize::from_str_radix(packet_version, 2).expect("known values");

    let (input, packet_type_id) = take(3usize)(input)?;
    let packet_type_id = usize::from_str_radix(packet_type_id, 2).expect("known values");

    let (input, packet) = match packet_type_id {
        4 => parse_literal(packet_version, input),
        _ => parse_operator(packet_version, packet_type_id, input),
    }?;

    Ok((input, packet))
}

fn parse_packets(input: &str) -> IResult<&str, Vec<Packet>> {
    many1(parse_packet)(input)
}

fn hex_to_bin(c: char) -> String {
    let s = match c {
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
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => unreachable!(),
    };

    s.to_string()
}

fn main() {
    let input = input_store::get_input(2021, 16);

    let input = input.trim().chars().map(|c| hex_to_bin(c)).join("");

    let (_, packets) = parse_packets(&input).unwrap();

    let part_1: usize = packets.iter().map(|p| p.sum_versions()).sum();

    let part_2 = packets.iter().map(|p| p.resolve()).next().unwrap();

    println!("part_1 => {}", part_1);
    println!("part_2 => {}", part_2);
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[test]
    fn do_test() {
        assert_eq!(2, 2);
    }

    #[rstest]
    #[case("8A004A801A8002F478", 16)]
    #[case("620080001611562C8802118E34", 12)]
    #[case("C0015000016115A2E0802F182340", 23)]
    #[case("A0016C880162017C3686B18A3D4780", 31)]
    fn p1_tests(#[case] given: &str, #[case] expected: usize) {
        let given = given.chars().map(|c| hex_to_bin(c)).join("");
        let packet = parse_packet(&given).unwrap().1;
        assert_eq!(packet.sum_versions(), expected);
    }

    #[rstest]
    #[case("C200B40A82", 3)]
    #[case("04005AC33890", 54)]
    #[case("880086C3E88112", 7)]
    #[case("CE00C43D881120", 9)]
    #[case("D8005AC2A8F0", 1)]
    #[case("F600BC2D8F", 0)]
    #[case("9C005AC2F8F0", 0)]
    #[case("9C0141080250320F1802104A08", 1)]

    fn p2_tests(#[case] given: &str, #[case] expected: usize) {
        let given = given.chars().map(|c| hex_to_bin(c)).join("");
        let packet = parse_packet(&given).unwrap().1;
        assert_eq!(packet.resolve(), expected);
    }
}
