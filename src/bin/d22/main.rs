mod coords;
mod cube_game;
mod flat_game;
mod board;
mod game;
use game::*;
use cube_game::*;
use flat_game::*;

fn main() {
    let puzzle_file = std::env::args()
        .nth(1)
        .expect("Error: Called without input");
    let puzzle_data = std::fs::read_to_string(puzzle_file).expect("Error: Invalid file.");
    let (board, path) = parse_input(&puzzle_data);
    let mut g = Game::<FlatBoard>::new(board);

    g.find_player();
    g.follow_path(path);

    println!("Part 1: {}", g.score());

    let mut g2 = Game::<CubeBoard>::new(board);
    g2.find_player();
    g2.follow_path(path);
    println!("Part 2: {}", g2.score());
}

fn parse_input(puzzle_data: &str) -> (&str, &str) {

    puzzle_data.split_once("\n\n").unwrap()
}
