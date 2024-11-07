// Advent of Code 2016
// Day 6: Signals and Noise

fn main() {
    let input = include_bytes!("../input.txt");

    let mut freq: Vec<[u8; 26]> = vec![];
    let mut fl = true;
    let mut idx = 0;

    for b in input {
        match b {
            b'\n' => {
                fl = false;
                idx = 0
            }
            _ if b.is_ascii_alphabetic() => {
                if fl {
                    freq.push([0; 26]);
                }

                freq[idx][(b - b'a') as usize] += 1;

                idx += 1;
            }
            _ => panic!(),
        }
    }

    let msg_max: Vec<u8> = freq
        .iter()
        .map(|e| b'a' + e.iter().enumerate().max_by_key(|(_, n)| **n).unwrap().0 as u8)
        .collect();

    let msg_min: Vec<u8> = freq
        .iter()
        .map(|e| {
            b'a' + e
                .iter()
                .enumerate()
                .fold((0, u8::MAX), |a, e| {
                    if *e.1 == 0 || *e.1 >= a.1 {
                        a
                    } else {
                        (e.0, *e.1)
                    }
                })
                .0 as u8
        })
        .collect();

    println!(
        "Max repetition message: {}",
        String::from_utf8_lossy(&msg_max)
    ); // kjxfwkdh

    println!(
        "Min repetition message: {}",
        String::from_utf8_lossy(&msg_min)
    ); // xrwcsnps
}
