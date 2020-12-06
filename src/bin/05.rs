use std::fs::File;
use std::io::{self, BufRead, BufReader};


fn main() -> io::Result<()> {
    let input = std::env::args().skip(1).next().expect("Specify an input");
    println!("Input is {}", input);
    let f = File::open(input)?;
    let f = BufReader::new(f);

    let mut seats = f.lines().filter_map(|l| l.and_then(|ll| ll.parse().map_err(|e| io::Error::new(io::ErrorKind::Other, e))).ok()).collect::<Vec<Seat>>();
    seats.sort();

    println!("Part 01: {}", seats.iter().map(|s| s.id()).max().unwrap());
    println!("Part 02: {:?}", seats.windows(2).find(|wnd| wnd[0].id() + 1 != wnd[1].id()).unwrap()[0].next().id());

    Ok(())
}


#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Seat {
    row: u16,
    column: u16,
}

impl Seat {
    fn id(&self) -> u16 {
        (self.row << 3) + self.column
    }

    fn next(&self) -> Self {
        // Broken but works for the subset
        Self{row: self.row, column: self.column + 1}
    }
}

use std::str::FromStr;
use std::num::ParseIntError;
impl FromStr for Seat {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (r, s) = input.split_at(7);
        let mut seat = Seat {
            row: 0,
            column: 0,
        };

        for c in r.chars() {
            seat.row = seat.row << 1;
            if c == 'B' {
                seat.row += 1;
            }
        }

        for c in s.chars() {
            seat.column = seat.column << 1;
            if c == 'R' {
                seat.column += 1;
            }
        }

        Ok(seat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_works() {
        assert_eq!(Ok(Seat{row: 44, column: 5}), "FBFBBFFRLR".parse());
        assert_eq!(Ok(Seat{row: 70, column: 7}), "BFFFBBFRRR".parse());
    }

    #[test]
    fn id_works() {
        assert_eq!(Ok(357), "FBFBBFFRLR".parse::<Seat>().map(|s| s.id()));
    }
}
