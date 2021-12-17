use std::borrow::BorrowMut;
use std::fs;

use bitvec::prelude::*;
use std::path::Path;

type Input = String;
pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day16.txt");
    println!("Solution: {}", part_one(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_from_file("./inputs/day16.txt");
    println!("Solution: {}", part_two(input));
    Ok(())
}

fn parse_from_file<T: AsRef<Path>>(filename: T) -> Input {
    fs::read_to_string(filename).unwrap()
}

pub fn part_one(input: Input) -> usize {
    let packet = parse_to_bitvec(&input);
    let packet = parse_packet(&mut packet.iter()).unwrap();
    sum_versions(&packet)
}

fn sum_versions(packet: &Packet) -> usize {
    if packet.packet_type == 4 {
        return packet.version;
    }
    let contained_packets = packet.contained_packets.as_ref().unwrap();
    return packet.version + contained_packets.iter().map(sum_versions).sum::<usize>();
}

pub fn part_two(input: Input) -> usize {
    let packet = parse_to_bitvec(&input);
    let packet = parse_packet(&mut packet.iter()).unwrap();
    get_value(&packet)
}

fn get_value(packet: &Packet) -> usize {
    match packet.packet_type {
        0 => packet.contained_packets.as_ref().unwrap().iter().map(get_value).sum(),
        1 => packet
            .contained_packets
            .as_ref()
            .unwrap()
            .iter()
            .map(get_value)
            .product(),
        2 => packet
            .contained_packets
            .as_ref()
            .unwrap()
            .iter()
            .map(get_value)
            .min()
            .unwrap(),
        3 => packet
            .contained_packets
            .as_ref()
            .unwrap()
            .iter()
            .map(get_value)
            .max()
            .unwrap(),
        4 => packet.literal.unwrap(),
        5 => {
            let contained_packets = packet.contained_packets.as_ref().unwrap();
            let packet_values: Vec<_> = contained_packets.iter().map(get_value).collect();
            if packet_values[0] > packet_values[1] {
                1
            } else {
                0
            }
        }
        6 => {
            let contained_packets = packet.contained_packets.as_ref().unwrap();
            let packet_values: Vec<_> = contained_packets.iter().map(get_value).collect();
            if packet_values[0] < packet_values[1] {
                1
            } else {
                0
            }
        }
        7 => {
            let contained_packets = packet.contained_packets.as_ref().unwrap();
            let packet_values: Vec<_> = contained_packets.iter().map(get_value).collect();
            if packet_values[0] == packet_values[1] {
                1
            } else {
                0
            }
        }
        _ => panic!("Unknown operation"),
    }
}

fn parse_to_bitvec(input: &str) -> BitVec {
    input
        .chars()
        .map(|hex| hex.to_digit(16).unwrap())
        .flat_map(|digit| {
            (0..4)
                .rev()
                .map(move |shift| (digit & (0x1 << shift)) == (0x1 << shift))
        })
        .collect()
}

fn get_value_from_bitslice(bitslice: &BitSlice) -> usize {
    let mut acc = 0;
    for bit in bitslice {
        acc <<= 1;
        acc |= if *bit { 1 } else { 0 };
    }
    acc
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Packet {
    version: usize,
    packet_type: usize,
    literal: Option<usize>,
    contained_packets: Option<Vec<Packet>>,
}

fn parse_literal<'a, Const: bitvec::ptr::Mutability, T: Iterator<Item = BitRef<'a, Const>>>(mut bits_iter: T) -> usize {
    let mut literal = 0;
    let mut continuing;
    loop {
        literal <<= 4;
        let bit = bits_iter.borrow_mut().next();
        match bit {
            None => {
                return literal;
            }
            Some(b) => {
                continuing = *b;
            }
        }
        let literal_bits = bits_iter.borrow_mut().take(4).collect::<BitVec>();
        literal |= get_value_from_bitslice(&literal_bits[..]);
        if !continuing {
            break literal;
        }
    }
}

fn get_subpacket_bits_length<Const: bitvec::ptr::Mutability>(
    bits_iter: &mut dyn Iterator<Item = BitRef<Const>>,
) -> usize {
    get_value_from_bitslice(&(bits_iter.borrow_mut()).take(15).collect::<BitVec>())
}

fn get_subpacket_number<Const: bitvec::ptr::Mutability>(bits_iter: &mut dyn Iterator<Item = BitRef<Const>>) -> usize {
    get_value_from_bitslice(&(bits_iter.borrow_mut()).take(11).collect::<BitVec>())
}

fn parse_packet<Const: bitvec::ptr::Mutability>(
    mut bits_iter: &mut dyn Iterator<Item = BitRef<Const>>,
) -> Option<Packet> {
    let version = &bits_iter.borrow_mut().take(3).collect::<BitVec>();
    if version.len() != 3 {
        return None;
    }
    let version = get_value_from_bitslice(version);
    let packet_type = get_value_from_bitslice(&bits_iter.borrow_mut().take(3).collect::<BitVec>());
    match packet_type {
        4 => {
            let literal = parse_literal(&mut bits_iter);
            Some(Packet {
                version,
                packet_type,
                literal: Some(literal),
                contained_packets: None,
            })
        }
        _ => match *bits_iter.borrow_mut().next().unwrap() {
            false => {
                let bit_length_of_contained_packets = get_subpacket_bits_length(&mut bits_iter);
                let mut contained_packets_bits = bits_iter.borrow_mut().take(bit_length_of_contained_packets);
                let mut contained_packets = vec![];
                loop {
                    let result = parse_packet(&mut contained_packets_bits);
                    if let Some(packet) = result {
                        contained_packets.push(packet);
                    } else {
                        break;
                    }
                }
                Some(Packet {
                    packet_type,
                    version,
                    literal: None,
                    contained_packets: Some(contained_packets),
                })
            }
            true => {
                let number_of_subpackets = get_subpacket_number(&mut bits_iter);
                let mut contained_packets = vec![];
                while contained_packets.len() < number_of_subpackets {
                    let result = parse_packet(&mut bits_iter);
                    contained_packets.push(result.expect("This is some sort of error I'd reckon!"));
                }
                Some(Packet {
                    packet_type,
                    version,
                    literal: None,
                    contained_packets: Some(contained_packets),
                })
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_STRS_PART_ONE: [&str; 4] = [
        "8A004A801A8002F478",
        "620080001611562C8802118E34",
        "C0015000016115A2E0802F182340",
        "A0016C880162017C3686B18A3D4780",
    ];

    const EXAMPLE_STRS_PART_TWO: [&str; 8] = [
        "C200B40A82",
        "04005AC33890",
        "880086C3E88112",
        "CE00C43D881120",
        "D8005AC2A8F0",
        "F600BC2D8F",
        "9C005AC2F8F0",
        "9C0141080250320F1802104A08",
    ];

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_STRS_PART_ONE[0].to_owned()), 16);
        assert_eq!(part_one(EXAMPLE_STRS_PART_ONE[1].to_owned()), 12);
        assert_eq!(part_one(EXAMPLE_STRS_PART_ONE[2].to_owned()), 23);
        assert_eq!(part_one(EXAMPLE_STRS_PART_ONE[3].to_owned()), 31);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_STRS_PART_TWO[0].to_owned()), 3);
        assert_eq!(part_two(EXAMPLE_STRS_PART_TWO[1].to_owned()), 54);
        assert_eq!(part_two(EXAMPLE_STRS_PART_TWO[2].to_owned()), 7);
        assert_eq!(part_two(EXAMPLE_STRS_PART_TWO[3].to_owned()), 9);
        assert_eq!(part_two(EXAMPLE_STRS_PART_TWO[4].to_owned()), 1);
        assert_eq!(part_two(EXAMPLE_STRS_PART_TWO[5].to_owned()), 0);
        assert_eq!(part_two(EXAMPLE_STRS_PART_TWO[6].to_owned()), 0);
        assert_eq!(part_two(EXAMPLE_STRS_PART_TWO[7].to_owned()), 1);
    }

    #[test]
    fn test_simple_literal_packet() {
        let packet = parse_to_bitvec("D2FE28");
        let mut bits_iter = packet.iter();
        assert_eq!(
            parse_packet(&mut bits_iter),
            Some(Packet {
                version: 6,
                packet_type: 4,
                literal: Some(2021),
                contained_packets: None
            })
        );
    }

    #[test]
    fn get_parse_literal() {
        let bits = bitvec![1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0,];
        assert_eq!(parse_literal(&mut bits.iter()), 2021)
    }

    #[test]
    fn test_parse_operator_with_subpacket_count() {
        let packet = parse_to_bitvec("EE00D40C823060");
        let packet = parse_packet(&mut packet.iter());
        assert!(packet.is_some());
        let packet = packet.unwrap();
        assert_ne!(packet.packet_type, 4);
        assert_eq!(packet.version, 7);
        assert!(packet.contained_packets.is_some());
        let contained_packets = packet.contained_packets.unwrap();
        assert_eq!(contained_packets.len(), 3);
        assert_eq!(contained_packets[0].literal, Some(1));
        assert_eq!(contained_packets[1].literal, Some(2));
        assert_eq!(contained_packets[2].literal, Some(3));
    }

    #[test]
    fn test_parse_operator_with_subpacket_bits_count() {
        let packet = parse_to_bitvec("38006F45291200");
        let packet = parse_packet(&mut packet.iter());
        assert!(packet.is_some());
        let packet = packet.unwrap();
        assert_eq!(packet.version, 1);
        assert_ne!(packet.packet_type, 4);
        assert!(packet.contained_packets.is_some());
        let contained_packets = packet.contained_packets.unwrap();
        assert_eq!(contained_packets.len(), 2);
        assert_eq!(contained_packets[0].literal, Some(10));
        assert_eq!(contained_packets[1].literal, Some(20));
    }

    #[test]
    #[ignore]
    fn test_to_binary() {
        assert_eq!(
            parse_to_bitvec("8A00"),
            bitvec![1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(parse_to_bitvec("8A0"), bitvec![1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0,]);
    }
}
