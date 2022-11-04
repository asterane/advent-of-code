// Advent of Code 2015
// Day 3

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut received = vec![(0, 0)];
    let mut coords = (0, 0);

    let mut cursor = input.iter();
    while let Some(b) = cursor.next() {
        let (x, y) = coords;
        match b {
            b'^' => coords = (x, y + 1),
            b'v' => coords = (x, y - 1),
            b'>' => coords = (x + 1, y),
            b'<' => coords = (x - 1, y),
            _ => panic!(),
        }
        received.push(coords);
    }

    received.sort();
    received.dedup();

    println!("Santa alone hit: {}", received.len()); // 2081

    let mut recv_redux = vec![(0, 0)];
    let mut santa = (0, 0);
    let mut robot = (0, 0);

    let mut tick = true;

    let mut cursor = input.iter();
    while let Some(b) = cursor.next() {
        let cur = if tick { &mut santa } else { &mut robot };
        let (x, y) = cur.clone();
        match b {
            b'^' => *cur = (x, y + 1),
            b'v' => *cur = (x, y - 1),
            b'>' => *cur = (x + 1, y),
            b'<' => *cur = (x - 1, y),
            _ => panic!(),
        }
        recv_redux.push(*cur);
        tick ^= true;
    }

    recv_redux.sort();
    recv_redux.dedup();

    println!("Santa plus robot hit: {}", recv_redux.len()); // 2341

    // Correct!
}
