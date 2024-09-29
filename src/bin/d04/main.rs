fn main() {
    let puzzle_file = std::env::args()
        .nth(1)
        .expect("Error: Called without input");
    let puzzle_data = std::fs::read_to_string(puzzle_file).expect("Error: Invalid file.");
    let pairs: Vec<(Range, Range)> = puzzle_data.lines().map(parse_pairs).collect();
    println!(
        "Part 1: {}",
        pairs
            .iter()
            .filter(|(r1, r2)| r2.contains(r1) || r1.contains(r2))
            .count()
    );
    println!(
        "Part 2: {}",
        pairs.iter().filter(|(r1, r2)| r1.overlaps(r2)).count()
    );
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Range {
    from: i32,
    to: i32,
}

impl Range {
    pub fn overlaps(self, other: &Range) -> bool {
        self.from == other.from
            || (self.from < other.from && self.to >= other.from)
            || (self.from > other.from && self.from <= other.to)
    }
    pub fn contains(self, other: &Range) -> bool {
        self.from <= other.from && self.to >= other.to
    }
}

fn parse_pairs(line: &str) -> (Range, Range) {
    let comma: usize = line.find(|c| c == ',').unwrap();

    (
        parse_range(&line[0..comma]),
        parse_range(&line[(comma + 1)..(line.len())]),
    )
}

fn parse_range(span: &str) -> Range {
    let dash = span.find(|c| c == '-').unwrap();
    Range {
        from: span[0..dash].parse().unwrap(),
        to: span[(dash + 1)..(span.len())].parse().unwrap(),
    }
}

#[test]
fn test_parse_pairs() {
    assert_eq!(
        parse_pairs("372-372,350-400"),
        (Range { from: 372, to: 372 }, Range { from: 350, to: 400 })
    );
}
