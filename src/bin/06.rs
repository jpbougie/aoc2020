use std::env;
use std::fs::File;
use std::io::{self, Read};
//use regex::Regex;

use std::collections::HashSet;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let path = env::args().skip(1).next().expect("Specify an input file");
    let mut file = File::open(path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    let groups = s.split("\n\n").collect::<Vec<_>>();
    let answers_part1 = groups.iter().map(|group| group.split_whitespace().fold(HashSet::new(), |mut h, a| {
        a.chars().for_each(|c| { h.insert(c); });
        h
    })).collect::<Vec<HashSet<char>>>();

    println!("Part 01: {}", answers_part1.iter().map(HashSet::len).sum::<usize>());

    let answer_part2: usize = groups.iter().map(|group| {
        let indiv = group.split_whitespace().collect::<Vec<_>>();
        let h = indiv.iter().fold(HashMap::new(), |mut h, a| {
            a.chars().for_each(|c| {
                let entry = h.entry(c).or_insert(0);
                *entry += 1;
            });
            h
        });
        h.iter().filter(|(_k, v)| **v == indiv.len()).count()
    }).sum();
    println!("Part 02: {}", answer_part2);

    Ok(())
}
