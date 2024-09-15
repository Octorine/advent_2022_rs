use itertools::Itertools;
use petgraph::{algo::dijkstra, Directed, Graph};
use std::collections::HashMap;

fn main() {
    let puzzle_file = std::env::args()
        .nth(1)
        .expect("Error: Called without input");
    println!("Part 1: {}", p1(&puzzle_file));
    println!("Part 2: {}", p2(&puzzle_file));
}

fn p1(filename: &str) -> String {
    let txt = std::fs::read_to_string(filename).unwrap();
    let valves = parse::parse(&txt).unwrap();
    let start = valves.iter().find(|v| v.name == "AA").unwrap();
    let g = build_graph(&valves);
    let unvisited: Vec<&Valve> = valves
        .iter()
        .filter(|v| v.name != "AA" && v.value > 0)
        .collect();

    let path_lengths = get_path_lengths(&g, &unvisited);

    let mut bs = BestState::new();

    let state = State::new(start, &valves, &path_lengths, 30);
    let total_pressure = get_high_score(state, &mut bs);

    format!("{}", total_pressure.0)
}
fn get_high_score(state: State, bs: &mut BestState) -> (i32, Vec<String>) {
    bs.add(&state);
    if state.minutes <= 0 {
        (state.pressure, state.path)
    } else if state.unvisited.is_empty() {
        (state.pressure, state.path)
    } else {
        state
            .unvisited
            .iter()
            .map(|v| get_high_score(state.move_to(v), bs))
            .max()
            .unwrap()
    }
}

fn get_path_lengths<'a>(
    g: &Graph<String, ()>,
    pressure_valves: &'a [&Valve],
) -> HashMap<(&'a str, &'a str), i32> {
    let nodes: HashMap<_, _> = g
        .node_indices()
        .map(|i| (g.node_weight(i).unwrap(), i))
        .collect();
    let mut lengths: HashMap<(&str, &str), i32> = HashMap::new();
    let v1 = nodes.get(&"AA".to_string()).unwrap();
    let dij = dijkstra(g, *v1, None, |_| 1);
    for v2 in pressure_valves.iter() {
        lengths.insert(
            ("AA", &v2.name),
            *dij.get(nodes.get(&v2.name).unwrap()).unwrap(),
        );
    }
    for v1 in pressure_valves.iter() {
        for v2 in pressure_valves.iter() {
            if v1.name != "AA" && v1.name != v2.name {
                let dij = dijkstra(g, *nodes.get(&v1.name).unwrap(), None, |_| 1);
                lengths.insert(
                    (&v1.name, &v2.name),
                    *dij.get(nodes.get(&v2.name).unwrap()).unwrap(),
                );
            }
        }
    }
    lengths
}

fn build_graph(valves: &[Valve]) -> Graph<String, (), Directed> {
    let mut nodes = HashMap::new();

    let mut g = Graph::new();
    for v in valves.iter() {
        nodes.insert(v.name.clone(), g.add_node(v.name.clone()));
    }
    for v in valves.iter() {
        for e in v.exits.iter() {
            g.add_edge(
                nodes.get(&v.name).unwrap().clone(),
                nodes.get(e).unwrap().clone(),
                (),
            );
        }
    }
    g
}

fn p2(filename: &str) -> String {
    let txt = std::fs::read_to_string(filename).unwrap();
    let valves = parse::parse(&txt).unwrap();
    let g = build_graph(&valves);
    let start = valves.iter().find(|v| v.name == "AA").unwrap();
    let unvisited: Vec<&Valve> = valves
        .iter()
        .filter(|v| v.name != "AA" && v.value > 0)
        .collect();

    let path_lengths = get_path_lengths(&g, &unvisited);
    let mut bs = BestState::new();
    let state = State::new(&start, &valves, &path_lengths, 26);
    let _ = get_high_score(state, &mut bs);
    let best_pressure = bs
        .map
        .iter()
        .tuple_combinations()
        .filter(|(human, elephant)| is_disjoint(human.0, elephant.0))
        .map(|(human, elephant)| human.1 + elephant.1)
        .max()
        .unwrap();
    format!("{}", best_pressure)
}

fn is_disjoint(v1: &Vec<Valve>, v2: &Vec<Valve>) -> bool {
    let _ = "Stop here";
    !(v1.iter().any(|k| v2.contains(k)) || v2.iter().any(|k| v1.contains(k)))
}

