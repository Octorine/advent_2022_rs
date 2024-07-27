use std::collections::HashMap;
fn main() {
    let puzzle_file = std::env::args()
        .nth(1)
        .expect("Error: Called without input");
    let puzzle_data = std::fs::read_to_string(puzzle_file).expect("Error: Invalid file.");
    let mut crane = parse_crane(&puzzle_data);
    let mut crane_2 = crane.clone();
    for (moves, from, to) in crane.instructions.iter() {
        do_move(&mut crane.start, *moves, *from, *to);
    }

    for i in 1..crane.start.len() + 1 {
        print!("{}", crane.start.get(&i).unwrap().clone().pop().unwrap());
    }
    println!("");

    for (moves, from, to) in crane_2.instructions.iter() {
        do_move_2(&mut crane_2.start, *moves, *from, *to);
    }

    for i in 1..crane_2.start.len() + 1 {
        print!("{}", crane_2.start.get(&i).unwrap().clone().pop().unwrap());
    }
    println!("");
}
#[derive(Debug, Clone)]
struct Crane<'a> {
    start: HashMap<usize, Vec<&'a str>>,
    instructions: Vec<(usize, usize, usize)>,
}

fn parse_crane(input: &str) -> Crane {
    let start = parse_strt(&mut input.lines().take_while(|line| *line != ""));
    let instructions = parse_instructions(&mut input.lines().skip_while(|line| *line != ""));
    Crane {
        start,
        instructions,
    }
}

fn parse_strt<'a, I: Iterator<Item = &'a str>>(i: I) -> HashMap<usize, Vec<&'a str>> {
    let mut map: HashMap<usize, Vec<&'a str>> = HashMap::new();
    for line in i {
        if line.contains("[") {
            for k in 1..10 {
                if let Some(v) = line.get(((k - 1) * 4)..(k * 4 - 1)) {
                    if v != "   " {
                        let e = map.entry(k).or_insert(Vec::new());
                        e.push(v.trim());
                    }
                }
            }
        }
    }
    let keys: Vec<usize> = map.keys().cloned().collect();
    keys.iter().for_each(|k| {
        map.entry(*k).and_modify(|e| e.reverse());
    });
    map
}

fn parse_instructions<'a, I: Iterator<Item = &'a str>>(i: I) -> Vec<(usize, usize, usize)> {
    i.filter_map(|line_txt| {
        let words: Vec<&str> = line_txt.split_terminator(" ").collect();
        if let Some(moves) = words.get(1).and_then(|n| n.parse::<usize>().ok()) {
            if let Some(from) = words.get(3).and_then(|n| n.parse::<usize>().ok()) {
                if let Some(to) = words.get(5).and_then(|n| n.parse::<usize>().ok()) {
                    Some((moves as usize, from as usize, to as usize))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    })
    .collect()
}

fn do_move(map: &mut HashMap<usize, Vec<&str>>, moves: usize, from: usize, to: usize) {
    for _i in 0..moves {
        match map.get_mut(&from).unwrap().pop() {
            Some(tmp) => map.get_mut(&to).unwrap().push(tmp),
            None => (),
        }
    }
}

fn do_move_2(map: &mut HashMap<usize, Vec<&str>>, moves: usize, from: usize, to: usize) {
    for _i in 0..moves {
        let mut tmp = vec![];
        match map.get_mut(&from).unwrap().pop() {
            Some(value) => tmp.push(value),
            None => (),
        }
        tmp.reverse();
        for v in tmp.iter() {
            map.get_mut(&to).unwrap().push(v);
        }
    }
}
