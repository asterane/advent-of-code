// Advent of Code 2015
// Day 5

fn main() {
    let input = std::include_bytes!("../input.txt");

    let nice_ct_1 = {
        let mut count = 0;

        let mut last = b'\n';
        let mut vwlc = 0;
        let mut dblp = false;
        let mut dnyp = false;

        for b in input {
            match b {
                b'\n' => {
                    if dblp && !dnyp && vwlc >= 3 {
                        count += 1;
                    }
                    dblp = false;
                    dnyp = false;
                    vwlc = 0;
                }
                c if b.is_ascii_alphabetic() => match c {
                    b'a' | b'e' | b'i' | b'o' | b'u' => vwlc += 1,
                    b'b' if last == b'a' => dnyp = true,
                    b'd' if last == b'c' => dnyp = true,
                    b'q' if last == b'p' => dnyp = true,
                    b'y' if last == b'x' => dnyp = true,
                    _ => (),
                },
                _ => panic!(),
            }

            if last == *b {
                dblp = true
            }

            last = *b;
        }

        count
    };

    let nice_ct_2 = {
        let mut count = 0;

        let mut elt = Vec::new();

        let (mut pair_p, mut lett_p) = (false, false);

        for b in input {
            if *b == b'\n' {
                if lett_p {
                    'out: for (i, w) in elt.windows(2).enumerate() {
                        for x in elt[i + 2..].windows(2) {
                            if w == x {
                                pair_p = true;
                                break 'out;
                            }
                        }
                    }

                    count += pair_p as i32;
                }

                (pair_p, lett_p) = (false, false);
                elt.clear();
                continue;
            }
            elt.push(*b);
            let idx = elt.len() - 1;
            lett_p = lett_p || (idx >= 2 && elt[idx - 2] == *b);
        }

        count
    };

    println!("Nice strings 1: {}", nice_ct_1); // 236
    println!("Nice strings 2: {}", nice_ct_2); // 51

    // Correct!
}
