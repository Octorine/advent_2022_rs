use std::collections::HashMap;

fn main() {
    let puzzle_file = std::env::args()
        .nth(1)
        .expect("Error: Called without input");
    let puzzle_data = std::fs::read_to_string(puzzle_file).expect("Error: Invalid file.");
    let defs: HashMap<String, Def> = HashMap::from_iter(puzzle_data.lines().map(parse_def));
    let result = monkey_result(&defs, "root");
    println!("Part 1: {}", result.unwrap());
    let p2_answer = if let Some(Def::Op {
        op: _,
        op_dbg: _,
        a,
        b,
    }) = defs.get("root")
    {
        if has_monkey(&defs, a, "humn") {
            find_input(&defs, monkey_result(&defs, b).unwrap(), "humn", a)
        } else {
            find_input(&defs, monkey_result(&defs, a).unwrap(), "humn", b)
        }
    } else {
        0.0
    };
    println!("Part 2: {}", p2_answer);
}
fn has_monkey(defs: &HashMap<String, Def>, start: &str, monkey: &str) -> bool {
    if start == monkey {
        true
    } else {
        match defs.get(start).unwrap() {
            Def::Val(_) => false,
            Def::Op {
                op: _,
                op_dbg: _,
                a,
                b,
            } => has_monkey(defs, a, monkey) || has_monkey(defs, b, monkey),
        }
    }
}

fn find_input(defs: &HashMap<String, Def>, target: f64, input: &str, root: &str) -> f64 {
    if input == root {
        target
    } else {
        match defs.get(root).unwrap() {
            Def::Val(_) => todo!(), // should never get here.
            Def::Op {
                op: _,
                op_dbg,
                a,
                b,
            } => {
                let this_branch;
                let other_branch;
                let monkey_is_a = has_monkey(defs, a, input);
                if monkey_is_a {
                    this_branch = a;
                    other_branch = monkey_result(defs, b).unwrap();
                } else {
                    this_branch = b;
                    other_branch = monkey_result(defs, a).unwrap();
                }

                match op_dbg.as_str() {
                    "+" => find_input(defs, target - other_branch, input, this_branch),
                    "-" => find_input(
                        defs,
                        if monkey_is_a {
                            target + other_branch
                        } else {
                            other_branch - target
                        },
                        input,
                        this_branch,
                    ),
                    "*" => find_input(defs, target / other_branch, input, this_branch),
                    "/" => find_input(
                        defs,
                        if monkey_is_a {
                            target * other_branch
                        } else {
                            other_branch / target
                        },
                        input,
                        this_branch,
                    ),
                    _ => panic!("Invalid op in find_input"),
                }
            }
        }
    }
}

fn monkey_result(defs: &HashMap<String, Def>, root: &str) -> Option<f64> {
    let mut stack = vec!["root".to_string()];
    let mut results: HashMap<String, f64> = HashMap::new();
    while results.get("root").is_none() {
        let current = stack.pop().unwrap();
        if results.get(&current).is_none() {
            let def = defs.get(&current).unwrap();
            match def {
                Def::Val(v) => {
                    results.insert(current, *v);
                }
                Def::Op {
                    op,
                    op_dbg: _,
                    a,
                    b,
                } => {
                    let a_res = results.get(a);
                    let b_res = results.get(b);
                    if a_res.is_some() && b_res.is_some() {
                        results.insert(current, op(*a_res.unwrap(), *b_res.unwrap()));
                    } else {
                        stack.push(current);
                        if a_res.is_none() {
                            stack.push(a.to_string());
                        }
                        if b_res.is_none() {
                            stack.push(b.to_string());
                        }
                    }
                }
            }
        }
    }
    results.get(root).copied()
}
enum Def {
    Val(f64),
    Op {
        op: Box<dyn Fn(f64, f64) -> f64>,
        op_dbg: String,
        a: String,
        b: String,
    },
}
impl std::fmt::Debug for Def {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Def::Val(val) => f.debug_struct("Def::Val").field("val", val).finish(),
            Def::Op { op_dbg, a, b, .. } => f
                .debug_struct("Def::Op")
                .field("op", op_dbg)
                .field("a", a)
                .field("b", b)
                .finish(),
        }
    }
}

fn parse_def(line: &str) -> (String, Def) {
    let words: Vec<&str> = line.split_whitespace().collect();
    if words.len() == 2 {
        (
            String::from(words[0].replace(":", "")),
            Def::Val(words[1].parse().unwrap()),
        )
    } else if words.len() == 4 {
        (
            String::from(words[0].replace(":", "")),
            Def::Op {
                op: match words[2] {
                    "+" => Box::new(|a, b| a + b),
                    "-" => Box::new(|a, b| a - b),
                    "*" => Box::new(|a, b| a * b),
                    "/" => Box::new(|a, b| a / b),
                    _ => panic!("Invalid op"),
                },
                op_dbg: String::from(words[2]),
                a: String::from(words[1]),
                b: String::from(words[3]),
            },
        )
    } else {
        panic!("Invalid definition")
    }
}
