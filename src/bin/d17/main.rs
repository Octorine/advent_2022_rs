mod coords;
use std::collections::HashSet;

use coords::Coords;
fn main() {
    let puzzle_file = std::env::args()
        .nth(1)
        .expect("Error: Called without input");
    let puzzle_data = std::fs::read_to_string(puzzle_file).expect("Error: Invalid file.");
    let gusts: Vec<char> = puzzle_data.chars().collect();
    let block_descriptions = [
        vec!["####"],
        vec![".#.", "###", ".#."],
        vec!["..#", "..#", "###"],
        vec!["#", "#", "#", "#"],
        vec!["##", "##"],
    ];
    let blocks: Vec<Block> = block_descriptions
        .into_iter()
        .map(|lines| Block::new(&lines))
        .collect();
    // Positive y is up.
    // Left wall is at x == 0.
    // Right wall is at 8;
    // Floor is at y == 0;
    let mut current_block: usize = 0;
    let mut total_blocks: usize = 0;
    // pos is the position of the bottom-left corner of the current block.
    let mut pos = Coords::new(2, 3);
    let mut round = 0;
    let mut obstacles: Block = Block::new_empty();
    let mut scorecard = vec![];
    let top = 10_000;
    while total_blocks <= top + 1 {
        match gusts[round % gusts.len()] {
            '<' => {
                let dir = Coords::new(-1, 0);
                if pos.x > 0 && blocks[current_block].can_move(pos + dir, &obstacles) {
                    pos += dir;
                }
            }
            '>' => {
                let dir = Coords::new(1, 0);
                if pos.x + blocks[current_block].width() < 7
                    && blocks[current_block].can_move(pos + dir, &obstacles)
                {
                    pos += dir;
                }
            }
            _ => {
                panic!("Invalid character in input")
            }
        }
        let down = Coords::new(0, -1);
        if pos.y > 0 && blocks[current_block].can_move(pos + down, &obstacles) {
            pos += down;
        } else {
            if total_blocks == 2022 {
                println!("Part 1: {}", obstacles.height())
            }
            scorecard.push((total_blocks, obstacles.height()));
            obstacles
                .cells
                .extend(blocks[current_block].cells.iter().map(|c| *c + pos));
            total_blocks += 1;
            current_block += 1;
            current_block = current_block % blocks.len();
            pos.x = 2;
            pos.y = obstacles.height() + 3;
        }

        round += 1;
    }
    let mut period = 50;

    let s1 = scorecard[top].1;
    let mut s2 = scorecard[top - period].1;
    let mut s3 = scorecard[top - 2 * period].1;
    let mut s4 = scorecard[top - 3 * period].1;
    let mut s5 = scorecard[top - 4 * period].1;
    let mut diff1 = s1 - s2;
    let mut diff2 = s2 - s3;
    let mut diff3 = s3 - s4;
    let mut diff4 = s4 - s5;
    while diff1 != diff2 || diff2 != diff3 || diff3 != diff4 {
        period += 1;
        s2 = scorecard[top - period].1;
        s3 = scorecard[top - 2 * period].1;
        s4 = scorecard[top - 3 * period].1;
        s5 = scorecard[top - 4 * period].1;
        diff1 = s1 - s2;
        diff2 = s2 - s3;
        diff3 = s3 - s4;
        diff4 = s4 - s5;
    }
    // we now know  the period at which the growth of the obstacle stack loops.
    let goal: i64 = 1000000000000;
    let rem = ((goal - top as i64) % (period as i64)) as usize;
    let starting_point = scorecard[top - rem].1;
    let div = (goal - (top as i64)) / (period as i64);

    println!("Part 2: {}", starting_point as i64 + diff1 as i64 * div);
}

#[derive(Debug)]
struct Block {
    cells: HashSet<Coords>,
}
impl Block {
    pub fn new(lines: &[&str]) -> Block {
        let mut cells = HashSet::new();
        let height = lines.len();

        for (line_num, line) in lines.iter().enumerate() {
            for (char_num, char) in line.chars().enumerate() {
                if char == '#' {
                    cells.insert(Coords::new(
                        char_num as i32,
                        height as i32 - line_num as i32 - 1,
                    ));
                }
            }
        }

        Block { cells }
    }
    pub fn new_empty() -> Block {
        Block {
            cells: HashSet::new(),
        }
    }

    pub fn width(&self) -> i32 {
        self.cells.iter().map(|c| c.x).max().unwrap_or(0) + 1
    }
    pub fn height(&self) -> i32 {
        self.cells.iter().map(|c| c.y).max().unwrap_or(0) + 1
    }
    pub fn can_move(&self, dir: Coords, other: &Block) -> bool {
        !self.cells.iter().any(|c| other.cells.contains(&(*c + dir)))
    }
    #[allow(dead_code)]
    pub fn draw(&self) {
        for j in 0..self.height() {
            for i in 0..self.width() {
                if self.cells.contains(&Coords::new(i, self.height() - j - 1)) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }
}
