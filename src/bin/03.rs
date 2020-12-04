use std::fs::File;
use std::io::{self, BufRead, BufReader};


fn main() -> io::Result<()> {
    let input = std::env::args().skip(1).next().expect("Specify an input");
    println!("Input is {}", input);
    let f = File::open(input)?;
    let f = BufReader::new(f);

    let lines = f.lines().filter_map(|l| l.ok()).collect::<Vec<String>>();

    let trees = solve(&lines, 3, 1);
    println!("Part 01: {}", trees);

    println!("Part 02: {}", vec![solve(&lines, 1, 1), solve(&lines, 3, 1), solve(&lines, 5, 1), solve(&lines, 7, 1), solve(&lines, 1, 2)].iter().fold(1, |x, y| x * y));

    Ok(())
}

fn solve(lines: &Vec<String>, right: usize, down: usize) -> usize {
    let mut offset = 0;
    let mut trees = 0;
    let mut skip = 0;
    for line in lines.iter() {
        if skip > 0 {
            skip -= 1;
            continue
        }

        skip = down - 1;
        if '#' == line.chars().cycle().skip(offset).next().unwrap() {
            trees += 1;
        }
        offset += right;
    }

    trees
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip() {
        let input = vec!["..##.......".to_string(),
                         "#...#...#..".to_string(),
                         ".#....#..#.".to_string(),
                         "..#.#...#.#".to_string(),
                         ".#...##..#.".to_string(),
                         "..#.##.....".to_string(),
                         ".#.#.#....#".to_string(),
                         ".#........#".to_string(),
                         "#.##...#...".to_string(),
                         "#...##....#".to_string(),
                         ".#..#...#.#".to_string()];

        assert_eq!(2, solve(&input, 1, 1));
        assert_eq!(7, solve(&input, 3, 1));
        assert_eq!(3, solve(&input, 5, 1));
        assert_eq!(4, solve(&input, 7, 1));
        assert_eq!(2, solve(&input, 1, 2));
    }
}
