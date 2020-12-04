use std::env;
use std::fs::File;
use std::io::{self, Read};
use regex::Regex;

use std::collections::HashMap;

fn main() -> io::Result<()> {
    let path = env::args().skip(1).next().expect("Specify an input file");
    let mut file = File::open(path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    let passports = s.split("\n\n").map(|entry| {
        let mut pass = HashMap::new();

        entry.split_whitespace().for_each(|kv| {
            let mut it = kv.split(":");
            let k = it.next().unwrap();
            let v = it.next().unwrap();
            pass.insert(k, v);
        });

        pass
    }).collect::<Vec<_>>();

    println!("Part 01: {}", passports.iter().filter(|p| p.len() == 8 || (p.len() == 7 && !p.contains_key("cid"))).count());
    println!("Part 02: {}", passports.iter().filter(|p| valid_part2(p)).count());

    Ok(())
}

fn valid_part2(passport: &HashMap<&str, &str>) -> bool {
    if let Some(yr) = passport.get("byr") {
        if yr < &"1920" || yr > &"2002" {
            return false
        }
    } else {
        return false
    }

    if let Some(yr) = passport.get("iyr") {
        if yr < &"2010" || yr > &"2020" {
            return false
        }
    } else {
        return false
    }

    if let Some(yr) = passport.get("eyr") {
        if yr < &"2020" || yr > &"2030" {
            return false
        }
    } else {
        return false
    }

    if let Some(h) = passport.get("hgt") {
        if h.ends_with("in") {
            if h < &"59in" || h > &"76cm" {
                return false
            }
        } else if h.ends_with("cm") {
            if h < &"150cm" || h > &"193cm" {
                return false
            }
        } else {
            return false
        }
    } else {
        return false
    }

    if let Some(c) = passport.get("hcl") {
        let re = Regex::new("^#[0-9a-f]{6}$").unwrap();
        if !re.is_match(c) {
            return false
        }
    } else {
        return false
    }

    if let Some(c) = passport.get("ecl") {
        let re = Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        if !re.is_match(c) {
            return false
        }
    } else {
        return false
    }

    if let Some(p) = passport.get("pid") {
        let re = Regex::new("^\\d{9}$").unwrap();
        if !re.is_match(p) {
            return false
        }
    } else {
        return false
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse<'a>(entry: &'a str) -> HashMap<&'a str, &'a str> {
        let mut pass = HashMap::new();

        entry.split_whitespace().for_each(|kv| {
            let mut it = kv.split(":");
            let k = it.next().unwrap();
            let v = it.next().unwrap();
            pass.insert(k, v);
        });

        pass
    }

    #[test]
    fn check() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f";
        let passport = parse(input);
        assert!(valid_part2(&passport));
    }

}
