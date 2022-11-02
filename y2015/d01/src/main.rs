// Advent of Code 2015
// Day 1

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut floor = 0;
    let mut bsmt = None;

    let mut cursor = input.iter().enumerate();
    while let Some((i, b)) = cursor.next() {
        match b {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => panic!(),
        }

        if bsmt.is_none() && floor < 0 {
            bsmt = Some(i + 1)
        }
    }

    println!("Final Floor: {}", floor); // 138

    println!("Basement Entry: {}", bsmt.unwrap_or(0)); // 1771

    // Correct!
}
