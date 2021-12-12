use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Count the number of lines and the number of 1s in each
    // position, then subtract and compare to determine whether 1s or
    // 0s were more common (more than half the number of lines means
    // more common). The two desired numbers are inverses of each
    // other, so it's easy to construct both at once. Multiply to
    // finish.

    let input = File::open("input.txt").unwrap();
    let reader = BufReader::new(input);

    let mut freq: [u16; 12] = [0; 12];
    let mut count = 0;

    let mut list: Vec<u16> = Vec::new();

    for line in reader.lines() {
        list.push(0);
        for bit in line.unwrap().as_bytes().iter().enumerate() {
            match bit.1 {
                b'0' => (),
                b'1' => {
                    freq[bit.0] += 1;
                    list[count] |= 1 << (11 - bit.0);
                }
                _ => panic!("invalid"),
            }
        }
        count += 1;
    }

    let dec: (u32, u32) =
        freq.iter()
            .enumerate()
            .fold((0, 0), |a: (u32, u32), elt: (usize, &u16)| {
                if *(elt.1) as usize > (count / 2) {
                    (a.0, a.1 + (1 << (11 - elt.0)))
                } else {
                    (a.0 + (1 << (11 - elt.0)), a.1)
                }
            });

    let ox = rate(list.clone(), true) as u32;
    let co2 = rate(list, false) as u32;

    println!("P1: {}", dec.0 * dec.1); // 1071734
    println!("P2: {}", ox * co2);      // 6124992

    // Correct!
}

fn rate(mut coll: Vec<u16>, most: bool) -> u16 {
    let mut pow = 11;
    while coll.len() != 1 {
        let bit = {
            let onect = coll
                .iter()
                .fold(0, |a, n| a + ((n & (1 << pow)) != 0) as usize);
            let zerct = coll.len() - onect;
            if onect >= zerct {
                most
            } else {
                !most
            }
        };
        coll.retain(|elt| ((elt & (1 << pow)) != 0) == bit);
        pow -= 1;
    }
    coll[0]
}
