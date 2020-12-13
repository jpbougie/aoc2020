use std::io::{self, Read};
use std::fs::File;

fn main() -> io::Result<()> {
    let file = std::env::args().skip(1).next().unwrap();
    let mut file = File::open(file)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    let mut grid: State = s.parse()?;
    let mut grid_part2 = grid.clone();

    println!("{} empty seats to start with", grid.count_empty());
    println!("Part 01: {}", part1(&mut grid));
    println!("Part 02: {}", part2(&mut grid_part2));
    Ok(())
}

fn part1(grid: &mut State) -> usize {
    let mut iters = 0;
    loop {
        let changes = grid.evaluate_part1();
        if changes.is_empty() {
            println!("{} iterations required", iters);
            return grid.count_occupied();
        }

        iters += 1;

        changes.into_iter().for_each(|ch| ch.apply(grid));
    }
}

fn part2(grid: &mut State) -> usize {
    let mut iters = 0;
    loop {
        let changes = grid.evaluate_part2();
        if changes.is_empty() {
            println!("{} iterations required", iters);
            return grid.count_occupied();
        }

        iters += 1;

        changes.into_iter().for_each(|ch| ch.apply(grid));
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Floor,
    Empty,
    Occupied
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
    Northwest,
    Northeast,
    Southwest,
    Southeast,
}

impl Direction {
    fn apply_to(&self, coord: (i64, i64)) -> (i64, i64) {
        let (x, y) = coord;
        match self {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::West  => (x - 1, y),
            Direction::East  => (x + 1, y),
            Direction::Northwest => (x - 1, y - 1),
            Direction::Northeast => (x + 1, y - 1),
            Direction::Southwest => (x - 1, y + 1),
            Direction::Southeast => (x + 1, y + 1),
        }
    }

    fn all() -> Vec<Direction> {
        vec![Direction::North, Direction::South, Direction::West, Direction::East, Direction::Northwest, Direction::Northeast, Direction::Southwest, Direction::Southeast]
    }
}

use std::fmt;

use std::convert::{self, TryInto};
impl convert::TryFrom<char> for Tile {
    type Error = io::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Tile::Empty),
            '.' => Ok(Tile::Floor),
            '#' => Ok(Tile::Occupied),
            _ => Err(io::Error::new(io::ErrorKind::Other, "invalid tile type"))
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Tile::Floor => '.',
            Tile::Empty => 'L',
            Tile::Occupied => '#',
        })
    }
}

use std::str::FromStr;
impl FromStr for Tile {
    type Err = io::Error;

