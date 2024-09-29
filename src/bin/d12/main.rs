mod coords;
use std::collections::{HashMap, VecDeque};

use coords::Coords;
fn main() {
    let puzzle_file = std::env::args()
        .nth(1)
        .expect("Error: Called without input");

    println!("Part 1: {}", p1(&puzzle_file));
    println!("Part 2: {}", p2(&puzzle_file));
}
fn p1(filename: &str) -> String {
    let txt = std::fs::read_to_string(filename).unwrap();
    let chars: Vec<Vec<char>> = txt.lines().map(|line| line.chars().collect()).collect();
    let start = find_start(&chars);
    let end = find_end(&chars);
    let &dist = get_dists(&chars, end).get(&start).unwrap();

    format!("{}", dist.to_string())
}

fn p2(filename: &str) -> String {
    let txt = std::fs::read_to_string(filename).unwrap();
    let chars: Vec<Vec<char>> = txt.lines().map(|line| line.chars().collect()).collect();
    let starts = find_starts(&chars);
    let end = find_end(&chars);
    let dists = get_dists(&chars, end);

    format!(
        "{}",
        starts
            .into_iter()
            .filter_map(|s| dists.get(&s))
            .min()
            .unwrap()
    )
}

fn find_start(chars: &Vec<Vec<char>>) -> Coords {
    find_char(chars, 'S')
}

fn find_end(chars: &Vec<Vec<char>>) -> Coords {
    find_char(chars, 'E')
}

fn find_starts(chars: &Vec<Vec<char>>) -> Vec<Coords> {
    let mut ends = vec![];
    for j in 0..chars.len() {
        for i in 0..chars[0].len() {
            if chars[j][i] == 'a' || chars[j][i] == 'S' {
                ends.push(Coords {
                    x: i as i32,
                    y: j as i32,
                });
            }
        }
    }
    ends
}

fn find_char(chars: &Vec<Vec<char>>, c: char) -> Coords {
    for j in 0..chars.len() {
        for i in 0..chars[0].len() {
            if chars[j][i] == c {
                return Coords {
                    x: i as i32,
                    y: j as i32,
                };
            }
        }
    }
    Coords { x: 0, y: 0 }
}

fn get_char(chars: &Vec<Vec<char>>, row: i32, column: i32) -> char {
    let c = chars[row as usize][column as usize];
    if c == 'E' {
        'z'
    } else if c == 'S' {
        'a'
    } else {
        c
    }
}

fn get_dists(chars: &Vec<Vec<char>>, end: Coords) -> HashMap<Coords, i32> {
    let dirs = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    let mut fringe = VecDeque::new();
    let mut dists: HashMap<Coords, i32> = HashMap::new();
    dists.insert(end, 0);
    for (i, j) in dirs
        .clone()
        .into_iter()
        .map(|(x, y)| (x + end.x, y + end.y))
    {
        if i >= 0
            && i < chars[0].len() as i32
            && j >= 0
            && j < chars.len() as i32
            && (get_char(chars, j, i) == 'z' || get_char(chars, j, i) == 'y')
        {
            let c = Coords { x: i, y: j };
            fringe.push_back(c);
            dists.insert(c, 1);
        }
    }
    while !fringe.is_empty() {
        let current = fringe.pop_front().unwrap();
        for (i, j) in dirs
            .into_iter()
            .map(|(x, y)| (x + current.x, y + current.y))
        {
            if i >= 0
                && i < chars[0].len() as i32
                && j >= 0
                && j < chars.len() as i32
                && (get_char(&chars, j as i32, i as i32) as u32
                    >= get_char(&chars, current.y, current.x) as u32 - 1)
            {
                let neighbor = Coords { x: i, y: j };
                let current_dist = dists.get(&current).unwrap();
                if let Some(&old_dist) = dists.get(&neighbor) {
                    dists.insert(neighbor, old_dist.min(current_dist + 1));
                } else {
                    dists.insert(neighbor, current_dist + 1);
                    fringe.push_back(neighbor);
                }
            }
        }
    }

    dists
}
