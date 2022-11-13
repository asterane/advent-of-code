// Advent of Code 2015
// Day 10

fn main() {
    let input = b"1321131112";

    let mut vec = Vec::new();
    let mut nex = &input[..];
    for _ in 0..40 {
        vec = next(nex);
        nex = &vec;
    }

    let len_40 = vec.len();

    println!("Length after 40: {}", len_40); // 492982

    for _ in 0..10 {
        vec = next(nex);
        nex = &vec;
    }

    let len_50 = vec.len();

    println!("Length after 50: {}", len_50); // 6989950

    // Correct!
}

fn next(from: &[u8]) -> Vec<u8> {
    let mut run_len = 1;
    let mut acc = Vec::new();

    let mut cur = from.iter().chain(std::iter::once(&0));
    let mut last = cur.next().unwrap() & 15;
    for c in cur {
        assert!(*c == b'1' || *c == b'2' || *c == b'3' || *c == 0);
        if last != c & 15 {
            acc.push(run_len | 0x30);
            acc.push(last | 0x30);
            run_len = 0;
        }
        last = c & 15;
        run_len += 1;
    }
    acc
}

#[test]
fn test() {
    let input = b"111221";
    let xpect = b"312211";

    let out = next(input);
    assert_eq!(&xpect[..], &out[..]);
}