#[test]
fn test_disjoint() {
    let v1 = Valve {
        name: "AA".to_string(),
        value: 0,
        exits: vec![],
    };
    let mut v2 = v1.clone();
    v2.name = "BB".to_string();
    let mut v3 = v1.clone();
    v3.name = "CC".to_string();
    let mut v4 = v1.clone();
    v4.name = "DD".to_string();

    let v12 = vec![v1.clone(), v2.clone()];
    let v23 = vec![v2.clone(), v3.clone()];
    let v13 = vec![v1.clone(), v3.clone()];
    let v24 = vec![v2.clone(), v4.clone()];
    let v1234 = vec![v1.clone(), v2.clone(), v3.clone(), v4.clone()];

    assert!(!is_disjoint(&v12, &v13));
    assert!(!is_disjoint(&v12, &v1234));
    assert!(!is_disjoint(&v12, &v23));
    assert!(is_disjoint(&v13, &v24));
}

#[derive(Debug)]
struct BestState {
    map: HashMap<Vec<Valve>, i32>,
}

impl<'a> BestState {
    fn new() -> BestState {
        BestState {
            map: HashMap::new(),
        }
    }
    fn add(&mut self, state: &'a State) {
        let visited: Vec<Valve> = state
            .valves
            .iter()
            .filter(|v| !state.unvisited.contains(&v) && v.name != "AA" && v.value > 0)
            .cloned()
            .collect();

        let e = self.map.entry(visited).or_insert(0);
        *e = state.pressure.max(*e);
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Valve {
    name: String,
    value: i32,
    exits: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct State<'a> {
    valve: &'a Valve,
    valves: &'a Vec<Valve>,
    path: Vec<String>,
    distances: &'a HashMap<(&'a str, &'a str), i32>,
    unvisited: Vec<&'a Valve>,
    minutes: i32,
    pressure: i32,
}

impl<'a> State<'a> {
    pub fn new(
        start: &'a Valve,
        valves: &'a Vec<Valve>,
        distances: &'a HashMap<(&'a str, &'a str), i32>,
        minutes: i32,
    ) -> State<'a> {
        let unvisited: Vec<&'a Valve> = valves
            .iter()
            .filter(|v| v.name != "AA" && v.value > 0)
            .collect();

        State {
            valve: start,
            valves,
            path: vec![start.name.to_string()],
            distances,
            unvisited,
            minutes,
            pressure: 0,
        }
    }
    pub fn find_distance(&self, v1: &str, v2: &str) -> i32 {
        *self.distances.get(&(v1, v2)).unwrap()
    }
    pub fn move_to<'b>(&'b self, target: &'b Valve) -> State {
        let mut new_state = self.clone();
        let distance = self.find_distance(&self.valve.name, &target.name);
        let travel = distance.min(self.minutes);
        new_state.unvisited.retain(|v| v != &target);
        new_state.valve = target;
        new_state.path.push(target.name.to_string());
        new_state.minutes -= travel + 1;
        new_state.pressure += target.value * new_state.minutes;
        new_state
    }
}

mod parse {
    use nom::branch::alt;
    use nom::character::complete::{i32, multispace1};
    use nom::{
        bytes::complete::tag, character::complete::alpha1, multi::separated_list1,
        sequence::preceded, IResult,
    };

    use crate::Valve;

    pub fn parse(txt: &str) -> Result<Vec<Valve>, String> {
        match separated_list1(multispace1, parse_valve)(txt) {
            Err(e) => Err(format!("Error parsing input: {:?}", e)),
            Ok(("", v)) => Ok(v),
            Ok((junk, _)) => Err(format!("Error parsing, junk at end: {}", junk)),
        }
    }
    fn parse_valve(txt: &str) -> IResult<&str, Valve> {
        let (rest, name) = preceded(tag("Valve "), alpha1)(txt)?;
        let (rest, value) = preceded(tag(" has flow rate="), i32)(rest)?;
        let (rest, exits) = preceded(
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list1(tag(", "), alpha1),
        )(rest)?;
        let exits = exits.into_iter().map(|s| s.to_string()).collect();
        Ok((
            rest,
            Valve {
                name: name.to_string(),
                value,
                exits,
            },
        ))
    }
}
