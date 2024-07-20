fn main() {
    let puzzle_file = std::env::args().nth(1).expect("Error: Called without input");
    let puzzle_data = std::fs::read_to_string(puzzle_file).expect("Error: Invalid file.");
    println!("Part 1: {}", find_start_of_packet(&puzzle_data, 4));
    println!("Part 1: {}", find_start_of_packet(&puzzle_data, 14));
}

fn find_start_of_packet(s : &str, size: usize) -> usize {
    let mut stream = s.chars();
    let mut buffer : Vec<char> = stream.clone().take(size).collect();
    let mut i = 0;
    let mut sorted = buffer.clone();
    sorted.sort();
    while sorted.windows(2).any(|arr| arr[0] == arr[1]){
	    buffer [i % size] = stream.next().unwrap();
	i += 1;
	sorted = buffer.clone();
	sorted.sort();
	}
    i}

	
