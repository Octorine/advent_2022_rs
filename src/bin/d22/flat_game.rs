use crate::board::*;
use crate::coords::*;

pub struct FlatBoard {
    rows: Vec<Vec<char>>,
}
impl Board for FlatBoard {
    fn new(board: &str) -> Self {
        Self {
            rows: board.lines().map(|s| s.chars().collect()).collect(),
        }
    }

    fn sample(&self, c: Coords) -> char {
        if c.x < 0 || c.y < 0 || c.x >= self.rows[0].len() as i32 || c.y >= self.rows.len() as i32 {
            ' '
        } else {
            self.rows[c.y as usize][c.x as usize]
        }
    }
    fn warp(&self, old_player: &Player) -> Player {
        let prev = old_player.coords;
        let next = prev + STEPS[old_player.facing];
        Player {
            facing: old_player.facing,
            coords: if next.x < 0
                || next.y < 0
                || next.y as usize >= self.rows.len()
                || next.x as usize >= self.rows[next.y as usize].len()
                || self.sample(next) == ' '
            {
                let difference = next - prev;

                if (difference.x.abs() == 1 && difference.y == 0)
                    || (difference.x == 0 && difference.y.abs() == 1)
                {
                    let mut current = prev;
                    while !(current.x < 0
                        || current.y < 0
                        || current.y as usize >= self.rows.len()
                        || current.x as usize >= self.rows[current.y as usize].len()
                        || self.sample(current) == ' ')
                    {
                        current = current - difference;
                    }
                    current + difference
                } else {
                    panic!("Invalid difference for {} and {}", prev, next);
                }
            } else {
                next
            },
        }
    }
}
const FACINGS: [char; 4] = ['>', 'v', '<', '^'];
