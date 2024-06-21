fn main() {
    let puzzle_file = std::env::args().nth(1).expect("Error: Called without input");
    let puzzle_data = std::fs::read_to_string(puzzle_file).expect("Error: Invalid file.");
    let matches : i32 = puzzle_data.lines().map(|line| {
        let (him, me) = parse_line_p1(line);
        let outcome = me.play(him);
        me.score() + outcome.score()
    }).sum();
    println!("Part one: {}", matches);
    let matches_p2 : i32 = puzzle_data.lines().map(|line| {
        let (him, outcome) = parse_line_p2(line);
        let me = him.response(outcome);
        me.score() + outcome.score()
    }).sum();
    println!("Part two: {}", matches_p2);
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

impl RPS {
    fn play(self, other: RPS) -> MatchResult {
        match (self, other) {
            (RPS::Rock, RPS::Paper) | (RPS::Paper, RPS::Scissors) | (RPS::Scissors, RPS::Rock) => MatchResult::Lose,
            (RPS::Paper, RPS::Rock) | ( RPS::Scissors, RPS::Paper) | (RPS::Rock, RPS::Scissors) => MatchResult::Win,
            _ => MatchResult::Draw
        }
    }
    fn response(self, outcome: MatchResult) -> RPS {
        match (self, outcome) {
            (RPS::Paper, MatchResult::Lose) | (RPS::Rock, MatchResult::Draw) | (RPS::Scissors, MatchResult::Win) => RPS::Rock,
            (RPS::Paper, MatchResult::Draw) | (RPS::Scissors, MatchResult::Lose) | (RPS::Rock, MatchResult::Win) => RPS::Paper,
            (RPS::Paper, MatchResult::Win) | (RPS::Scissors, MatchResult::Draw) | (RPS::Rock, MatchResult::Lose) => RPS::Scissors,

        }
    }
    fn score(self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3
        }
    }
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum MatchResult {
    Win,
    Lose,
    Draw
}

impl MatchResult {
    fn score(self) -> i32 {
        match self {
            MatchResult::Win => 6,
            MatchResult::Lose => 0,
            MatchResult::Draw => 3
        }
    }
}

fn parse_line_p1(line: &str) -> (RPS, RPS) {
    let him = match line.chars().nth(0).unwrap(){
        'A' => RPS::Rock,
        'B' => RPS::Paper,
        'C' => RPS::Scissors,
        _ => panic!("Invalid value for him")
    };
    let me = match line.chars().nth(2).unwrap() {
        'X' => RPS::Rock,
        'Y' => RPS::Paper,
        'Z' => RPS::Scissors,
        _ => panic!("Invalid value for me")
    };
    (him, me)
}

fn parse_line_p2(line: &str) -> (RPS, MatchResult) {
    let him = match line.chars().nth(0).unwrap(){
        'A' => RPS::Rock,
        'B' => RPS::Paper,
        'C' => RPS::Scissors,
        _ => panic!("Invalid value for him")
    };
    let outcome = match line.chars().nth(2).unwrap() {
        'X' => MatchResult::Lose,
        'Y' => MatchResult::Draw,
        'Z' =>  MatchResult::Win,
        _ => panic!("Invalid value for me")
    };
    (him, outcome)
}
