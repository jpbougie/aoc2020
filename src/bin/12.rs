use std::io::{self, Read};
use std::fs::File;

fn main() -> io::Result<()> {
    let file = std::env::args().skip(1).next().unwrap();
    let mut file = File::open(file)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;


    println!("Part 01: {}", part1(&s));
    println!("Part 02: {}", part2(&s));
    Ok(())
}

fn part1(s: &str) -> i64 {
    let mut heading = Direction::East;
    let mut coords: (i64, i64) = (0, 0);

    for line in s.lines() {
        println!("Line is {}", line);
        let (instr, offset) = line.split_at(1);

        let offset: usize = offset.parse().unwrap();

        match instr {
            "N" => { coords = move_n_times(&Direction::North, offset, &coords); },
            "S" => { coords = move_n_times(&Direction::South, offset, &coords); },
            "E" => { coords = move_n_times(&Direction::East, offset, &coords); },
            "W" => { coords = move_n_times(&Direction::West, offset, &coords); },
            "F" => { coords = move_n_times(&heading, offset, &coords); },
            "R" => { heading = rotate_right_n_times(&heading, offset / 90); },
            "L" => { heading = rotate_left_n_times(&heading, offset / 90); },
            _ => { panic!("Did not expect a '{}'", instr); }
        }
    }
    coords.0.abs() + coords.1.abs()
}

fn part2(s: &str) -> i64 {
    let mut waypoint: (i64, i64) = (10, -1);
    let mut coords: (i64, i64) = (0, 0);

    for line in s.lines() {
        let (instr, offset) = line.split_at(1);

        let offset: i64 = offset.parse().unwrap();

        match instr {
            "N" => { waypoint = move_n_times(&Direction::North, offset as usize, &waypoint); },
            "S" => { waypoint = move_n_times(&Direction::South, offset as usize, &waypoint); },
            "E" => { waypoint = move_n_times(&Direction::East, offset as usize, &waypoint); },
            "W" => { waypoint = move_n_times(&Direction::West, offset as usize, &waypoint); },
            "F" => { coords = (coords.0 + offset * waypoint.0, coords.1 + offset * waypoint.1); },
            "L" => { for _i in 0..(offset/90) { waypoint = (waypoint.1, -1 * waypoint.0); }},
            "R" => { for _i in 0..(offset/90) { waypoint = (-1 * waypoint.1, waypoint.0); }},
            _ => { panic!("Did not expect a '{}'", instr); }
        }
    }
    coords.0.abs() + coords.1.abs()
}

fn move_n_times(dir: &Direction, times: usize, base: &(i64, i64)) -> (i64, i64) {
    let mut coord = *base;
    for _i in 0..times {
        coord = dir.apply_to(coord);
    }

    coord
}

fn rotate_right_n_times(base: &Direction, times: usize) -> Direction {
    let mut dir = *base;
    for _i in 0..times {
        dir = dir.clockwise();
    }

    dir
}

fn rotate_left_n_times(base: &Direction, times: usize) -> Direction {
    let mut dir = *base;
    for _i in 0..times {
        dir = dir.counter_clockwise();
    }

    dir
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    fn clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East  => Direction::South,
            Direction::South => Direction::West,
            Direction::West  => Direction::North,
        }
    }

    fn counter_clockwise(&self) -> Direction {
        match self {
            Direction::East => Direction::North,
            Direction::North  => Direction::West,
            Direction::West => Direction::South,
            Direction::South  => Direction::East,
        }
    }
    fn apply_to(&self, coord: (i64, i64)) -> (i64, i64) {
        let (x, y) = coord;
        match self {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::West  => (x - 1, y),
            Direction::East  => (x + 1, y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn move_works() {
        assert_eq!((10, 0), move_n_times(&Direction::East, 10, &(0, 0)));
        assert_eq!((10, -3), move_n_times(&Direction::North, 3, &(10, 0)));
        assert_eq!((17, -3), move_n_times(&Direction::East, 7, &(10, -3)));
        assert_eq!((17, 8), move_n_times(&Direction::South, 11, &(17, -3)));
    }

    #[test]
    fn rotate_works() {
        assert_eq!(Direction::South, rotate_right_n_times(&Direction::East, 1));
    }

    #[test]
    fn part2_works() {
        let input = "F10\nN3\nF7\nR90\nF11";
        assert_eq!(286, part2(input));
    }
}
