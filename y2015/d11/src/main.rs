// Advent of Code 2015
// Day 11

fn main() {
    let input = b"cqjxjnds";
    let mut pw = input.clone();

    while !chkpw(&pw) {
        incpw(&mut pw);
    }

    let pw_str_1 = String::from_utf8(pw.to_vec()).unwrap();

    incpw(&mut pw);

    while !chkpw(&pw) {
        incpw(&mut pw);
    }

    let pw_str_2 = String::from_utf8(pw.to_vec()).unwrap();

    println!("Next password: {}", pw_str_1); // cqjxxyzz
    println!("Following password: {}", pw_str_2); // cqkaabcc

    // Correct!
}

fn chkpw(pw: &[u8; 8]) -> bool {
    let (mut lastb, mut lastp) = (0, 0);
    let (mut strt, mut par1, mut par2) = (false, false, false);
    for c in pw {
        match c {
            b'i' | b'o' | b'l' => return false,
            _ if c.is_ascii_lowercase() => {
                if lastp == c - 1 && lastb == lastp - 1 {
                    strt = true;
                } else if lastp == *c && lastb != lastp {
                    par2 = par1;
                    par1 = true;
                }
            }
            _ => panic!(),
        }
        lastb = lastp;
        lastp = *c;
    }

    strt && par1 && par2
}

fn incpw(pw: &mut [u8; 8]) {
    let mut wrapped = true;
    let mut i = 7;
    while wrapped {
        pw[i] += 1;
        if pw[i] > b'z' {
            pw[i] = b'a';
            i -= 1;
            continue;
        }
        wrapped = false;
    }
}

#[test]
fn chktest() {
    let tcf = b"hijklmmn";
    let tct = b"abcdffaa";

    assert!(!chkpw(tcf));
    assert!(chkpw(tct));
}
