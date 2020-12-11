use std::io::{self, BufRead, BufReader};
use std::env;
use std::fs::File;

fn main() -> io::Result<()> {
    let file = env::args().skip(1).next().unwrap();
    let f = File::open(file)?;
    let f = BufReader::new(f);
    let numbers = f.lines().map(|l| l.unwrap().parse().unwrap()).collect::<Vec<u64>>();
    let mut weakness = 0;
    let mut index = 0;
    for wnd in numbers.windows(26) {
        let mut found = false;
        let needle = wnd[25];
        'outer: for i in 0..=24 {
            for j in 0..=24 {
                if i != j {
                    if wnd[i] + wnd[j] == needle {
                        found = true;
                        break 'outer
                    }
                }
            }
        }

        if !found {
            weakness = needle;
            println!("Part 01: {}", needle);
            break
        }
        index +=1;
    }


    let possible = &numbers[0..index];

    'all: for window_size in 2..index {
        for wnd in possible.windows(window_size) {
            if wnd.iter().fold(0, |s, x| s + *x) == weakness {
                println!("Part 02: {}", wnd.iter().min().unwrap() + wnd.iter().max().unwrap());
                break 'all
            }
        }
    }

    Ok(())
}
