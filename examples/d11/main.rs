use num_bigint::BigInt;

fn main() {
    let puzzle_file = std::env::args()
        .nth(1)
        .expect("Error: Called without input");
    println!("Part 1: {}\nPart 2: {}", p1(&puzzle_file), p2(&puzzle_file));
}
fn p1(filename: &str) -> String {
    let txt = std::fs::read_to_string(filename).unwrap();
    let mut monkeys = parse::parse_monkeys(&txt).unwrap();
    let mut round = 1;
    while round <= 20 {
        let last_monkey = monkeys.len();
        for j in 00..last_monkey {
            let mut monkey = monkeys[j].clone();
            monkey.count += monkey.items.len();
            while !monkey.items.is_empty() {
                let item = monkey.items[0].clone();
                monkey.items.remove(0);
                if monkey.test_item(&item) {
                    let if_true = monkey.if_true;
                    monkeys[if_true].items.push(monkey.perform_op(&item));
                } else {
                    monkeys[monkey.if_false]
                        .items
                        .push(monkey.perform_op(&item));
                }
            }
            monkeys[j] = monkey;
        }
        round += 1;
    }
    let mut top_two: Vec<usize> = monkeys.iter().map(|m| m.count).collect();
    top_two.sort();
    top_two.reverse();
    format!("{}", top_two[0] * top_two[1])
}

fn p2(filename: &str) -> String {
    let txt = std::fs::read_to_string(filename).unwrap();
    let mut monkeys = parse::parse_monkeys(&txt).unwrap();
    let mut round = 1;
    let one = BigInt::from(1);
    let modulus: BigInt = monkeys
        .iter()
        .map(|m| {
            &m.test
                * (match &m.op {
                    MonkeyOp::Times(n) => &n,
                    _ => &one,
                })
        })
        .product();
    while round <= 10_000 {
        let last_monkey = monkeys.len();
        for j in 00..last_monkey {
            let mut monkey = monkeys[j].clone();
            monkey.count += monkey.items.len();
            while !monkey.items.is_empty() {
                let item = monkey.items[0].clone();
                monkey.items.remove(0);
                if monkey.test_item_p2(&item) {
                    let if_true = monkey.if_true;
                    monkeys[if_true]
                        .items
                        .push(monkey.perform_op_p2(&item) % modulus.clone());
                } else {
                    monkeys[monkey.if_false]
                        .items
                        .push(monkey.perform_op_p2(&item) % modulus.clone());
                }
            }
            monkeys[j] = monkey;
        }
        round += 1;
    }
    let mut top_two: Vec<usize> = monkeys.iter().map(|m| m.count).collect();
    top_two.sort();
    top_two.reverse();
    format!("{}", top_two[0] * top_two[1])
}
#[derive(Clone, Debug)]
pub struct Monkey {
    pub number: usize,
    pub items: Vec<BigInt>,
    pub op: MonkeyOp,
    pub test: BigInt,
    pub if_true: usize,
    pub if_false: usize,
    pub count: usize,
}

#[derive(Clone, Debug)]
pub enum MonkeyOp {
    Times(BigInt),
    Plus(BigInt),
    Square,
}

impl Monkey {
    pub fn test_item(&mut self, item: &BigInt) -> bool {
        self.perform_op(&item) % &self.test == BigInt::from(0)
    }
    pub fn perform_op(&self, item: &BigInt) -> BigInt {
        (match &self.op {
            MonkeyOp::Times(x) => item * x,
            MonkeyOp::Plus(x) => item + x,
            MonkeyOp::Square => item * item,
        } / 3)
    }
    pub fn test_item_p2(&mut self, item: &BigInt) -> bool {
        self.perform_op_p2(&item) % &self.test == BigInt::from(0)
    }
    pub fn perform_op_p2(&self, item: &BigInt) -> BigInt {
        match &self.op {
            MonkeyOp::Times(x) => item * x,
            MonkeyOp::Plus(x) => item + x,
            MonkeyOp::Square => item * item,
        }
    }
}
mod parse {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{i32, multispace0, multispace1, u64},
        combinator::{map, value},
        multi::separated_list1,
        sequence::preceded,
        IResult,
    };

    use crate::{Monkey, MonkeyOp};
    pub fn parse_monkeys(txt: &str) -> Result<Vec<Monkey>, String> {
        match monkeys_parser(txt) {
            Ok(("", monkeys)) => Ok(monkeys),
            Ok((rest, _)) => Err(format!("Error parsing.  Remaining test: {:?}", rest)),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
    fn monkeys_parser(txt: &str) -> IResult<&str, Vec<Monkey>> {
        separated_list1(multispace1, monkey_parser)(txt)
    }
    fn monkey_parser(txt: &str) -> IResult<&str, Monkey> {
        let (rest, _) = tag("Monkey ")(txt)?;
        let (rest, number) = u64(rest)?;
        let (rest, _) = tag(":")(rest)?;
        let (rest, _) = multispace0(rest)?;
        let (rest, items) =
            preceded(tag("Starting items: "), separated_list1(tag(", "), i32))(rest)?;
        let (rest, _) = multispace0(rest)?;
        let (rest, op) = preceded(
            tag("Operation: new = old "),
            alt((
                preceded(
                    tag("+ "),
                    map(u64, |n| MonkeyOp::Plus(crate::BigInt::from(n))),
                ),
                value(MonkeyOp::Square, tag("* old")),
                preceded(
                    tag("* "),
                    map(u64, |n| MonkeyOp::Times(crate::BigInt::from(n))),
                ),
            )),
        )(rest)?;
        let (rest, _) = multispace0(rest)?;
        let (rest, _) = tag("Test: divisible by ")(rest)?;
        let (rest, test) = u64(rest)?;
        let (rest, _) = multispace0(rest)?;
        let (rest, if_true) = preceded(tag("If true: throw to monkey "), u64)(rest)?;
        let (rest, _) = multispace0(rest)?;
        let (rest, if_false) = preceded(tag("If false: throw to monkey "), u64)(rest)?;
        IResult::Ok((
            rest,
            Monkey {
                number: number as usize,
                items: items
                    .iter()
                    .map(|item| crate::BigInt::from(*item))
                    .collect(),
                op,
                test: crate::BigInt::from(test),
                if_true: if_true as usize,
                if_false: if_false as usize,
                count: 0,
            },
        ))
    }
}
