// Advent of Code 2016
// Day 8: Two-Factor Authentication

enum Op {
    Rect(usize, usize),
    RotR(usize, usize),
    RotC(usize, usize),
}

const SW: usize = 50;
const SH: usize = 6;

fn main() {
    let input = include_bytes!("../input.txt");

    let mut screen = [[false; SH]; SW];

    let mut rop;

    let mut bytes = input.iter();
    loop {
        match bytes.next() {
            Some(b) => assert_eq!(*b, b'r'),
            None => break,
        }

        match bytes.next().unwrap() {
            b'e' => rop = true,
            b'o' => rop = false,
            _ => panic!(),
        }

        while *bytes.next().unwrap() != b' ' {}

        let inst = if rop {
            let mut w = 0;
            let mut acc = 0;
            loop {
                match bytes.next().unwrap() {
                    b'\n' => break,
                    b'x' => {
                        w = acc;
                        acc = 0;
                    }
                    b if b.is_ascii_digit() => {
                        acc *= 10;
                        acc += (b & 0xF) as usize;
                    }
                    _ => panic!(),
                }
            }
            Op::Rect(w, acc)
        } else {
            let isx;
            match bytes.next().unwrap() {
                b'c' => isx = true,
                b'r' => isx = false,
                _ => panic!(),
            }

            while *bytes.next().unwrap() != b' ' {}

            if isx {
                assert_eq!(*bytes.next().unwrap(), b'x');
            } else {
                assert_eq!(*bytes.next().unwrap(), b'y');
            }

            bytes.next();

            let mut crd = 0;
            let mut acc = 0;

            loop {
                match bytes.next().unwrap() {
                    b'\n' => break,
                    b' ' => {
                        crd = acc;
                        acc = 0;
                        bytes.next();
                        bytes.next();
                        assert_eq!(*bytes.next().unwrap(), b' ');
                    }
                    b if b.is_ascii_digit() => {
                        acc *= 10;
                        acc += (b & 0xF) as usize;
                    }
                    _ => panic!(),
                }
            }

            if isx {
                Op::RotC(crd, acc)
            } else {
                Op::RotR(crd, acc)
            }
        };

        match inst {
            Op::Rect(w, h) => {
                for x in 0..w {
                    for y in 0..h {
                        screen[x][y] = true;
                    }
                }
            }
            Op::RotC(col, shf) => screen[col].rotate_right(shf),
            Op::RotR(row, shf) => {
                for _ in 0..shf {
                    let spill = screen[SW - 1][row];
                    for i in (1..SW).rev() {
                        screen[i][row] = screen[i - 1][row];
                    }
                    screen[0][row] = spill;
                }
            }
        }
    }

    let count = screen.iter().flatten().filter(|e| **e).count();

    println!("Lit pixels: {}", count); // 121

    println!("Display output:"); // RURUCEOEIL
    for y in 0..SH {
        for x in 0..SW {
            if screen[x][y] {
                print!("##");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}
