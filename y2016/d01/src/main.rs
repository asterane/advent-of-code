// Advent of Code 2016
// Day 1: No Time for a Taxicab

enum Heading {
    N,
    E,
    S,
    W,
}

fn main() {
    let input = include_bytes!("../input.txt");

    let mut head = Heading::N;
    let mut coords = (0, 0);

    let mut hist: Vec<(i32, i32)> = vec![];
    let mut last = coords;
    let mut fvt = None;

    let mut bytes = input.iter();
    loop {
        use Heading::*;

        match bytes.next().unwrap() {
            b'R' => match head {
                N => head = E,
                E => head = S,
                S => head = W,
                W => head = N,
            },
            b'L' => match head {
                N => head = W,
                E => head = N,
                S => head = E,
                W => head = S,
            },
            _ => panic!(),
        }

        let mut dacc = 0;
        loop {
            match bytes.next().unwrap() {
                b',' | b'\n' => break,
                n if n.is_ascii_digit() => {
                    dacc *= 10;
                    dacc += (n & 0xF) as i32;
                }
                _ => panic!(),
            }
        }

        match head {
            N => coords.1 += dacc,
            E => coords.0 += dacc,
            S => coords.1 -= dacc,
            W => coords.0 -= dacc,
        }

        if fvt.is_none() {
            let pos = hist.windows(2).find_map(|w| {
                if (w[0].0 == w[1].0 && last.0 == coords.0)
                    || (w[0].1 == w[1].1 && last.1 == coords.1)
                {
                    None
                } else if w[0].0 == w[1].0 {
                    assert_eq!(last.1, coords.1);

                    let x = w[0].0;
                    let y = coords.1;
                    if ((x >= last.0 && x <= coords.0) || (x <= last.0 && x >= coords.0))
                        && ((y >= w[0].1 && y <= w[1].1) || (y <= w[0].1 && y >= w[1].1))
                    {
                        Some((x, y))
                    } else {
                        None
                    }
                } else {
                    assert_eq!(w[0].1, w[1].1);
                    assert_eq!(last.0, coords.0);

                    let y = w[0].1;
                    let x = coords.0;
                    if ((x >= w[0].0 && x <= w[1].0) || (x <= w[0].0 && x >= w[1].0))
                        && ((y >= last.1 && y <= coords.1) || (y <= last.1 && y >= coords.1))
                    {
                        Some((x, y))
                    } else {
                        None
                    }
                }
            });

            fvt = pos;

            hist.push(last);
            last = coords;
        }

        match bytes.next() {
            None => break,
            Some(b' ') => (),
            Some(_) => panic!(),
        }
    }

    let cab_dist_tgt = coords.0.abs() + coords.1.abs();
    let cab_dist_fvt = match fvt {
        None => -1,
        Some((x, y)) => x.abs() + y.abs(),
    };

    println!("Distance to target: {}", cab_dist_tgt); // 262

    println!("Distance to first location visited twice: {}", cab_dist_fvt); // 131
}
