// Advent of Code 2022
// Day 10: Cathode-Ray Tube

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut cyc = 0;
    let mut x = 1;

    let mut strength_sum = 0;
    let mut crt_out: Vec<u8> = Vec::new();

    macro_rules! adv_cyc {
        () => {
            let pos = cyc % 40;
            crt_out.push(if x - 1 == pos || x == pos || x + 1 == pos {
                b'#'
            } else {
                b'.'
            });

            cyc += 1;

            if cyc % 40 == 0 {
                crt_out.push(b'\n')
            }
            if cyc % 40 == 20 {
                strength_sum += cyc * x
            }
        };
    }

    let mut cur = input.iter();
    loop {
        match cur.next() {
            Some(b'a') => {
                cur.next();
                cur.next();
                cur.next();
                assert_eq!(cur.next(), Some(&b' '));
            }
            Some(b'n') => {
                cur.next();
                cur.next();
                cur.next();
                assert_eq!(cur.next(), Some(&b'\n'));

                adv_cyc!();

                continue;
            }
            Some(_) => panic!(),
            None => break,
        }

        let mut neg = false;
        let mut acc = 0;

        loop {
            match cur.next().unwrap() {
                b'\n' => break,
                b'-' => neg = true,
                c if c.is_ascii_digit() => {
                    acc *= 10;
                    acc += (c & 15) as i32;
                }
                _ => panic!(),
            }
        }

        if neg {
            acc = -acc
        }

        adv_cyc!();
        adv_cyc!();

        x += acc;
    }

    // Part 1
    println!("Signal strength sum: {}", strength_sum); // 13740

    // Part 2
    println!("CRT display:\n\n{}", String::from_utf8(crt_out).unwrap()); // ZUPRFECL

    // Correct!
}
