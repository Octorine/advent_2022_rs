fn main() {
    let puzzle_file = std::env::args()
        .nth(1)
        .expect("Error: Called without input");
    let puzzle_data = std::fs::read_to_string(puzzle_file).expect("Error: Invalid file.");
    let numbers: Vec<(usize, i64)> = puzzle_data
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .enumerate()
        .collect::<Vec<(usize, i64)>>();
    let working = mix(&numbers, 1);
    let zero = working.iter().position(|n| n.1 == 0).unwrap();
    let c1 = (zero + 1000) % (&working.len());
    let c2 = (zero + 2000) % (&working.len());
    let c3 = (zero + 3000) % (&working.len());

    println!("Part 1: {}", working[c1].1 + working[c2].1 + working[c3].1);
    let n2: Vec<(usize, i64)> = numbers
        .into_iter()
        .map(|(i, value)| (i, value * 811589153))
        .collect();
    let w2 = mix(&n2, 10);

    let zero_2 = w2.iter().position(|n| n.1 == 0).unwrap();
    let c1_2 = (zero_2 + 1000) % (&w2.len());
    let c2_2 = (zero_2 + 2000) % (&w2.len());
    let c3_2 = (zero_2 + 3000) % (&w2.len());
    println!("Part 2: {}", w2[c1_2].1 + w2[c2_2].1 + w2[c3_2].1);
}

fn mix(numbers: &Vec<(usize, i64)>, times: usize) -> Vec<(usize, i64)> {
    let mut working = numbers.clone();
    (0..times).for_each(|t| {
        for &(index, number) in numbers.iter() {
            rot(&mut working, index, number);
        }
    });
    working
}

fn move_by_index<T: Copy>(v: &mut Vec<T>, i: i64, dist: i64) {
    let v_len = v.len() as i64;
    let dist = dist % (v_len - 1);
    let value = v.remove(i as usize);
    let new_index = i + dist;
    if new_index >= 0 && new_index <= v_len - 1 {
        v.insert(new_index as usize, value);
    } else if new_index < 0 {
        v.insert((v_len + new_index - 1) as usize, value);
    } else {
        // new_index >= v_len - 1
        v.insert((new_index + 1 - v_len) as usize, value);
    }
}

fn rot(v: &mut Vec<(usize, i64)>, i: usize, dist: i64) {
    let index = v
        .iter()
        .position(|(i2, _)| i2 == &i)
        .expect(&format!("Couldn't find {} in working set", &i)) as i64;
    move_by_index(v, index, dist);
}
#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn test_moves() {
        let mut v = vec![4, 5, 6, 1, 7, 8, 9];

        move_by_index(&mut v, 3, 1);
        assert_eq!(&v, &vec![4, 5, 6, 7, 1, 8, 9]);

        move_by_index(&mut v, 1, -2);
        assert_eq!(&v, &vec![4, 6, 7, 1, 8, 5, 9]);
    }
    #[test]
    fn there_and_back() {
        let mut v = vec![1, 2, 3, 4, 5];

        move_by_index(&mut v, 0, 1);
        assert_eq!(&v, &vec![2, 1, 3, 4, 5]);

        move_by_index(&mut v, 1, 1);
        assert_eq!(&v, &vec![2, 3, 1, 4, 5]);

        move_by_index(&mut v, 2, 1);
        assert_eq!(&v, &vec![2, 3, 4, 1, 5]);

        move_by_index(&mut v, 3, 1);
        assert_eq!(&v, &vec![2, 3, 4, 5, 1]);

        move_by_index(&mut v, 4, 1);
        assert_eq!(&v, &vec![2, 1, 3, 4, 5]);

        move_by_index(&mut v, 1, -1);
        assert_eq!(&v, &vec![1, 2, 3, 4, 5]);

        move_by_index(&mut v, 0, -1);
        assert_eq!(&v, &vec![2, 3, 4, 1, 5]);

        move_by_index(&mut v, 3, -1);
        assert_eq!(&v, &vec![2, 3, 1, 4, 5]);

        move_by_index(&mut v, 2, -1);
        assert_eq!(&v, &vec![2, 1, 3, 4, 5]);

        move_by_index(&mut v, 1, -1);
        assert_eq!(&v, &vec![1, 2, 3, 4, 5]);
    }
    #[test]
    fn thousands() {
        let mut v = vec![1, 2, 3, 4, 5];

        move_by_index(&mut v, 0, 1000);
        assert_eq!(&v, &vec![1, 2, 3, 4, 5]);

        move_by_index(&mut v, 1, 1000);
        assert_eq!(&v, &vec![1, 2, 3, 4, 5]);

        move_by_index(&mut v, 2, 1000);
        assert_eq!(&v, &vec![1, 2, 3, 4, 5]);

        move_by_index(&mut v, 3, 1000);
        assert_eq!(&v, &vec![1, 2, 3, 4, 5]);

        move_by_index(&mut v, 4, 1000);
        assert_eq!(&v, &vec![1, 2, 3, 4, 5]);
    }
    #[test]
    fn test_sample() {
        let mut sample = vec![1, -3, 2, 3, -2, 0, 4];
        move_by_index(&mut sample, 1, -3);
        assert_eq!(&sample, &vec![1, 2, 3, -2, -3, 0, 4]);
    }
}
