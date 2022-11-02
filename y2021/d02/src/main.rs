use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("input.txt").unwrap();
    let reader = BufReader::new(input);

    let mut h = 0;
    let mut d = 0;

    let mut aim = 0;
    let mut ah = 0;
    let mut ad = 0;

    for line in reader.lines() {
        let cmd = line.unwrap();
        let dir = cmd.as_bytes()[0];

        let dist: i32 = cmd
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        match dir {
            b'f' => {
                h += dist;
                ah += dist;
                ad += aim * dist;
            }
            b'd' => {
                d += dist;
                aim += dist;
            }
            b'u' => {
                d -= dist;
                aim -= dist;
            }
            _ => panic!("invalid"),
        }
    }

    println!("P1: {}", h * d);   // 1990000
    println!("P2: {}", ah * ad); // 1975421260

    // Correct!
}
