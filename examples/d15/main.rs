mod coords;

use coords::Coords;
use std::collections::VecDeque;
fn main() {
    let puzzle_file = std::env::args()
        .nth(1)
        .expect("Error: Called without input");
    println!("Part 1: {}", p1(&puzzle_file, 2000000));
    println!("Part 2: {}", p2(&puzzle_file, 4000000));
}

fn p1(filename: &str, row: i32) -> String {
    let txt = std::fs::read_to_string(filename).unwrap();
    let sensors = parse::parse(&txt).unwrap();
    let mut ranges = vec![];
    for s in sensors.iter() {
        let radius = s.location.distance(&s.beacon);
        if (s.location.y - row).abs() <= radius {
            let half_range = radius - (s.location.y - row).abs();
            ranges.push(Range {
                first: s.location.x - half_range,
                last: s.location.x + half_range,
            });
        }
    }
    let ranges: Vec<Range> = Collapse::new(ranges).collect();

    let count: i32 = ranges.iter().map(|r| r.last - r.first).sum();

    format!("{}", count)
}

fn p2(filename: &str, limit: i64) -> String {
    let txt = std::fs::read_to_string(filename).unwrap();
    let sensors = parse::parse(&txt).unwrap();
    let mut tuning = 0;
    for y in 0..=limit as i32 {
        let mut ranges: Vec<Range> = vec![];

        for s in sensors.iter() {
            let radius = s.location.distance(&s.beacon);
            if (s.location.y - y).abs() <= radius {
                let half_range = radius - (s.location.y - y).abs();
                ranges.push(Range {
                    first: s.location.x - half_range,
                    last: s.location.x + half_range,
                });
            }
        }
        let ranges: Vec<Range> = Collapse::new(ranges).collect();

        if ranges.len() == 2 {
            tuning = 4000000 * (ranges[0].last as i64 + 1 as i64) + y as i64;
        }
    }
    format!("{}", tuning)
}
#[derive(Debug)]
pub struct Sensor {
    pub location: Coords,
    pub beacon: Coords,
}
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Range {
    first: i32,
    last: i32,
}
pub struct Collapse {
    ranges: VecDeque<Range>,
}

impl Collapse {
    fn new(mut src: Vec<Range>) -> Collapse {
        src.sort();
        Collapse {
            ranges: VecDeque::<Range>::from(src),
        }
    }
}
impl Iterator for Collapse {
    type Item = Range;

    fn next(&mut self) -> Option<Self::Item> {
        match self.ranges.pop_front() {
            Some(mut first) => {
                let mut done = false;
                while !done {
                    match self.ranges.pop_front() {
                        Some(second) => {
                            if first.last + 1 >= second.first {
                                first.last = first.last.max(second.last);
                            } else {
                                self.ranges.push_front(second);
                                done = true;
                            }
                        }
                        None => done = true,
                    }
                }
                Some(first)
            }
            None => None,
        }
    }
}
mod parse {
    use super::coords::Coords;
    use nom::{
        bytes::complete::tag,
        character::complete::{i32, multispace1},
        multi::separated_list1,
        sequence::preceded,
        IResult,
    };

    use crate::Sensor;
    pub fn parse(txt: &str) -> Result<Vec<Sensor>, String> {
        match separated_list1(multispace1, parse_sensor)(txt) {
            Err(e) => Err(format!("Error parsing input: {:?}", e)),
            Ok(("", v)) => Ok(v),
            Ok((junk, _)) => Err(format!("Error parsing, junk at end: {}", junk)),
        }
    }
    fn parse_sensor(txt: &str) -> IResult<&str, Sensor> {
        let (rest, location_x) = preceded(tag("Sensor at x="), i32)(txt)?;
        let (rest, location_y) = preceded(tag(", y="), i32)(rest)?;
        let (rest, beacon_x) = preceded(tag(": closest beacon is at x="), i32)(rest)?;
        let (rest, beacon_y) = preceded(tag(", y="), i32)(rest)?;
        Ok((
            rest,
            Sensor {
                location: Coords {
                    x: location_x,
                    y: location_y,
                },
                beacon: Coords {
                    x: beacon_x,
                    y: beacon_y,
                },
            },
        ))
    }
}
