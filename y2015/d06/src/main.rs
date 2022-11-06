// Advent of Code 2015
// Day 6

#[derive(Debug)]
enum Action {
    On,
    Off,
    Tog,
}

#[derive(Debug)]
struct Rect {
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16,
}

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut grid_orig = [[false; 1000]; 1000];
    let mut grid_nord = [[0u16; 1000]; 1000];

    let mut cursor = input.iter();
    loop {
        match cursor.next() {
            Some(b't') => (),
            Some(_) => panic!(),
            None => break,
        }

        let inst = (
            match cursor.next() {
                Some(b'o') => Action::Tog,
                Some(b'u') => {
                    for _ in 0..4 {
                        cursor.next();
                    }

                    match cursor.next() {
                        Some(b'n') => Action::On,
                        Some(b'f') => Action::Off,
                        _ => panic!(),
                    }
                }
                _ => panic!(),
            },
            Rect {
                x1: pull_num(&mut cursor, b','),
                y1: pull_num(&mut cursor, b' '),
                x2: pull_num(&mut cursor, b','),
                y2: pull_num(&mut cursor, b'\n'),
            },
        );

        // println!("{:?}", inst);

        apply_inst_orig(&mut grid_orig, &inst);
        apply_inst_nord(&mut grid_nord, &inst);
    }

    let count = count_lit(&grid_orig);
    let intens = brightness(&grid_nord);

    println!("Lit lights: {}", count); // 569999
    println!("Intensity: {}", intens); // 17836115

    // Correct!
}

fn pull_num(cursor: &mut std::slice::Iter<u8>, delim: u8) -> u16 {
    let mut acc = 0;
    loop {
        match cursor.next() {
            Some(b) if *b == delim => break,
            Some(b) if b.is_ascii_digit() => {
                acc *= 10;
                acc += *b as u16 & 15;
            }
            Some(_) => continue,
            None => panic!(),
        }
    }
    acc
}

fn apply_inst_orig(grid: &mut [[bool; 1000]; 1000], inst: &(Action, Rect)) {
    for x in inst.1.x1..=inst.1.x2 {
        for y in inst.1.y1..=inst.1.y2 {
            let (x, y) = (x as usize, y as usize);
            match inst.0 {
                Action::On => grid[x][y] = true,
                Action::Off => grid[x][y] = false,
                Action::Tog => grid[x][y] ^= true,
            }
        }
    }
}

fn apply_inst_nord(grid: &mut [[u16; 1000]; 1000], inst: &(Action, Rect)) {
    for x in inst.1.x1..=inst.1.x2 {
        for y in inst.1.y1..=inst.1.y2 {
            let (x, y) = (x as usize, y as usize);
            match inst.0 {
                Action::On => grid[x][y] += 1,
                Action::Off => grid[x][y] = grid[x][y].saturating_sub(1),
                Action::Tog => grid[x][y] += 2,
            }
        }
    }
}

fn count_lit(grid: &[[bool; 1000]; 1000]) -> u32 {
    let mut acc = 0;
    for &x in grid.iter() {
        for &elt in x.iter() {
            acc += elt as u32;
        }
    }
    acc
}

fn brightness(grid: &[[u16; 1000]; 1000]) -> u32 {
    let mut acc = 0;
    for &x in grid.iter() {
        for &elt in x.iter() {
            acc += elt as u32;
        }
    }
    acc
}

#[test]
fn inst_orig_1() {
    let mut grid = [[true; 1000]; 1000];
    apply_inst_orig(
        &mut grid,
        &(
            Action::Off,
            Rect {
                x1: 499,
                y1: 499,
                x2: 500,
                y2: 500,
            },
        ),
    );
    assert_eq!(count_lit(&grid), 1000000 - 4);
}

#[test]
fn inst_orig_3() {
    let mut grid = [[false; 1000]; 1000];
    apply_inst_orig(
        &mut grid,
        &(
            Action::On,
            Rect {
                x1: 0,
                y1: 0,
                x2: 999,
                y2: 999,
            },
        ),
    );
    apply_inst_orig(
        &mut grid,
        &(
            Action::Tog,
            Rect {
                x1: 0,
                y1: 0,
                x2: 999,
                y2: 0,
            },
        ),
    );
    apply_inst_orig(
        &mut grid,
        &(
            Action::Off,
            Rect {
                x1: 499,
                y1: 499,
                x2: 500,
                y2: 500,
            },
        ),
    );
    assert_eq!(count_lit(&grid), 1000000 - 1000 - 4);
}

#[test]
fn inst_nord_2() {
    let mut grid = [[0u16; 1000]; 1000];
    apply_inst_nord(
        &mut grid,
        &(
            Action::On,
            Rect {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0,
            },
        ),
    );
    apply_inst_nord(
        &mut grid,
        &(
            Action::Tog,
            Rect {
                x1: 0,
                y1: 0,
                x2: 999,
                y2: 999,
            },
        ),
    );
    assert_eq!(brightness(&grid), 2000000 + 1);
}
