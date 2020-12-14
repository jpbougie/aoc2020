use std::env;
use std::io::{self, Read};
use std::fs::File;
use std::collections::BinaryHeap;

fn main() -> io::Result<()> {
    let f = env::args().skip(1).next().unwrap();
    let mut file = File::open(f)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    let mut lines = s.lines();

    let arrival_time = lines.next().unwrap().parse::<i64>().expect("arrival time");
    let buses = lines.next().unwrap().split(",").enumerate().filter_map(|(i, w)| w.parse().ok().map(|x| (x, i as i64))).collect::<Vec<(i64, i64)>>();

    println!("Part 01: {}", part1(arrival_time, &buses));
    println!("Part 02: {}", part2_crt(&buses.iter().map(|(modulo, offset)| (*modulo, modulo - offset % modulo)).collect::<Vec<_>>()));

    Ok(())
}

fn part1(arrival_time: i64, buses: &[(i64, i64)]) -> i64 {

    let mut heap = BinaryHeap::with_capacity(buses.len());
    for (b, _) in buses.iter() {
        heap.push(Bus{id: *b, next_arrival: *b});
    }

    while let Some(to_consider) = heap.pop() {
        if to_consider.next_arrival < arrival_time {
            heap.push(Bus{id: to_consider.id, next_arrival: to_consider.next_arrival + to_consider.id});
            continue
        }

        return to_consider.id * (to_consider.next_arrival - arrival_time)
    };

    0
}

fn part2(buses: &[(i64, i64)]) -> i64 {
    let stride = buses.iter().max_by(|(id_a, _), (id_b, _)| id_a.cmp(id_b)).unwrap();
    let rels = buses.iter().map(|(id, off)| (*id, off - stride.1)).collect::<Vec<_>>();
    let mut t = stride.0;
    println!("stride is {:?}", stride);
    loop {
        if rels.iter().all(|(id, off)| (t + off) % *id == 0) {
            let first = rels.iter().map(|(_, off)| *off).min().unwrap();
            return t + first;
        }

        t += stride.0;
    }
}

fn part2_crt(buses: &[(i64, i64)]) -> i64 {
    let M: i64 = buses.iter().map(|x| x.0).product();

    buses.iter().map(|(m, a)| {
        a * (M/m) * mod_inv(M/m, *m).unwrap()
    }).sum::<i64>() % M
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
 
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Bus {
    id: i64,
    next_arrival: i64,
}

use std::cmp::Ordering;
impl Ord for Bus {
    fn cmp(&self, other: &Self) -> Ordering {
        other.next_arrival.cmp(&self.next_arrival)
    }
}

impl PartialOrd for Bus {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.next_arrival.cmp(&self.next_arrival))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part2_works() {
        //17,x,13,19
        let buses = vec![(17, 0), (13, 11), (19, 16)];
        assert_eq!(3417, part2_crt(&buses));
        let buses = vec![(67, 0),(7, 6),(59, 57),(61, 58)];
        assert_eq!(754018, part2_crt(&buses));
    }

    #[test]
    fn crt_works() {
        let inputs = vec![(5, 2), (7, 3)];
        assert_eq!(17, part2_crt(&inputs));
    }
}
