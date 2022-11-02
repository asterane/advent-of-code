fn main() {
    let mut draws: Vec<u8> = vec![0];
    let mut boards: Vec<[[(u8, bool); 5]; 5]> = vec![[[(0, false); 5]; 5]];

    let mut input_file = std::fs::File::open("input.txt").unwrap();
    let mut input = Vec::new();
    std::io::Read::read_to_end(&mut input_file, &mut input).unwrap();

    let (mut d, mut b, mut x, mut y) = (0, 0, 0, 0);
    let mut wait = true;

    let mut bytes = input.iter();

    loop {
        let c = bytes.next().unwrap();
        match c {
            b'\n' => break,
            b',' => {
                d += 1;
                draws.push(0);
            }
            _ if c.is_ascii_digit() => draws[d] = (draws[d] * 10) + (c - 48),
            _ => panic!("invalid"),
        }
    }

    loop {
        let c = match bytes.next() {
            Some(c) => c,
            None => break,
        };
        match c {
            b' ' | b'\n' => {
                if !wait {
                    x += 1;
                    if x == 5 {
                        x = 0;
                        y += 1;
                    }
                    if y == 5 {
                        y = 0;
                        b += 1;
                        boards.push([[(0, false); 5]; 5]);
                    }
                    wait = true;
                }
            }
            _ if c.is_ascii_digit() => {
                boards[b][x][y].0 = (boards[b][x][y].0 * 10) + (c - 48);
                wait = false;
            }
            _ => panic!("invalid"),
        }
    }

    boards.pop();

    let mut first = true;
    let mut fboard = [[(0_u8, false); 5]; 5];
    let mut fdraw = &draws[0];

    let mut ldraw = &draws[0];

    'outer: for d in &draws {
        for n in boards.iter_mut().flatten().flatten() {
            if *d == n.0 {
                n.1 = true;
            }
        }

        let mut b = 0;
        let mut len = boards.len();
        while b < len {
            for x in 0..5 {
                let mut win = true;
                for y in 0..5 {
                    win = win && boards[b][x][y].1;
                }
                if win {
                    if first {
                        fboard = boards[b].clone();
                        fdraw = d;
                        first = false;
                    } else if boards.len() == 1 {
                        ldraw = d;
                        break 'outer;
                    }
                    boards.swap_remove(b);
                    len -= 1;
                    if b >= len {
                        b = 0;
                        continue;
                    }
                }
            }
            for y in 0..5 {
                let mut win = true;
                for x in 0..5 {
                    win = win && boards[b][x][y].1;
                }
                if win {
                    if first {
                        fboard = boards[b].clone();
                        fdraw = d;
                        first = false;
                    } else if boards.len() == 1 {
                        ldraw = d;
                        break 'outer;
                    }
                    boards.swap_remove(b);
                    len -= 1;
                }
            }
            b = if b >= len { 0 } else { b + 1 };
        }
    }

    let fsum = fboard
        .iter()
        .flatten()
        .filter(|e| !(e.1))
        .fold(0_u32, |a, e| a + e.0 as u32);

    let lsum = boards[0]
        .iter()
        .flatten()
        .filter(|e| !(e.1))
        .fold(0_u32, |a, e| a + e.0 as u32);

    println!("P1: {}", fsum * (*fdraw) as u32); // 35670
    println!("P2: {}", lsum * (*ldraw) as u32); // 22704

    // Correct!
}
