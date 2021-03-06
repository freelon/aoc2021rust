use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::char,
    combinator::map_res,
    error::ErrorKind,
    multi::{many0, many_m_n},
    sequence::preceded,
    IResult,
};

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let bitstring: String = to_bits(input);
    let p = transmission(&bitstring).unwrap().1.packet;
    p.version_sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let bitstring: String = to_bits(input);
    let p = transmission(&bitstring).unwrap().1.packet;
    p.value()
}

fn to_bits(input: &str) -> String {
    input
        .trim()
        .split("")
        .flat_map(|c| u8::from_str_radix(c, 16))
        .map(|u| format!("{:04b}", u))
        .collect()
}

type LiteralType = i64;

#[derive(Debug, PartialEq)]
enum PacketType {
    Literal(LiteralType),
    Operator(u8, Vec<Packet>),
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: u8,
    content: PacketType,
}

impl Packet {
    fn version_sum(&self) -> usize {
        let sub: usize = match &self.content {
            PacketType::Literal(_) => 0,
            PacketType::Operator(_, subs) => subs.iter().map(|p| p.version_sum()).sum(),
        };
        self.version as usize + sub
    }

    fn value(&self) -> usize {
        match &self.content {
            PacketType::Literal(value) => *value as usize,
            PacketType::Operator(0, subs) => subs.iter().map(|p| p.value()).sum(),
            PacketType::Operator(1, subs) => subs.iter().fold(1, |acc, p| acc * p.value()),
            PacketType::Operator(2, subs) => subs.iter().map(|p| p.value()).min().unwrap(),
            PacketType::Operator(3, subs) => subs.iter().map(|p| p.value()).max().unwrap(),
            PacketType::Operator(5, subs) => {
                if subs[0].value() > subs[1].value() {
                    1
                } else {
                    0
                }
            }
            PacketType::Operator(6, subs) => {
                if subs[0].value() < subs[1].value() {
                    1
                } else {
                    0
                }
            }
            PacketType::Operator(7, subs) => {
                if subs[0].value() == subs[1].value() {
                    1
                } else {
                    0
                }
            }
            PacketType::Operator(type_id, _) => panic!("Unknown type id '{}'", type_id),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Transmission {
    packet: Packet,
}

fn transmission(input: &str) -> IResult<&str, Transmission> {
    let (input, packet) = packet(&input)?;
    let (input, _) = many_m_n(0, 7, char('0'))(input)?;

    Ok((input, Transmission { packet }))
}

fn packet(input: &str) -> IResult<&str, Packet> {
    let (input, version) =
        map_res(take(3usize), |version: &str| u8::from_str_radix(version, 2))(input)?;
    let (input, content) = alt((literal, operator))(input)?;

    Ok((
        input,
        Packet {
            version: version as u8,
            content,
        },
    ))
}

fn operator(input: &str) -> IResult<&str, PacketType> {
    let (input, op_type) =
        map_res(take(3usize), |version: &str| u8::from_str_radix(version, 2))(input)?;
    let (input, length_type_id) = take(1usize)(input)?;
    let (input, sub_packets) = match length_type_id {
        "0" => {
            let (input, bit_length) = map_res(take(15usize), |version: &str| {
                usize::from_str_radix(version, 2)
            })(input)?;
            let (input, sub_packets_input) = take(bit_length)(input)?;
            let (sub_packets_input, sub_packets) = many0(packet)(sub_packets_input)?;
            if !sub_packets_input.is_empty() {
                println!("warning: not all bites consumed!");
            }

            (input, sub_packets)
        }
        "1" => {
            let (input, number_of_sub_packets) = map_res(take(11usize), |version: &str| {
                usize::from_str_radix(version, 2)
            })(input)?;
            let (input, sub_packets) = many_m_n(0, number_of_sub_packets, packet)(input)?;

            (input, sub_packets)
        }
        _ => {
            return Err(nom::Err::Failure(nom::error::Error::new(
                input,
                ErrorKind::Alpha,
            )))
        }
    };
    Ok((input, PacketType::Operator(op_type, sub_packets)))
}

fn literal(input: &str) -> IResult<&str, PacketType> {
    let (input, _) = tag("100")(input)?;
    let (input, number) = literal_content(input)?;

    Ok((input, PacketType::Literal(number)))
}

fn literal_content(input: &str) -> IResult<&str, LiteralType> {
    let (input, v1) = many0(preceded(tag("1"), take(4usize)))(input)?;
    let (input, v2) = preceded(tag("0"), take(4usize))(input)?;
    let v = format!("{}{}", v1.join(""), v2);
    let number = LiteralType::from_str_radix(&v, 2)
        .map_err(|_| nom::Err::Failure(nom::error::Error::new(input, ErrorKind::Alt)))?;
    Ok((input, number))
}

#[cfg(test)]
pub mod test {

    use super::*;

    #[test]
    fn literal() {
        assert_eq!(
            transmission(&to_bits("D2FE28")),
            Ok((
                "",
                Transmission {
                    packet: Packet {
                        version: 6,
                        content: PacketType::Literal(2021),
                    },
                },
            )),
        )
    }

    #[test]
    pub fn test_16_1() {
        assert_eq!(part1("8A004A801A8002F478"), 16);
    }

    #[test]
    pub fn test_16_2() {
        assert_eq!(part2("C200B40A82"), 3);
        assert_eq!(part2("04005AC33890"), 54);
        assert_eq!(part2("880086C3E88112"), 7);
        assert_eq!(part2("CE00C43D881120"), 9);
        assert_eq!(part2("D8005AC2A8F0"), 1);
        assert_eq!(part2("F600BC2D8F"), 0);
        assert_eq!(part2("9C005AC2F8F0"), 0);
        assert_eq!(part2("9C0141080250320F1802104A08"), 1);
    }
}
