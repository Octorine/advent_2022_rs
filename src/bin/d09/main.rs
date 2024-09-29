use std::collections::HashSet;
mod coords;
use coords::Coords;
fn main() {
    let puzzle_file = std::env::args()
        .nth(1)
        .expect("Error: Called without input");
    let puzzle_data = std::fs::read_to_string(puzzle_file).expect("Error: Invalid file.");
    let mut head = Coords { x: 0, y: 0 };
    let mut tail = head;
    let mut visited: HashSet<Coords> = HashSet::new();
    for line in puzzle_data.clone().lines() {
        let v: Vec<&str> = line.split_whitespace().collect();
        (0..v[1].parse().unwrap()).for_each(|_| {
            head.mv(line.chars().nth(0).unwrap());
            tail.follow(head);
            visited.insert(tail);
        });
    }
    println!("Part 1: {}", visited.len());

    let mut rope = vec![Coords { x: 0, y: 0 }; 10];
    let mut visited: HashSet<Coords> = HashSet::new();
    for line in puzzle_data.lines() {
        let v: Vec<&str> = line.split_whitespace().collect();
        (0..v[1].parse().unwrap()).for_each(|_i| {
            rope[0].mv(line.chars().nth(0).unwrap());
            let mut tmp = rope[0];
            for segment in rope.iter_mut() {
                segment.follow(tmp);
                tmp = *segment;
            }
            visited.insert(rope[9]);
        });
    }
    println!("Part 2 {}", visited.len())
}
