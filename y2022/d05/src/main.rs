// Advent of Code 2022
// Day 5: Supply Stacks

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut stacks: Vec<Vec<u8>> = Vec::new();
    let (mut llen, mut mlen) = (0, 0);

    let mut cursor = input.iter();
    loop {
        match cursor.next().unwrap() {
            b' ' => {
                llen += 1;
                if llen > mlen {
                    mlen = llen;
                    stacks.push(Vec::new());
                }
                if cursor.next().unwrap().is_ascii_digit() {
                    break;
                }
                cursor.next();
                match cursor.next().unwrap() {
                    b'\n' => {
                        llen = 0;
                        continue;
                    }
                    b' ' => (),
                    _ => panic!(),
                }
            }
            b'[' => {
                llen += 1;
                if llen > mlen {
                    mlen = llen;
                    stacks.push(Vec::new());
                }

                stacks[llen - 1].push(*(cursor.next().unwrap()));

                cursor.next();
                match cursor.next().unwrap() {
                    b'\n' => {
                        llen = 0;
                        continue;
                    }
                    b' ' => (),
                    _ => panic!(),
                }
            }
            c => panic!("{c}"),
        }
    }

    while *(cursor.next().unwrap()) != b'\n' {}
    cursor.next();

    for s in stacks.iter_mut() {
        s.reverse()
    }

    let mut stacks_0 = stacks.clone();
    let mut stacks_1 = stacks;

    let (mut ct, mut fr, mut to) = (0, 0, 0);
    let (mut ctp, mut frp) = (false, false);
    let mut nump = false;

    for c in cursor {
        match c {
            b'\n' => {
                nump = false;
                (ctp, frp) = (false, false);
                {
                    let (ct, fr, to) = (ct as usize, fr as usize, to as usize);

                    for _ in 0..ct {
                        let off = stacks_0[fr - 1].pop().unwrap();
                        stacks_0[to - 1].push(off)
                    }

                    let stk_frm = &mut (stacks_1[fr - 1]);
                    let lifted = Vec::from(&stk_frm[stk_frm.len() - ct..]);
                    stk_frm.truncate(stk_frm.len() - ct);
                    stacks_1[to - 1].extend_from_slice(&lifted);
                }
                (ct, fr, to) = (0, 0, 0)
            }
            b' ' => {
                if nump {
                    frp |= ctp;
                    ctp = true;
                }
                nump = false
            }
            _ if c.is_ascii_digit() => {
                let t = if ctp && frp {
                    &mut to
                } else if ctp {
                    &mut fr
                } else {
                    &mut ct
                };

                *t *= 10;
                *t += *c & 15;

                nump = true;
            }
            _ if c.is_ascii_lowercase() => (),
            _ => panic!(),
        }
    }

    // println!("{stacks:?}");

    let msg_0 = top_crates(stacks_0);
    let msg_1 = top_crates(stacks_1);

    // Part 1
    println!("Crates atop each stack (CM 9000): {}", msg_0); // TLNGFGMFN

    // Part 2
    println!("Crates atop each stack (CM 9001): {}", msg_1); // FGLQJCMBD

    // Correct!
}

fn top_crates(stacks: Vec<Vec<u8>>) -> String {
    let mut msg = Vec::new();
    for s in stacks {
        msg.push(*(s.last().unwrap()))
    }
    String::from_utf8(msg).unwrap()
}
