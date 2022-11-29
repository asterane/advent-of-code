// Advent of Code 2015
// Day 25!

const INIT: usize = 20151125;
const MULT: usize = 252533;
const DIVS: usize = 33554393;

fn main() {
    let input = std::include_bytes!("../input.txt");

    let (mut rp, mut nump) = (false, false);
    let mut acc = 0;
    let (mut ir, mut ic) = (0, 0);
    for c in input {
        match c {
            b',' | b'.' if nump => {
                if !rp {
                    ir = acc
                } else {
                    ic = acc
                }
                acc = 0;
                rp = true;
                nump = false;
            }
            _ if c.is_ascii_digit() => {
                acc *= 10;
                acc += (c & 15) as u32;
                nump = true;
            }
            _ if c.is_ascii() => (),
            _ => panic!(),
        }
    }

    // println!("{}", seq_of_pos(dbg!(ir), dbg!(ic)));

    let code = adv_val(seq_of_pos(ir, ic), INIT);

    println!("Code for the machine: {}", code); // 9132360

    // Correct!
}

fn adv_val(n: u32, ini: usize) -> usize {
    let mut cur = ini;
    for _ in 1..n {
        cur *= MULT;
        cur %= DIVS;
    }
    cur
}

fn seq_of_pos(r: u32, c: u32) -> u32 {
    let (mut cr, mut cc) = (1, 1);
    let mut diag = 1;
    let mut count = 1;
    while !(cr == r && cc == c) {
        cr -= 1;
        cc += 1;
        if cr == 0 {
            diag += 1;
            cr = diag;
            cc = 1;
        }
        count += 1
    }
    count
}

#[test]
fn pos_test() {
    assert_eq!(seq_of_pos(1, 1), 1);
    assert_eq!(seq_of_pos(6, 1), 16);
    assert_eq!(seq_of_pos(1, 6), 21);
    assert_eq!(seq_of_pos(3, 3), 13);
}

#[test]
fn val_test() {
    assert_eq!(adv_val(seq_of_pos(1, 1), INIT), INIT);
    assert_eq!(adv_val(seq_of_pos(5, 1), INIT), 77061);
    assert_eq!(adv_val(seq_of_pos(1, 5), INIT), 10071777);
    assert_eq!(adv_val(seq_of_pos(6, 6), INIT), 27995004);
}
