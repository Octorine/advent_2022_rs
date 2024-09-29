mod coords;
use std::collections::HashMap;

use coords::Coords;
fn main() {
    let puzzle_file = std::env::args()
        .nth(1)
        .expect("Error: Called without input");
    println!("Part 1: {}", p1(&puzzle_file));
    println!("Part 2: {}", p2(&puzzle_file));
    fn p1(filename: &str) -> String {
        let txt = std::fs::read_to_string(filename).unwrap();
        let paths = parse::parse(&txt).unwrap();
        let mut env = make_env(paths);
        let bottom = find_bottom(&env).unwrap();
        let mut score = 0;
        let down = Coords { x: 0, y: 1 };
        let left = Coords { x: -1, y: 1 };
        let right = Coords { x: 1, y: 1 };
        let mut drip = Coords { x: 500, y: 0 };
        loop {
            if drip.y >= bottom {
                return format!("{}", score);
            }
            if env.get(&(drip + down)) == Some(&'#') {
                if env.get(&(drip + left)) == Some(&'#') {
                    if env.get(&(drip + right)) == Some(&'#') {
                        env.insert(drip, '#');
                        drip = Coords { x: 500, y: 0 };
                        score += 1;
                    } else {
                        drip += right;
                    }
                } else {
                    drip += left;
                }
            } else {
                drip += down;
            }
        }
    }

    fn find_bottom(env: &HashMap<Coords, char>) -> Option<i32> {
        env.keys().map(|c| c.y).max()
    }

    fn p2(filename: &str) -> String {
        let txt = std::fs::read_to_string(filename).unwrap();
        let paths = parse::parse(&txt).unwrap();
        let mut env = make_env(paths);
        let bottom = find_bottom(&env).unwrap() + 1;
        let mut score = 0;
        let down = Coords { x: 0, y: 1 };
        let left = Coords { x: -1, y: 1 };
        let right = Coords { x: 1, y: 1 };
        let mut drip = Coords { x: 500, y: 0 };
        loop {
            if drip.y == 0 && env.get(&drip) == Some(&'#') {
                return format!("{}", score);
            }
            if drip.y >= bottom {
                env.insert(drip, '#');
                drip = Coords { x: 500, y: 0 };
                score += 1;
            } else if env.get(&(drip + down)) == Some(&'#') {
                if env.get(&(drip + left)) == Some(&'#') {
                    if env.get(&(drip + right)) == Some(&'#') {
                        env.insert(drip, '#');
                        drip = Coords { x: 500, y: 0 };
                        score += 1;
                    } else {
                        drip += right;
                    }
                } else {
                    drip += left;
                }
            } else {
                drip += down;
            }
        }
    }

    type Path = Vec<Coords>;

    fn make_env(paths: Vec<Path>) -> HashMap<Coords, char> {
        let mut env: HashMap<Coords, char> = HashMap::new();
        for path in paths.into_iter() {
            for span in path.windows(2) {
                let p1 = span[0];
                let p2 = span[1];
                let step = norm(p2 - p1);
                let mut p = p1;
                while p != p2 {
                    env.insert(p, '#');
                    p += step;
                }
                env.insert(p, '#');
            }
        }
        env
    }
    fn norm(v: Coords) -> Coords {
        let len = (v.x + v.y).abs();
        Coords {
            x: v.x / len,
            y: v.y / len,
        }
    }
    mod parse {

        use super::coords::Coords;
        use nom::character::complete::{i32, multispace1};
        use nom::multi::separated_list1;
        use nom::{bytes::complete::tag, combinator::map, sequence::separated_pair, IResult};

        pub fn parse(txt: &str) -> Result<Vec<Vec<Coords>>, String> {
            match paths_parser(txt) {
                Ok(("", result)) => Ok(result),
                Ok((rest, _)) => Err(format!("Parse error: leftovers [{}]", rest)),
                Err(e) => Err(format!("Parse Error: {:?}", e)),
            }
        }
        fn paths_parser(txt: &str) -> IResult<&str, Vec<Vec<Coords>>> {
            separated_list1(multispace1, path_parser)(txt)
        }
        fn coord_parser(txt: &str) -> IResult<&str, Coords> {
            map(separated_pair(i32, tag(","), i32), |(x, y)| Coords { x, y })(txt)
        }

        fn path_parser(txt: &str) -> IResult<&str, Vec<Coords>> {
            separated_list1(tag(" -> "), coord_parser)(txt)
        }
    }
}
