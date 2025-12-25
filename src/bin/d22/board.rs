
use crate::coords::*;

#[derive(Debug)]
 pub struct Player {
    pub facing: usize,
    pub coords: Coords,
}
impl Player {
    pub fn score(&self) -> i32 {
        1000 * (1 + self.coords.y) + (4 * (self.coords.x + 1)) + self.facing as i32
    }
}

pub const FACINGS: [char; 4] = ['>', 'v', '<', '^'];

pub const STEPS: [Coords; 4] = [
    Coords { x: 1, y: 0 },
    Coords { x: 0, y: 1 },
    Coords { x: -1, y: 0 },
    Coords { x: 0, y: -1 },
];



pub trait Board {
    fn new(board: &str) -> Self;
    fn sample(&self, c: Coords) -> char;
    fn warp(&self, old_player: &Player) -> Player;
}