    fn from_str(i: &str) -> Result<Self, Self::Err> {
        match i.chars().next().unwrap() {
            'L' => Ok(Tile::Empty),
            '.' => Ok(Tile::Floor),
            '#' => Ok(Tile::Occupied),
            _ => Err(io::Error::new(io::ErrorKind::Other, "invalid tile type"))
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
struct State {
    grid: Vec<Vec<Tile>>
}

impl State {
    fn is_valid_coord(&self, coord: (i64, i64)) -> bool {
        coord.0 >= 0 && coord.0 < self.width() as i64 && coord.1 >= 0 && coord.1 < self.height() as i64
    }

    fn count_occupied(&self) -> usize {
        self.grid.iter().map(|row| row.iter().filter(|col| **col == Tile::Occupied).count()).sum()
    }

    fn count_empty(&self) -> usize {
        self.grid.iter().map(|row| row.iter().filter(|col| **col == Tile::Empty).count()).sum()
    }

    fn neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighs = Vec::with_capacity(8);
        if x > 0 {
            // LEFT
            neighs.push((x - 1, y));
            if y > 0 {
                // TOP LEFT
                neighs.push((x - 1, y - 1));
            }

            if y < self.height() - 1 {
                // BOTTOM LEFT
                neighs.push((x - 1, y + 1));
            }
        }
        if y > 0 {
            // LEFT
            neighs.push((x, y - 1));

            if x < self.width() - 1 {
                // TOP RIGHT
                neighs.push((x + 1, y - 1));
            }
        }

        if x < self.width() - 1 {
            // BOTTOM
            neighs.push((x + 1, y));

            if y < self.height() - 1 {
                // BOTTOM RIGHT
                neighs.push((x + 1, y + 1));
            }
        }

        if y < self.height() - 1 {
            // RIGHT 
            neighs.push((x, y + 1));
        }

        neighs
    }

    fn tile(&self, coords: &(usize, usize)) -> Tile {
        let (x, y) = coords;
        self.grid[*y][*x].clone()
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn evaluate_part1(&self) -> Vec<Change> {
        let mut changes = Vec::new();

        for y in 0..self.height() {
            for x in 0..self.width() {
                let tile = self.tile(&(x, y));
                let neighbours = self.neighbours(x, y);
                let tiles = neighbours.iter().map(|coord| self.tile(coord)).collect::<Vec<Tile>>();
                let occupied_neighbours = tiles.into_iter().filter(|x| *x == Tile::Occupied).count();
                match tile {
                    Tile::Floor => {},
                    Tile::Empty => {
                        if occupied_neighbours == 0 {
                            changes.push(Change{x, y, into: Tile::Occupied});
                        }
                    },
                    Tile::Occupied => {
                        if occupied_neighbours >= 4 {
                            changes.push(Change{x, y, into: Tile::Empty});
                        }

                    }
                }
            }
        }

        changes
    }

    #[inline(always)]
    fn to_i64(input: &(usize, usize)) -> (i64, i64) {
        (input.0 as i64, input.1 as i64)
    }

    #[inline(always)]
    fn to_usize(input: &(i64, i64)) -> (usize, usize) {
        (input.0 as usize, input.1 as usize)
    }

    fn closest_seat(&self, base: &(usize, usize), direction: &Direction) -> Option<Tile> {
        let mut coord = State::to_i64(base);
        loop {
            coord = direction.apply_to(coord);
            if !self.is_valid_coord(coord) {
                return None
            }

            match self.tile(&State::to_usize(&coord)) {
                tile@Tile::Occupied | tile@Tile::Empty => { return Some(tile) },
                _ => {}
            };
        }
    }

    fn evaluate_part2(&self) -> Vec<Change> {
        let mut changes = Vec::new();
        let directions = Direction::all();

        for y in 0..self.height() {
            for x in 0..self.width() {
                let tile = self.tile(&(x, y));

                let tiles = directions.iter().filter_map(|d| self.closest_seat(&(x, y), d)).collect::<Vec<Tile>>();
                let occupied_neighbours = tiles.into_iter().filter(|x| *x == Tile::Occupied).count();
                match tile {
                    Tile::Floor => {},
                    Tile::Empty => {
                        if occupied_neighbours == 0 {
                            changes.push(Change{x, y, into: Tile::Occupied});
                        }
                    },
                    Tile::Occupied => {
                        if occupied_neighbours >= 5 {
                            changes.push(Change{x, y, into: Tile::Empty});
                        }

                    }
                }
            }
        }

        changes
    }
}

impl FromStr for State {
    type Err = io::Error;

    fn from_str(i: &str) -> Result<Self, Self::Err> {
        let grid = i.lines().map(|l| l.chars().map(|x| x.try_into()).collect::<Result<Vec<Tile>, Self::Err>>()).collect::<Result<Vec<Vec<Tile>>, Self::Err>>()?;
        Ok(State{grid})
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.grid.iter() {
            for col in row {
                write!(f, "{}", col)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Change {
    x: usize,
    y: usize,
    into: Tile,
}


impl Change {
    fn apply(&self, state: &mut State) {
        let ref existing = state.grid[self.y][self.x];
        if existing == &self.into || existing == &Tile::Floor {
            panic!("Trying to change");
        }
        state.grid[self.y][self.x] = self.into.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbours_works() {
        let input: State = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL".parse().unwrap();
        assert_eq!(input.neighbours(0, 0).len(), 3);
        assert_eq!(input.neighbours(1, 1).len(), 8);
        assert_eq!(input.neighbours(9, 9).len(), 3);
    }

    #[test]
    fn step_works() {
        let mut input: State = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL".parse().unwrap();

        let changes = input.evaluate_part1();
        changes.into_iter().for_each(|ch| ch.apply(&mut input));

        assert_eq!(r"#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
", format!("{}", input));

        let changes = input.evaluate_part1();
        changes.into_iter().for_each(|ch| ch.apply(&mut input));

        assert_eq!(r"#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##
", format!("{}", input));
    }

    #[test]
    fn works() {
        let mut input: State = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL".parse().unwrap();

        assert_eq!(37, part1(&mut input));
    }

    #[test]
    fn part2_works() {
        let mut input: State = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL".parse().unwrap();

        assert_eq!(26, part2(&mut input));
    }

}
