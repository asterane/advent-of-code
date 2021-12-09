use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("input.txt").unwrap();
    let reader = BufReader::new(input);

    let mut last = u16::MAX;
    let mut curr: u16;
    let mut ct = 0;

    let mut lsum = u16::MAX;
    let mut win: [u16; 3] = [0; 3];
    let mut winct = 0;

    for line in reader.lines() {
        let val = line.unwrap().parse().unwrap();

        // Part One
        curr = val;
        if curr > last {
            ct = ct + 1;
        }
        last = curr;

        // Part Two
        win[2] = win[1];
        win[1] = win[0];
        win[0] = val;

        if win[0] + win[1] + win[2] > lsum {
            winct = winct + 1;
        }

        lsum = win[0] + win[1] + win[2];
    }

    println!("P1: {}", ct);
    println!("P2: {}", winct - 2);

    // Correct!
}
