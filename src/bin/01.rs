use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::cmp::Ordering;

fn main() -> io::Result<()> {
    let input = std::env::args().skip(1).next().expect("Specify an input");
    println!("Input is {}", input);
    let f = File::open(input)?;
    let f = BufReader::new(f);
    let mut numbers = f.lines().map(|l| l.unwrap().parse::<i64>().expect("invalid int")).collect::<Vec<i64>>();
    numbers.sort();

    part1(&numbers);
    part2(&numbers);
    Ok(())
}

fn part1(numbers: &Vec<i64>) {
    let l = numbers.len();

    let mut i = 0;
    while i < l {
         for j in (i+1)..l {
            match (numbers[i] + numbers[j]).cmp(&2020) {
                Ordering::Equal => {
                    println!("Solution to part 1 is {}", numbers[i] * numbers[j]);
                }
                Ordering::Greater => break,
                _ => {},
            }
         }
         i += 1;
    };
}

fn part2(numbers: &Vec<i64>) {
    let l = numbers.len();

    let mut i = 0;
    while i < l {
         for j in (i+1)..l {
            match (numbers[i] + numbers[j]).cmp(&2020) {
                Ordering::Greater | Ordering::Equal => break,
                _ => {
                    for k in (j+1)..l {
                        match (numbers[i] + numbers[j] + numbers[k]).cmp(&2020) {
                            Ordering::Equal => {
                                println!("Solution to part 2 is {}", numbers[i] * numbers[j] * numbers[k]);
                            }
                            Ordering::Greater => break,
                            _ => {},
                        }
                    }
                }
            }
         }
         i += 1;
    };
}
