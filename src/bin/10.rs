use std::io::{self, BufRead, BufReader};
use std::env;
use std::fs::File;

fn main() -> io::Result<()> {
    let file = env::args().skip(1).next().unwrap();
    let f = File::open(file)?;
    let f = BufReader::new(f);
    let mut numbers = f.lines().map(|l| l.unwrap().parse().unwrap()).collect::<Vec<u64>>();
    numbers.push(0);
    numbers.sort();

    let diffs = numbers.windows(2).map(|wnd| wnd[1] - wnd[0]).collect::<Vec<u64>>();

    println!("Part 01: {}", diffs.iter().filter(|x| **x == 1).count() * (diffs.iter().filter(|x| **x == 3).count() + 1));

    println!("Part 02: {}", combi(&numbers));
    Ok(())
}

fn combi(input: &[u64]) -> usize {
    let mut nums = input.iter().map(|x| *x).collect::<Vec<u64>>();
    nums.sort();
    let windows = nums.windows(2).collect::<Vec<_>>();
    let parts = windows.split(|wnd| wnd[1] - wnd[0] == 3).collect::<Vec<_>>();
    let parts = parts.iter().map(|wnd| wnd.len()).collect::<Vec<_>>();
    println!("{:?}", parts);
    let mut cache = HashMap::new();
    cache.insert(3, 2);
    cache.insert(4, 4);
    cache.insert(5, 7);
    parts.iter().filter(|l| **l > 1).map(|l| permutations((1..=(l+1)).collect(), &mut cache)).product::<usize>()
}

use std::collections::{HashMap};
fn permutations(input: Vec<usize>, cache: &mut HashMap<usize, usize>) -> usize {
    if let Some(value) = cache.get(&input.len()) {
        return *value
    }

    let mut target = input.last().unwrap();
    let mut paths = Vec::new();
    let mut possibilities = 0;
    paths.push(vec![1]);
    while !paths.is_empty() {
        let to_test = paths.pop().unwrap();
        let last = to_test.last().unwrap();
        if last == target {
            println!("{:?} is a valid path", to_test);
            possibilities += 1;
        }

        let new_options = input.iter().filter(|x| *x > last && **x <= last + 3);
        new_options.for_each(|opt| {
            let mut new_opt = to_test.clone();
            new_opt.push(*opt);
            paths.push(new_opt);
        });

    }

    cache.insert(input.len(), possibilities);

    possibilities
}

// 1 2 3 4
// 1 3 4
// 1 4
// 1 2 4

//  1 2 3 4 5
// 1 2
//  1 2 5
//  1 2 3 5
//  1 2 4 5
// 1 3
//  1 3 5
//  1 3 4 5
// 1 4
//  1 4 5


#[cfg(test)]
mod tests {
    use super::combi;

    #[test]
    fn combi_works() {
        let num = &[0, 16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!(8, combi(num));
        let num = &[0, 28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
        assert_eq!(19208, combi(num));

    }
}
