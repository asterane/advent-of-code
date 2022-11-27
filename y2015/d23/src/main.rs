// Advent of Code 2015
// Day 23

#[derive(Debug, Clone, Copy)]
enum Reg {
    A,
    B,
}

#[derive(Debug)]
enum Opcode {
    Hlf(Reg),
    Tpl(Reg),
    Inc(Reg),
    Jmp(i8),
    Jie(Reg, i8),
    Jio(Reg, i8),
}

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut listing: Vec<Opcode> = Vec::new();

    fn advance(cur: &mut std::slice::Iter<u8>, n: usize) {
        for _ in 0..n {
            cur.next();
        }
    }

    fn reg_parse(cur: &mut std::slice::Iter<u8>) -> Reg {
        match cur.next() {
            Some(b'a') => Reg::A,
            Some(b'b') => Reg::B,
            _ => panic!(),
        }
    }

    fn offset_parse(cur: &mut std::slice::Iter<u8>) -> i8 {
        let pos = match cur.next() {
            Some(b'+') => true,
            Some(b'-') => false,
            _ => panic!(),
        };
        let mut acc = 0;
        loop {
            match cur.next() {
                Some(b'\n') => break if pos { acc } else { -acc },
                Some(c) if c.is_ascii_digit() => {
                    acc *= 10;
                    acc += (c & 15) as i8;
                }
                _ => panic!(),
            }
        }
    }

    let mut cursor = input.iter();
    loop {
        listing.push(match cursor.next() {
            Some(b'h') => {
                advance(&mut cursor, 3);
                let reg = reg_parse(&mut cursor);
                assert_eq!(cursor.next(), Some(&b'\n'));
                Opcode::Hlf(reg)
            }
            Some(b't') => {
                advance(&mut cursor, 3);
                let reg = reg_parse(&mut cursor);
                assert_eq!(cursor.next(), Some(&b'\n'));
                Opcode::Tpl(reg)
            }
            Some(b'i') => {
                advance(&mut cursor, 3);
                let reg = reg_parse(&mut cursor);
                assert_eq!(cursor.next(), Some(&b'\n'));
                Opcode::Inc(reg)
            }
            Some(b'j') => match cursor.next() {
                Some(b'm') => {
                    advance(&mut cursor, 2);
                    Opcode::Jmp(offset_parse(&mut cursor))
                }
                Some(b'i') => match cursor.next() {
                    Some(b'e') => {
                        cursor.next();
                        Opcode::Jie(reg_parse(&mut cursor), {
                            advance(&mut cursor, 2);
                            offset_parse(&mut cursor)
                        })
                    }
                    Some(b'o') => {
                        cursor.next();
                        Opcode::Jio(reg_parse(&mut cursor), {
                            advance(&mut cursor, 2);
                            offset_parse(&mut cursor)
                        })
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            },
            None => break,
            _ => panic!(),
        })
    }

    let listing = listing;

    let (_, b1) = execute(&listing, 0, 0);
    let (_, b2) = execute(&listing, 1, 0);

    println!("Value in =b= with 0 in =a=: {}", b1); // 184
    println!("Value in =b= with 1 in =a=: {}", b2); // 231

    // Correct!
}

fn execute(listing: &[Opcode], in_a: u32, in_b: u32) -> (u32, u32) {
    let (mut a, mut b) = (in_a, in_b);

    let mut pc = 0;
    loop {
        if pc >= listing.len() {
            break;
        }
        match listing[pc] {
            Opcode::Hlf(r) => match r {
                Reg::A => a /= 2,
                Reg::B => b /= 2,
            },
            Opcode::Tpl(r) => match r {
                Reg::A => a *= 3,
                Reg::B => b *= 3,
            },
            Opcode::Inc(r) => match r {
                Reg::A => a += 1,
                Reg::B => b += 1,
            },
            Opcode::Jmp(o) => {
                pc = (pc as isize + o as isize) as usize;
                continue;
            }
            Opcode::Jie(r, o) => {
                let reg = match r {
                    Reg::A => &mut a,
                    Reg::B => &mut b,
                };
                if *reg & 1 == 0 {
                    pc = (pc as isize + o as isize) as usize;
                    continue;
                }
            }
            Opcode::Jio(r, o) => {
                let reg = match r {
                    Reg::A => &mut a,
                    Reg::B => &mut b,
                };
                if *reg == 1 {
                    pc = (pc as isize + o as isize) as usize;
                    continue;
                }
            }
        }
        pc += 1;
    }

    (a, b)
}
