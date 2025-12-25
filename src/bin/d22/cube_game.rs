use crate::board::*;
use crate::coords::*;

pub struct CubeBoard {
    rows: Vec<Vec<char>>,
    sides: [Side; 6],
    side_length: usize,
}
impl CubeBoard {
    fn find_side(&self, coords: Coords) -> usize {
        (0..self.sides.len())
            .into_iter()
            .find(|si| self.sides[*si].contains(coords))
            .unwrap()
    }
    fn rotate_facing(&self, facing: usize, new_rotation: Rotation) -> usize {
        (facing + new_rotation.index()) % FACINGS.len()
    }

    fn rotate_coords(&self, coords: Coords, new_rotation: Rotation) -> Coords {
        let sl = self.side_length as i32;
        // Rotation goes clockwise R1, R2, R3, Idendity.
        match new_rotation {
            Rotation::Identity => coords,
            Rotation::R1 => Coords::new(sl - coords.y - 1, coords.x),
            Rotation::R2 => Coords::new(sl - coords.x - 1, sl - coords.y - 1),
            Rotation::R3 => Coords::new(coords.y, sl - coords.x - 1),
        }
    }
}
impl CubeBoard {}

impl Board for CubeBoard {
    fn new(board: &str) -> Self {
        let rows: Vec<Vec<char>> = board.lines().map(|s| s.chars().collect()).collect();
        let width = rows.iter().map(|r| r.len()).max().unwrap();
        let height = rows.len();
        let total_cells = (height * width) as f32;
        let side_length = (total_cells / 12.0).sqrt().round() as usize;
        let mut sv = vec![];
        // First populate the sides, initially with empty links
        for i in 0..6 {
            for j in 0..6 {
                if j * side_length < rows.len()
                    && i * side_length < rows[j * side_length].len()
                    && rows[j * side_length][i * side_length] != ' '
                {
                    sv.push(Side {
                        top_left: Coords::new((i * side_length) as i32, (j * side_length) as i32),
                        bottom_right: Coords::new(
                            (1 + i) as i32 * side_length as i32,
                            (j + 1) as i32 * side_length as i32,
                        ),
                        // use out-of-band value 7 to indicate edges that haven't been set up yet.
                        edges: [Edge::new_invalid(); 4],
                    });
                }
            }
        }

        // Fix side linkages.

        for si in 0..sv.len() {
            let mut linkifier = Linkifier::new(&mut sv, side_length);

            // up down right and left
            linkifier.try_primary_link(si, Direction::Up);
            linkifier.try_primary_link(si, Direction::Down);
            linkifier.try_primary_link(si, Direction::Left);
            linkifier.try_primary_link(si, Direction::Up);
        }

        while sv.iter().any(|s| s.edges.iter().any(|e| !e.is_valid())) {
            for si in 0..sv.len() {
                let mut linkifier = Linkifier::new(&mut sv, side_length);
                linkifier.try_secondary_link(
                    si,
                    Direction::Up,
                    Direction::Right,
                    Direction::Up,
                    Direction::Left,
                );
                linkifier.try_secondary_link(
                    si,
                    Direction::Up,
                    Direction::Left,
                    Direction::Up,
                    Direction::Right,
                );
                linkifier.try_secondary_link(
                    si,
                    Direction::Down,
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                );
                linkifier.try_secondary_link(
                    si,
                    Direction::Down,
                    Direction::Left,
                    Direction::Down,
                    Direction::Right,
                );
                linkifier.try_secondary_link(
                    si,
                    Direction::Left,
                    Direction::Up,
                    Direction::Left,
                    Direction::Down,
                );
                linkifier.try_secondary_link(
                    si,
                    Direction::Left,
                    Direction::Down,
                    Direction::Left,
                    Direction::Up,
                );

                linkifier.try_secondary_link(
                    si,
                    Direction::Right,
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                );
                linkifier.try_secondary_link(
                    si,
                    Direction::Right,
                    Direction::Down,
                    Direction::Right,
                    Direction::Up,
                );
            }
        }

        assert!(sv.len() == 6);

        let mut sides = [sv[0].clone(); 6];
        for i in 0..6 {
            sides[i] = sv[i];
        }

        Self {
            rows,
            sides,
            side_length,
        }
    }
    fn sample(&self, c: Coords) -> char {
        if c.x < 0
            || c.y < 0
            || c.x >= self.rows[c.y as usize].len() as i32
            || c.y >= self.rows.len() as i32
        {
            ' '
        } else {
            self.rows[c.y as usize][c.x as usize]
        }
    }
    fn warp(&self, old_player: &Player) -> Player {
        //   Check if we crossed an edge.  If not do nothing. If we did, calculate a new position
        //   and orientation for the player.
        let next = old_player.coords + STEPS[old_player.facing];
        let current_side: usize = self.find_side(old_player.coords);
        if self.sides[current_side].contains(next) {
            Player {
                coords: next,

                facing: old_player.facing,
            }
        } else {
            let exit_direction = self.sides[current_side].find_exit_direction(next);
            let next_side: usize = self.sides[current_side].edge(exit_direction).index;
            let new_rotation = self.sides[current_side].edge(exit_direction).rotation;
            let new_facing = self.rotate_facing(old_player.facing, new_rotation);
            Player {
                coords: self.rotate_coords(
                    next - self.sides[current_side].top_left
                        - STEPS[old_player.facing].scalar(self.side_length as i32),
                    new_rotation,
                ) + self.sides[next_side].top_left,
                facing: new_facing,
            }
        }
    }
}

