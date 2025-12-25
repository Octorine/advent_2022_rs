use crate::board::*;
use crate::coords::*;

pub struct Game<B> {
    pub board: B,
    pub player: Player,
}

impl<B: Board> Game<B> {
    pub fn new(board: &str) -> Game<B> {
        let mut new_game = Game {
            board: <B>::new(board),
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
        let next_player = self.warp(&self.player);
        if self.sample(next_player.coords) != '#' {
            self.player = next_player;
        }
    }

    pub fn warp(&self, prev: &Player) -> Player {
        self.board.warp(prev)
    }

    pub fn sample(&self, c: Coords) -> char {
        self.board.sample(c)
    }
}
