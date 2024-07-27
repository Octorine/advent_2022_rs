fn main() {
    let puzzle_file = std::env::args()
        .nth(1)
        .expect("Error: Called without input");
    println!("Opening file {}.", &puzzle_file);
    let puzzle_data = std::fs::read_to_string(puzzle_file).expect("Error: Invalid file.");
    let puzzle_lines: Vec<&str> = puzzle_data.lines().collect();
    let backpacks: Vec<Vec<i32>> = puzzle_lines
        .split(|&elt| elt == "")
        .map(|ss| ss.iter().filter_map(|&s| s.parse().ok()).collect())
        .collect();
    println!(
        "Part 1: {}.",
        backpacks
            .iter()
            .map(|b| b.iter().sum::<i32>())
            .max()
            .unwrap()
    );
    let mut sums: Vec<i32> = backpacks.iter().map(|b| b.iter().sum::<i32>()).collect();
    sums.sort();
    sums.reverse();
    println!("Part 1: {}.", sums.iter().take(3).sum::<i32>());
}
