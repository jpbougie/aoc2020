use std::fs::File;
use std::io::{self, Read};

use std::collections::{HashMap, HashSet, VecDeque};

fn main() -> io::Result<()> {
    let input = std::env::args().skip(1).next().expect("Specify an input");
    println!("Input is {}", input);
    let mut f = File::open(input)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    println!("Part 01: {}", part1(&s));
    println!("Part 02: {}", part2(&s));
    Ok(())
}

fn part2(input: &str) -> usize {
    let (_, lines) = parse::lines(input).expect("valid input");
    let mut rules = HashMap::new();
    for (container, contained) in lines {
        rules.insert(container, contained);
    }


    weight(&rules, &("shiny", "gold")) - 1

}

fn weight(rules: &HashMap<(&str, &str), Vec<(usize, (&str, &str))>>, bag: &(&str, &str)) -> usize {
    let contained = rules.get(bag).unwrap();

    contained.iter().fold(1, |sum, (w, b)| sum + (w * weight(rules, b)))
}

fn part1(input: &str) -> usize {
    let (_, lines) = parse::lines(input).expect("valid input");

    let mut outer_bags = HashSet::with_capacity(lines.len());
    let mut inverted = HashMap::new();
    for (container, contained) in lines {
        outer_bags.insert(container);

        for (_count, containee) in contained {
            let entry = inverted.entry(containee).or_insert_with(HashSet::new);
            (*entry).insert(container);

        }
    }

    let mut solution = HashSet::new();
    let mut visited = HashSet::new();
    visited.insert(("shiny", "gold"));
    let mut to_visit = inverted.get(&("shiny", "gold")).unwrap().iter().map(|x| *x).collect::<VecDeque<(&str, &str)>>();
    while let Some(x) = to_visit.pop_front() {
        if visited.contains(&x) {
            continue
        }

        if outer_bags.contains(&x) {
            solution.insert(x);
        }

        if let Some(others) = inverted.get(&x) {
            for other in others {
                to_visit.push_back(*other);
            }
        } else {
            solution.insert(x);
        }
    }

    solution.len()
}

mod parse {
    use nom::{
        IResult,
        character::complete::{digit1, line_ending, alpha1},
        bytes::complete::{tag},
        combinator::map_res,
        sequence::{separated_pair},
        multi::{separated_list1},
        branch::alt,
    };

    use std::str::FromStr;

    fn line(i: &str) -> IResult<&str, ((&str, &str), Vec<(usize, (&str, &str))>)> {
        let (i, color_origin) = complex_color(i)?;
        let (i, _) = tag(" bags contain ")(i)?;
        let (i, contained) = alt((no_other_bags, some_bags))(i)?;
        let (i, _) = tag(".")(i)?;

        Ok((i, (color_origin, contained)))

    }

    fn one(i: &str) -> IResult<&str, (usize, (&str, &str))> {
        let (i, _) = tag("1")(i)?;
        let (i, _) = tag(" ")(i)?;
        let (i, col) = complex_color(i)?;
        let (i, _) = tag(" bag")(i)?;


        Ok((i, (1, col)))
    }

    fn many(i: &str) -> IResult<&str, (usize, (&str, &str))> {
        let (i, count) = map_res(digit1, FromStr::from_str)(i)?;
        let (i, _) = tag(" ")(i)?;
        let (i, col) = complex_color(i)?;
        let (i, _) = tag(" bags")(i)?;


        Ok((i, (count, col)))
    }

    fn some_bags(i: &str) -> IResult<&str, Vec<(usize, (&str, &str))>> {
        let (i, bags) = separated_list1(tag(", "), alt((one, many)))(i)?;

        Ok((i, bags))
    }

    fn no_other_bags(i: &str) -> IResult<&str, Vec<(usize, (&str, &str))>> {
        let (i, _) = tag("no other bags")(i)?;

        Ok((i, vec![]))
    }

    fn complex_color(input: &str) -> IResult<&str, (&str, &str)> {
        separated_pair(alpha1, tag(" "), alpha1)(input)
    }

    pub fn lines(input: &str) -> IResult<&str, Vec<((&str, &str), Vec<(usize, (&str, &str))>)>> {
        separated_list1(line_ending, line)(input)
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_works() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        assert_eq!(4, part1(&input));
    }


    #[test]
    fn part2_works() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        assert_eq!(126, part2(&input));
    }


}
