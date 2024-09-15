fn main() {
    let puzzle_file = std::env::args()
        .nth(1)
        .expect("Error: Called without input");
    let puzzle_data = std::fs::read_to_string(&puzzle_file).expect("Error: Invalid file.");
    println!("Part 1: {}\nPart 2: {}", p1(&puzzle_file), p2(&puzzle_file));
}

fn p1(filename: &str) -> String {
    let txt = std::fs::read_to_string(filename).unwrap();
    let mut acc = 1;
    let mut total = 0;
    let mut cycle = 1;
    let mut code = txt.lines().chain(vec!["noop"].into_iter().cycle());
    while cycle < 300 {
        let line = code.next().unwrap();
        let words: Vec<&str> = line.split_whitespace().collect();
        if words[0] == "addx" {
            if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                total += cycle as i32 * acc;
            }
            cycle += 1;
            if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                total += cycle as i32 * acc;
            }
            acc += words[1].parse::<i32>().unwrap();
            cycle += 1;
        } else {
            if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                total += cycle as i32 * acc;
            }
            cycle += 1;
        }
    }
    total.to_string()
}

fn p2(filename: &str) -> String {
    let txt = std::fs::read_to_string(filename).unwrap();
    let mut acc = 1;
    let mut cycle: i32 = 1;
    let mut code = txt.lines().chain(vec!["noop"].into_iter().cycle());
    while cycle < 1000 {
        let line = code.next().unwrap();
        let words: Vec<&str> = line.split_whitespace().collect();
        if words[0] == "addx" {
            pixel(cycle, acc);
            cycle += 1;
            pixel(cycle, acc);
            acc += words[1].parse::<i32>().unwrap();
            cycle += 1;
        } else {
            pixel(cycle, acc);
            cycle += 1;
        }
    }

    "NO OUTPUT".to_string()
}
pub fn pixel(cycle: i32, acc: i32) {
    print!(
        "{}",
        if ((cycle - 1) % 40 - acc).abs() <= 1 {
            '#'
        } else {
            '.'
        }
    );
    if (cycle - 1) % 40 == 39 {
        println!("");
    }
}
