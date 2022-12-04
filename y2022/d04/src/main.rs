// Advent of Code 2022
// Day 4: Camp Cleanup

fn main() {
    let input = std::include_bytes!("../input.txt");

    let (mut l1, mut h1, mut l2) = (0, 0, 0);
    let mut h2;

    let mut acc = 0;
    let mut sep = false;

    let mut range_contains = 0;
    let mut range_overlaps = 0;

    for c in input {
        match c {
            b'\n' => {
                h2 = acc;

                if (l1 >= l2 && h1 <= h2) || (l2 >= l1 && h2 <= h1) {
                    range_contains += 1
                }
                if !(h1 < l2 || h2 < l1) {
                    range_overlaps += 1
                }

                (l1, h1, l2) = (0, 0, 0);
                sep = false;
                acc = 0;
            }
            b'-' => {
                if sep {
                    l2 = acc;
                } else {
                    l1 = acc;
                }
                acc = 0;
            }
            b',' => {
                h1 = acc;
                sep = true;
                acc = 0;
            }
            _ if c.is_ascii_digit() => {
                acc *= 10;
                acc += (c & 15) as u32;
            }
            _ => panic!(),
        }
    }

    // Part 1
    println!("Pairs with fully contained ranges: {}", range_contains); // 485

    // Part 2
    println!("Pairs with overlapping ranges: {}", range_overlaps); // 857

    // Correct!
}
