use std::slice::Chunks;

fn main() {
    let puzzle_file = std::env::args().nth(1).expect("Error: Called without input");
    let puzzle_data = std::fs::read_to_string(puzzle_file).expect("Error: Invalid file.");
    println!("Part 1: {}", puzzle_data.lines().map(parse_rucksack).sum::<i32>());
    let puzzle_lines: Vec<&str> = puzzle_data.lines().collect();
    println!("Part 2: {}", puzzle_lines.chunks(3).map(parse_rucksack_p2).sum::<i32>());(/* Chunks<'_, &str> */)
    
}

fn parse_rucksack(line: &str) -> i32 {
    let chars : Vec<char>= line.chars().collect::<Vec<char>>();
    let half = chars.len() / 2;
    let c = chars[0..half].iter().find(|c| chars[half..chars.len()].contains(c)).unwrap();
    priority(*c)
}
fn priority(c: char) -> i32
{    if c.is_lowercase() 
    { c as i32   - 'a' as i32  + 1} else { c as i32 - 'A' as i32 + 27 } 
}

#[test]
fn test_parse_rucksack() {
    assert_eq!(parse_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp"), 16);
    assert_eq!(parse_rucksack("PmmdzqPrVvPwwTWBwg"), 42);
}
fn parse_rucksack_p2(lines: &[&str]) -> i32 {
    priority(lines[0].chars().find(|&c| lines[1].contains(c) && lines[2].contains(c)).unwrap())
}
