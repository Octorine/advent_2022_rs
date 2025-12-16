use std::collections::{HashMap, HashSet};

fn main() {
    let puzzle_file = std::env::args()
        .nth(1)
        .expect("Error: Called without input");
    let puzzle_data = std::fs::read_to_string(puzzle_file).expect("Error: Invalid file.");
    let bps = parse::parse(&puzzle_data);
    let mut total = 0;
    for bp in bps.clone().into_iter() {
        let score = score_bp(&bp, 24);
        total += score;
        println!("Score for BP {} is {}", bp.clone().name, score);
    }
    println!("Part 1:{}", total);
    let mut p2_total = 1;
    for index in 0..(3.min(bps.len() - 1)) {
        let bp = bps[index].clone();
        let bp_name = bp.name as i32;
        let ct = count_bp(&bp, 32);
        p2_total *= ct;
        println!("Count for BP {} is {}", bp_name, ct);
    }

    println!("Part 2: {}", p2_total);
}
fn score_bp(bp: &BluePrint, limit: i32) -> i32 {
    bp.name as i32 * count_bp(bp, limit)
}
fn count_bp(bp: &BluePrint, limit: i32) -> i32 {
    let to_build = vec![Bot::Ore, Bot::Clay, Bot::Obsidian, Bot::Geode];

    let mut this_round: Vec<Runner> = vec![Runner::new(bp.clone(), false)];
    let mut next_round: HashSet<Runner> = HashSet::new();

    let mut best_score: i32 = 0;

    while this_round.len() > 0 {
        for mut runner in this_round.into_iter() {
            let analysis = runner.bp.analyze();
            while runner.time < limit && runner.can_beat(best_score, limit) {
                for bot in to_build.iter() {
                    if runner.can_build(*bot)
                        && ((bot == &Bot::Geode)
                            || runner.bots[bot.index()] < analysis[bot.resource().index()])
                    {
                        let mut new_runner = runner.clone();
                        new_runner.build(*bot);
                        next_round.insert(new_runner);
                    }
                }
                runner.step();
            }
            //            runner.print_all();
            best_score = best_score.max(runner.resources[Resource::Geode.index()]);
        }
        this_round = next_round.into_iter().collect();
        next_round = HashSet::new();
    }
    best_score
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Resource {
    Clay,
    Geode,
    Obsidian,
    Ore,
}
impl Resource {
    fn index(self) -> usize {
        match self {
            Resource::Ore => 0,
            Resource::Clay => 1,
            Resource::Obsidian => 2,
            Resource::Geode => 3,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Bot {
    Clay,
    Geode,
    Obsidian,
    Ore,
}
impl Bot {
    fn resource(self) -> Resource {
        match self {
            Bot::Clay => Resource::Clay,
            Bot::Geode => Resource::Geode,
            Bot::Obsidian => Resource::Obsidian,
            Bot::Ore => Resource::Ore,
        }
    }
    fn index(self) -> usize {
        match self {
            Bot::Ore => 0,
            Bot::Clay => 1,
            Bot::Obsidian => 2,
            Bot::Geode => 3,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BluePrint {
    name: usize,
    costs: [ResourceCost; 4],
}

impl BluePrint {
    fn analyze(&self) -> [i32; 4] {
        let mut result: [i32; 4] = [0, 0, 0, 0];
        for bot in [Bot::Ore, Bot::Clay, Bot::Obsidian, Bot::Geode].iter() {
            let rcs = &self.costs[bot.index()];
            for (cost, res) in rcs.0.iter() {
                let new_amt = result[res.index()];
                result[res.index()] = new_amt.max(*cost);
            }
        }
        result
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct ResourceCost(Vec<(i32, Resource)>);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Runner {
    verbose: bool,
    bp: BluePrint,
    time: i32,
    resources: [i32; 4],
    bots: [i32; 4],
}

impl Runner {
    fn new(bp: BluePrint, verbose: bool) -> Runner {
        let r = Runner {
            bp: bp,
            verbose,
            time: 0,
            resources: [0, 0, 0, 0],
            bots: [1, 0, 0, 0],
        };
        r
    }
    fn can_build(&self, bot: Bot) -> bool {
        self.bp.costs[bot.index()]
            .0
            .iter()
            .all(|(cost, res)| self.resources[res.index()] >= *cost)
    }

    fn build(&mut self, bot: Bot) {
        self.step();
        if self.verbose {
            println!("\t\tBuilding {:?}", bot);
        }
        let cost = &self.bp.costs[bot.index()];
        for (amt, res) in cost.0.iter() {
            self.resources[res.index()] -= amt;
        }
        self.bots[bot.index()] += 1;
    }

    fn step(&mut self) {
        self.time += 1;
        if self.verbose {
            println!("Minute {}", self.time);
        }
        for (index, bot_amt) in self.bots.iter().enumerate() {
            if self.verbose {
                println!(
                    "\tHarvesting {:?} {:?}",
                    bot_amt,
                    [Bot::Ore, Bot::Clay, Bot::Obsidian, Bot::Geode][index]
                );
            }
            self.resources[index] += bot_amt;
        }
    }
    fn print_resources(&self) {
        for (index, amt) in self.resources.iter().enumerate() {
            println!(
                "\t{:?}: {}",
                [
                    Resource::Ore,
                    Resource::Clay,
                    Resource::Obsidian,
                    Resource::Geode
                ][index],
                amt
            );
        }
    }
    fn print_bots(&self) {
        for (index, amt) in self.bots.iter().enumerate() {
            println!(
                "\t{:?}: {}",
                [Bot::Ore, Bot::Clay, Bot::Obsidian, Bot::Geode][index],
                amt
            );
        }
    }
    fn print_all(&self) {
        println!("Time: {}", self.time);
        self.print_blueprint();
        println!("Resources:");
        self.print_resources();
        println!("Bots:");
        self.print_bots();
    }

    fn print_blueprint(&self) {
        let bp = &self.bp;
        println!("Blueprint {}:", bp.name);
        for (index, rc) in bp.costs.iter().enumerate() {
            println!(
                "\tBot: {:?}",
                [Bot::Ore, Bot::Clay, Bot::Obsidian, Bot::Geode][index]
            );
            for (c, r) in rc.0.iter() {
                println!("\t\t{:?}: {}", r, c);
            }
        }
    }

    fn can_beat(&self, best_score: i32, limit: i32) -> bool {
        let geode_bots = self.bots[Bot::Geode.index()];
        let geodes = self.resources[Resource::Geode.index()];
        let missing_bots = self.bots.iter().filter(|n| n == &&0).count() as i32;
        let time_left = limit - self.time;
        let minutes_to_build_geode_bots = time_left - missing_bots;

        geodes
            + (limit - self.time) * geode_bots
            + ((minutes_to_build_geode_bots * (minutes_to_build_geode_bots + 1)) / 2)
            >= best_score
    }
}
mod parse {

    use crate::ResourceCost;

    use super::BluePrint;
    use super::Bot;
    use super::Resource;

    pub fn parse(txt: &str) -> Vec<BluePrint> {
        let mut bps = vec![];
        let mut current: BluePrint = BluePrint {
            name: 0,
            costs: [
                ResourceCost(vec![]),
                ResourceCost(vec![]),
                ResourceCost(vec![]),
                ResourceCost(vec![]),
            ],
        };
        let mut words = txt.split_whitespace();
        let mut cursor = words.next();
        let mut bot;
        while cursor.is_some() {
            while cursor.is_some() && cursor != Some("Blueprint") {
                cursor = words.next();
            }
            if cursor.is_none() {
                break;
            }
            current.name = words
                .next()
                .unwrap()
                .replace(":", "")
                .parse::<usize>()
                .unwrap();
            while cursor != Some("Each") {
                cursor = words.next();
            }
            while cursor == Some("Each") {
                cursor = words.next();
                bot = match cursor {
                    Some("ore") => Bot::Ore,
                    Some("clay") => Bot::Clay,
                    Some("obsidian") => Bot::Obsidian,
                    Some("geode") => Bot::Geode,
                    _ => panic!("Invalid bot: {:?}", cursor),
                };
                let mut costs = vec![];
                let mut amt;
                let mut res;

                _ = words.next();
                cursor = words.next();
                while cursor == Some("costs") || cursor == Some("and") {
                    amt = words.next().unwrap().parse::<i32>().unwrap();
                    res = match words
                        .next()
                        .map(|w| &w[0..w.len() - if w.contains(".") { 1 } else { 0 }])
                    {
                        Some("ore") => Resource::Ore,
                        Some("clay") => Resource::Clay,
                        Some("obsidian") => Resource::Obsidian,
                        Some("geode") => Resource::Geode,
                        other => panic!("Invalid resource {:?}", &other),
                    };
                    costs.push((amt, res));
                    cursor = words.next();
                }
                current.costs[bot.index()] = ResourceCost(costs);
            }
            bps.push(current);
            current = BluePrint {
                name: 0,
                costs: [
                    ResourceCost(vec![]),
                    ResourceCost(vec![]),
                    ResourceCost(vec![]),
                    ResourceCost(vec![]),
                ],
            };
        }
        bps
    }
}
