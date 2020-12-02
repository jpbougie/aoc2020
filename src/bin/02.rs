use std::env;
use std::fs::File;
use std::io::{self, Read};


fn main() -> io::Result<()> {
    let path = env::args().skip(1).next().expect("Specify an input file");
    let mut file = File::open(path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    let (_, rules) = parse::lines(&s).expect("invalid input");
    let valid_count = rules.iter().filter(|(r, i)| r.matches_part1(i)).count();
    println!("Part 01: {}", valid_count);
    let valid_count = rules.iter().filter(|(r, i)| r.matches_part2(i)).count();
    println!("Part 02: {}", valid_count);

    Ok(())
}

use std::ops::RangeInclusive;

#[derive(Debug, PartialEq)]
pub struct Rule {
    range: RangeInclusive<usize>,
    value: char,
}

impl Rule {
    fn matches_part1(&self, input: &str) -> bool {
        self.range.contains(&input.chars().filter(|ch| ch == &self.value).count())
    }

    fn matches_part2(&self, input: &str) -> bool {
        let chars = input.chars().collect::<Vec<_>>();
        input.len() >= *self.range.end() && (chars[*self.range.start() - 1] == self.value) != (chars[*self.range.end() - 1] == self.value)
    }
}

mod parse {
    use nom::{
        IResult,
        character::complete::{digit1, satisfy, line_ending},
        bytes::complete::{tag, take_while},
        combinator::map_res,
        sequence::{pair, terminated},
        multi::many1,
    };

    use std::str::FromStr;
    use std::ops::RangeInclusive;

    use super::Rule;

    fn range(input: &str) -> IResult<&str, RangeInclusive<usize>> {
        let (input, from) = map_res(digit1, FromStr::from_str)(input)?;
        let (input, _) = tag("-")(input)?;
        let (input, to) = map_res(digit1, FromStr::from_str)(input)?;

        Ok((input, from..=to))
    }

    fn rule(input: &str) -> IResult<&str, Rule> {
        let (input, range) = range(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, value) = satisfy(|ch| ch.is_alphabetic())(input)?;
        let (input, _) = tag(": ")(input)?;

        Ok((input, Rule{range, value}))
    }

    fn line(input: &str) -> IResult<&str, (Rule, &str)> {
        pair(rule, terminated(take_while(|ch: char| ch.is_alphabetic()), line_ending))(input)
    }

    pub fn lines(input: &str) -> IResult<&str, Vec<(Rule, &str)>> {
        many1(line)(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn check_range() {
            assert_eq!(Ok(("", 1..=3)), range("1-3"));
        }

        #[test]
        fn check_value() {
            assert_eq!(Ok(("", Rule{range: 1..=3, value: 'a'})), rule("1-3 a: "));
        }

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule() {
        let rule = Rule{range: 1..=3, value: 'a'};
        assert!(rule.matches("abcde"));
    }

}
