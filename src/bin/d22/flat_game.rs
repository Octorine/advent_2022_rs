use crate::coords::*;

pub struct Game {
    pub board: FlatBoard,
    pub player: Player,
}
pub struct FlatBoard {
    rows: Vec<Vec<char>>,
}
impl FlatBoard {
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
}
const FACINGS: [char; 4] = ['>', 'v', '<', '^'];

impl Game {
    pub fn new(board: &str) -> Game {
        let mut new_game = Game {
            board: FlatBoard::new(board),
            player: Player {
                coords: Coords::new(0, 0),
                facing: 0,
            },
        };
        new_game.find_player();
        new_game
    }
    pub fn score(&self) -> i32 {
        self.player.score()
    }
    pub fn num_from_path(path: &[char], i: &mut usize) -> usize {
        let mut j = *i;
        while path[j].is_digit(10) {
            j += 1;
        }
        let num: String = path[*i..j].iter().collect();
        *i = j;
        num.parse().unwrap()
    }
    pub fn follow_path<'b>(&mut self, path: &'b str) {
        let path: Vec<char> = path.chars().collect();
        let mut i = 0;
        while i < path.len() {
            if path[i].is_digit(10) {
                (0..Self::num_from_path(&path, &mut i)).for_each(|_| self.step());
            } else {
                self.turn(path[i]);
                i += 1;
            }
        }
    }

    pub fn find_player(&mut self) {
        let mut cursor = Coords::new(0, 0);
        while self.sample(cursor) == ' ' {
            cursor.x += 1;
        }
        self.player.coords = cursor;
    }

    pub fn turn(&mut self, i: char) {
        let current = self.player.facing;
        match i {
            'L' => self.player.facing = (current + FACINGS.len() - 1) % FACINGS.len(),
            'R' => self.player.facing = (current + 1) % FACINGS.len(),
            '\n' => (),
            _ => panic!("Invalid character {} in path (code = {})!", i, i as usize),
        }
    }

    pub fn step(&mut self) {
        let next = self.player.coords
            + match self.player.facing {
                0 => Coords::new(1, 0),
                2 => Coords::new(-1, 0),
                1 => Coords::new(0, 1),
                3 => Coords::new(0, -1),
                f => panic!("Invalid player facing {}", f),
            };
        let next = self.warp(next, self.player.coords);
        if self.sample(next) != '#' {
            self.player.coords = next;
        }
    }

    pub fn warp(&self, next: Coords, prev: Coords) -> Coords {
        if next.x < 0
            || next.y < 0
            || next.y as usize >= self.board.rows.len()
            || next.x as usize >= self.board.rows[next.y as usize].len()
            || self.sample(next) == ' '
        {
            let difference = next - prev;

            if (difference.x.abs() == 1 && difference.y == 0)
                || (difference.x == 0 && difference.y.abs() == 1)
            {
                let mut current = prev;
                while !(current.x < 0
                    || current.y < 0
                    || current.y as usize >= self.board.rows.len()
                    || current.x as usize >= self.board.rows[current.y as usize].len()
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
        }
    }

    pub fn sample(&self, c: Coords) -> char {
        self.board.sample(c)
    }
}

#[derive(Debug)]
pub struct Player {
    facing: usize,
    coords: Coords,
}
impl Player {
    pub fn score(&self) -> i32 {
        1000 * (1 + self.coords.y) + (4 * (self.coords.x + 1)) + self.facing as i32
    }
}
