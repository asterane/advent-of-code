// Advent of Code 2022
// Day 6: Tuning Trouble

fn main() {
    let input = std::include_bytes!("../input.txt");

    let pkt = find_first_distinct(&input[..], 4);
    let msg = find_first_distinct(&input[..], 14);

    // Part 1
    println!("Characters before packet marker: {}", pkt); // 1702

    // Part 2
    println!("Characters before message marker: {}", msg); // 3559

    // Correct!
}

fn find_first_distinct(buf: &[u8], len: usize) -> usize {
    let mut pos = 0;
    for (i, w) in buf.windows(len).enumerate() {
        if !(1..len).any(|j| w[j..].contains(&w[j - 1])) {
            pos = i + len;
            break;
        }
    }
    pos
}
