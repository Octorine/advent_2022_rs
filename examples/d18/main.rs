use std::collections::{HashSet, VecDeque};

fn main() {
    let puzzle_file = std::env::args()
        .nth(1)
        .expect("Error: Called without input");
    let puzzle_data = std::fs::read_to_string(puzzle_file).expect("Error: Invalid file.");
    let sides: HashSet<Coords> = puzzle_data.lines().map(parse_line).collect();
    let outsides: i32 = sides
        .iter()
        .map(|c| {
            (if sides.contains(&(c.plus(Coords::POS_X))) {
                0
            } else {
                1
            }) + (if sides.contains(&(c.plus(Coords::NEG_X))) {
                0
            } else {
                1
            }) + (if sides.contains(&(c.plus(Coords::POS_Y))) {
                0
            } else {
                1
            }) + (if sides.contains(&(c.plus(Coords::NEG_Y))) {
                0
            } else {
                1
            }) + (if sides.contains(&(c.plus(Coords::POS_Z))) {
                0
            } else {
                1
            }) + (if sides.contains(&(c.plus(Coords::NEG_Z))) {
                0
            } else {
                1
            })
        })
        .sum();
    println!("Part 1: {}", outsides);
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    let min_x = sides.iter().map(|s| s.x).min().unwrap() - 1;
    let min_y = sides.iter().map(|s| s.y).min().unwrap() - 1;
    let min_z = sides.iter().map(|s| s.z).min().unwrap() - 1;
    let max_x = sides.iter().map(|s| s.x).max().unwrap() + 1;
    let max_y = sides.iter().map(|s| s.y).max().unwrap() + 1;
    let max_z = sides.iter().map(|s| s.z).max().unwrap() + 1;
    let min_corner = Coords::new(min_x, min_y, min_z);
    let max_corner = Coords::new(max_x, max_y, max_z);
    let mut current = min_corner;

    let mut real_outsides = 0;
    add_neighbors(
        current,
        &mut q,
        &mut real_outsides,
        &mut visited,
        min_corner,
        max_corner,
        &sides,
    );
    while !q.is_empty() {
        current = q.pop_front().unwrap();
        add_neighbors(
            current,
            &mut q,
            &mut real_outsides,
            &mut visited,
            min_corner,
            max_corner,
            &sides,
        );
    }
    println!("Part 2: {}", real_outsides);
}

fn add_neighbors(
    current: Coords,
    q: &mut VecDeque<Coords>,
    real_outsides: &mut i32,
    visited: &mut HashSet<Coords>,
    min_corner: Coords,
    max_corner: Coords,
    sides: &HashSet<Coords>,
) {
    let dirs = vec![
        Coords::POS_X,
        Coords::NEG_X,
        Coords::POS_Y,
        Coords::NEG_Y,
        Coords::POS_Z,
        Coords::NEG_Z,
    ];
    *real_outsides += dirs
        .iter()
        .filter(|dir| sides.contains(&(dir.plus(current))))
        .count() as i32;
    for dir in dirs.iter() {
        let candidate = current.plus(*dir);
        if !visited.contains(&candidate)
            && !sides.contains(&candidate)
            && !q.contains(&candidate)
            && candidate.x >= min_corner.x
            && candidate.y >= min_corner.y
            && candidate.z >= min_corner.z
            && candidate.x <= max_corner.x
            && candidate.y <= max_corner.y
            && candidate.z <= max_corner.z
        {
            q.push_back(candidate);
        }
    }
    visited.insert(current);
}

fn parse_line(line: &str) -> Coords {
    let mut i = line.split_terminator(',');

    Coords::new(
        i.next().unwrap().parse().unwrap(),
        i.next().unwrap().parse().unwrap(),
        i.next().unwrap().parse().unwrap(),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords {
    x: i32,
    y: i32,
    z: i32,
}

impl Coords {
    pub fn new(x: i32, y: i32, z: i32) -> Coords {
        Coords { x, y, z }
    }
    pub fn plus(&self, other: Coords) -> Coords {
        Coords::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
    pub const POS_X: Coords = Coords { x: 1, y: 0, z: 0 };
    pub const NEG_X: Coords = Coords { x: -1, y: 0, z: 0 };
    pub const POS_Y: Coords = Coords { x: 0, y: 1, z: 0 };
    pub const NEG_Y: Coords = Coords { x: 0, y: -1, z: 0 };
    pub const POS_Z: Coords = Coords { x: 0, y: 0, z: 1 };
    pub const NEG_Z: Coords = Coords { x: 0, y: 0, z: -1 };
}
