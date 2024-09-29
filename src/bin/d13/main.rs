use std::cmp::Ordering;
fn main() {
    let puzzle_file = std::env::args()
        .nth(1)
        .expect("Error: Called without input");
    println!("Part 1: {}", p1(&puzzle_file));
    println!("Part 2: {}", p2(&puzzle_file));
}

fn p1(filename: &str) -> String {
    let txt = std::fs::read_to_string(filename).unwrap();
    let pairs = parse::parse_pairs(&txt).unwrap();
    pairs
        .into_iter()
        .enumerate()
        .map(|(number, Pair { left, right })| if left < right { number + 1 } else { 0 })
        .sum::<usize>()
        .to_string()
}

fn p2(filename: &str) -> String {
    let txt = std::fs::read_to_string(filename).unwrap();
    let mut all_packets = vec![];
    for pair in parse::parse_pairs(&txt).unwrap() {
        all_packets.push(pair.left);
        all_packets.push(pair.right);
    }
    let separator1 = Packet::List(vec![Box::new(Packet::List(vec![Box::new(
        Packet::Integer(2),
    )]))]);
    let separator2 = Packet::List(vec![Box::new(Packet::List(vec![Box::new(
        Packet::Integer(6),
    )]))]);
    all_packets.push(separator1.clone());
    all_packets.push(separator2.clone());

    all_packets.sort();
    let d1 = all_packets.binary_search(&separator1).unwrap() + 1;
    let d2 = all_packets.binary_search(&separator2).unwrap() + 1;

    (d1 * d2).to_string()
}

#[derive(Debug)]
pub struct Pair {
    left: Packet,
    right: Packet,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Packet {
    Integer(i32),
    List(Vec<Box<Packet>>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Integer(left), Packet::Integer(right)) => left.partial_cmp(right),
            (Packet::Integer(left), Packet::List(right)) => {
                if right.is_empty() {
                    Some(Ordering::Greater)
                } else {
                    match Packet::Integer(*left).partial_cmp(&right[0]) {
                        Some(Ordering::Equal) => {
                            if right.len() > 1 {
                                Some(Ordering::Less)
                            } else {
                                Some(Ordering::Equal)
                            }
                        }
                        nonequal => nonequal,
                    }
                }
            }
            (Packet::List(left), Packet::Integer(right)) => {
                if left.is_empty() {
                    Some(Ordering::Less)
                } else {
                    match left[0].partial_cmp(&Box::new(Packet::Integer(*right))) {
                        Some(Ordering::Equal) => {
                            if left.len() > 1 {
                                Some(Ordering::Greater)
                            } else {
                                Some(Ordering::Equal)
                            }
                        }
                        nonequal => nonequal,
                    }
                }
            }
            (Packet::List(left), Packet::List(right)) => {
                if left.is_empty() {
                    if right.is_empty() {
                        return Some(Ordering::Equal);
                    } else {
                        return Some(Ordering::Less);
                    }
                };
                if right.is_empty() {
                    return Some(Ordering::Greater);
                };
                let mut i = 0;
                while i < left.len() && i < right.len() {
                    match left[i].partial_cmp(&right[i]) {
                        Some(Ordering::Equal) => i += 1,
                        other => return other,
                    };
                }
                if i == left.len() && i == right.len() {
                    return Some(Ordering::Equal);
                } else if i == left.len() {
                    return Some(Ordering::Less);
                } else {
                    return Some(Ordering::Greater);
                }
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

mod parse {
    use crate::{Packet, Pair};
    use nom::bytes::complete::tag;
    use nom::character::complete::multispace0;
    use nom::multi::{separated_list0, separated_list1};
    use nom::{
        branch::alt,
        character::complete::{i32, multispace1},
        combinator::map,
        sequence::{delimited, preceded, terminated},
        IResult,
    };
    pub fn parse_pairs(txt: &str) -> Result<Vec<Pair>, String> {
        match pairs_parser(txt) {
            Ok(("", pairs)) => Ok(pairs),
            Ok((rest, _)) => Err(format!("Leftover junk at end: {}", rest)),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    fn pairs_parser(txt: &str) -> IResult<&str, Vec<Pair>> {
        terminated(separated_list1(multispace1, pair_parser), multispace0)(txt)
    }
    fn pair_parser(txt: &str) -> IResult<&str, Pair> {
        let (rest, left) = packet_parser(txt)?;
        let (rest, right) = preceded(multispace1, packet_parser)(rest)?;
        Ok((rest, Pair { left, right }))
    }
    fn packet_int_parser(txt: &str) -> IResult<&str, Packet> {
        map(i32, Packet::Integer)(txt)
    }
    fn packet_list_parser(txt: &str) -> IResult<&str, Packet> {
        map(
            delimited(tag("["), separated_list0(tag(","), packet_parser), tag("]")),
            |packets| Packet::List(packets.into_iter().map(|packet| Box::new(packet)).collect()),
        )(txt)
    }
    fn packet_parser(txt: &str) -> IResult<&str, Packet> {
        alt((packet_int_parser, packet_list_parser))(txt)
    }
}
