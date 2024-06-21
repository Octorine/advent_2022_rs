fn main() {
    let puzzle_file = std::env::args().nth(1).expect("Error: Called without input");
    println!("Opening file {}.", &puzzle_file);
    let puzzle_data = std::fs::read_to_string(puzzle_file).expect("Error: Invalid file.");
    println!("{}", puzzle_data);
}