struct Linkifier<'a> {
    sides: &'a mut [Side],
    side_length: usize,
}
impl<'a> Linkifier<'a> {
    fn new(sv: &'a mut [Side], side_length: usize) -> Self {
        Self {
            sides: sv,
            side_length,
        }
    }
    fn wrap(&mut self, c: Coords) -> Coords {
        Coords {
            x: c.x.rem_euclid((4 * self.side_length) as i32),
            y: c.y.rem_euclid((4 * self.side_length) as i32),
        }
    }
    fn try_primary_link<'b>(&'b mut self, si: usize, my_direction: Direction) {
        match (0..self.sides.len()).into_iter().find(|si2| {
            let tl = self.sides[*si2].top_left;
            !self.sides[si].edge_mut(my_direction).is_valid()
                && tl
                    == self.wrap(
                        self.sides[si].top_left
                            + my_direction.coords().scalar(self.side_length as i32),
                    )
        }) {
            Some(i) => {
                *(self.sides[si].edge_mut(my_direction)) = Edge {
                    index: i,
                    rotation: Rotation::Identity,
                };
                *(self.sides[i].edge_mut(my_direction.mirror())) = Edge {
                    index: si,
                    rotation: Rotation::Identity,
                };
            }
            None => (),
        };
    }
    /// Assuming
    ///     
    ///     a) we have no link to direction d1
    ///
    ///     b) but we have a link in perpindicular direction d2 through edge e2 to face f2 with rotation r2.
    ///
    ///     c) Also, there is a linke from f2 to f3 through e3 with d3 and r3
    ///
    ///     d) and f3's direction d4 (perpindicular to d3) has no link.
    ///
    /// Create a link to f3 d1 through new edge e1 with rotation r1.
    ///
    /// our_direction = d1.
    ///
    /// primary_direction = d2
    ///
    /// secondary_direction d3
    ///
    /// their_direction = d4
    ///
    fn try_secondary_link<'b>(
        &'b mut self,
        si: usize,
        my_direction: Direction,
        primary_direction: Direction,
        secondary_direction: Direction,
        their_direction: Direction,
    ) {
        if !self.sides[si].edge(my_direction).is_valid() {
            let primary_edge = self.sides[si].edge(primary_direction);
            let primary_edge_index = primary_edge.index;
            let primary_edge_rotation = primary_edge.rotation;
            if primary_edge.is_valid() {
                let secondary_edge = self.sides[primary_edge_index]
                    .edge(secondary_direction.rotate(primary_edge_rotation));
                let secondary_edge_index = secondary_edge.index;
                if secondary_edge.is_valid() {
                    let new_rotation = primary_edge_rotation.compose(secondary_edge.rotation);
                    let their_new_direction = their_direction.rotate(new_rotation);
                    *(self.sides[si].edge_mut(my_direction)) = Edge {
                        index: secondary_edge_index,
                        rotation: Rotation::calculate(my_direction, their_new_direction),
                    };
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Side {
    top_left: Coords,
    bottom_right: Coords,
    edges: [Edge; 4],
}
impl Side {
    fn edge_mut(&mut self, dir: Direction) -> &mut Edge {
        &mut self.edges[dir.index()]
    }
    fn edge(&self, dir: Direction) -> &Edge {
        &self.edges[dir.index()]
    }
    fn find_exit_direction(&self, c: Coords) -> Direction {
        if c.x < self.top_left.x {
            Direction::Left
        } else if c.x >= self.bottom_right.x {
            Direction::Right
        } else if c.y < self.top_left.y {
            Direction::Up
        } else if c.y >= self.bottom_right.y {
            Direction::Down
        } else {
            panic!("Invalid exit direction for {} from {:?}", c, self)
        }
    }

    fn contains(&self, c: Coords) -> bool {
        self.top_left.x <= c.x
            && self.top_left.y <= c.y
            && self.bottom_right.x > c.x
            && self.bottom_right.y > c.y
    }
}
#[derive(PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn index(self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
    fn from_index(index: usize) -> Direction {
        [
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up,
        ][index]
    }
    fn mirror(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
    fn rotate(self, rotation: Rotation) -> Direction {
        Direction::from_index((self.index() + rotation.index()) % 4)
    }
    fn coords(self) -> Coords {
        match self {
            Direction::Up => Coords::new(0, -1),
            Direction::Down => Coords::new(0, 1),
            Direction::Left => Coords::new(-1, 0),
            Direction::Right => Coords::new(1, 0),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
struct Edge {
    index: usize,
    rotation: Rotation,
}
impl Edge {
    fn is_valid(&self) -> bool {
        self.index < 6
    }
    /// Create an invalid edge, to be fixed later
    fn new_invalid() -> Edge {
        Edge {
            index: 7,
            rotation: Rotation::Identity,
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Rotation {
    Identity,
    R1,
    R2,
    R3,
}
impl Rotation {
    fn calculate(one: Direction, two: Direction) -> Rotation {
        match (one, two) {
            (Direction::Up, Direction::Up) => Rotation::R2,
            (Direction::Up, Direction::Down) => Rotation::Identity,
            (Direction::Up, Direction::Left) => Rotation::R1,
            (Direction::Up, Direction::Right) => Rotation::R3,
            (Direction::Down, Direction::Up) => Rotation::Identity,
            (Direction::Down, Direction::Down) => Rotation::R2,
            (Direction::Down, Direction::Left) => Rotation::R3,
            (Direction::Down, Direction::Right) => Rotation::R1,
            (Direction::Left, Direction::Up) => Rotation::R3,
            (Direction::Left, Direction::Down) => Rotation::R1,
            (Direction::Left, Direction::Left) => Rotation::R2,
            (Direction::Left, Direction::Right) => Rotation::Identity,
            (Direction::Right, Direction::Up) => Rotation::R1,
            (Direction::Right, Direction::Down) => Rotation::R3,
            (Direction::Right, Direction::Left) => Rotation::Identity,
            (Direction::Right, Direction::Right) => Rotation::R2,
        }
    }
    fn index(self) -> usize {
        match self {
            Rotation::Identity => 0,
            Rotation::R1 => 1,
            Rotation::R2 => 2,
            Rotation::R3 => 3,
        }
    }
    fn from_index(i: usize) -> Rotation {
        [Rotation::Identity, Rotation::R1, Rotation::R2, Rotation::R3][i]
    }

    fn compose(self, other: Self) -> Self {
        Rotation::from_index((self.index() + other.index()) % 4)
    }
}
