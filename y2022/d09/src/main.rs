// Advent of Code 2022
// Day 9: Rope Bridge

use std::collections::HashSet;
use std::ops::{Add, Sub};

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut moves: Vec<(Dir, u8)> = Vec::new();

    for c in input {
        match c {
            b'\n' | b' ' => (),
            b'U' => moves.push((Dir::Up, 0)),
            b'D' => moves.push((Dir::Down, 0)),
            b'L' => moves.push((Dir::Left, 0)),
            b'R' => moves.push((Dir::Right, 0)),
            _ if c.is_ascii_digit() => {
                let cur = &mut (moves.last_mut().unwrap().1);
                *cur *= 10;
                *cur += c & 15;
            }
            _ => panic!(),
        }
    }

    let mut short_rope = [[0; 2]; 2];
    let mut long_rope = [[0; 2]; 10];

    let mut visited_short: HashSet<[i16; 2]> = HashSet::new();
    visited_short.insert(short_rope[1]);

    let mut visited_long: HashSet<[i16; 2]> = HashSet::new();
    visited_long.insert(long_rope[9]);

    // _dbg_scanout(&short_rope, [0, 0], [6, 5]);
    // _dbg_scanout(&long_rope, [0, 0], [6, 5]);
    // _dbg_scanout(&long_rope, [-11, -5], [15, 16]);

    for m in moves {
        let mag = m.1 as i16;
        let (i, f): (usize, fn(i16, i16) -> i16) = match m.0 {
            Dir::Up => (1, i16::add),
            Dir::Down => (1, i16::sub),
            Dir::Left => (0, i16::sub),
            Dir::Right => (0, i16::add),
        };
        for _ in 0..mag {
            short_rope[0][i] = f(short_rope[0][i], 1);
            long_rope[0][i] = f(long_rope[0][i], 1);

            let tpps = short_rope[0];
            rope_seg_sim(&tpps, &mut short_rope[1]);

            for i in 1..long_rope.len() {
                let tppl = long_rope[i - 1];
                rope_seg_sim(&tppl, &mut long_rope[i]);
            }

            // _dbg_scanout(&short_rope, [0, 0], [6, 5]);
            // _dbg_scanout(&long_rope, [0, 0], [6, 5]);

            visited_short.insert(short_rope[1]);
            visited_long.insert(long_rope[9]);
        }
        // _dbg_scanout(&long_rope, [-11, -5], [15, 16]);
    }

    // _dbg_showvis(&visited_short, [0, 0], [6, 5]);
    // _dbg_showvis(&visited_long, [-11, -5], [15, 16]);

    // Part 1
    println!("Len 2; distinct tail positions: {}", visited_short.len()); // 6175

    // Part 2
    println!("Len 10; distinct tail positions: {}", visited_long.len()); // 2578

    // Correct!
}

fn rope_seg_sim(prec: &[i16; 2], foll: &mut [i16; 2]) {
    let dist = [prec[0] - foll[0], prec[1] - foll[1]];
    *foll = match dist {
        [-1..=1, -1..=1] => *foll,
        [-2, 0] => [foll[0] - 1, foll[1]],
        [0, -2] => [foll[0], foll[1] - 1],
        [2, 0] => [foll[0] + 1, foll[1]],
        [0, 2] => [foll[0], foll[1] + 1],
        [-2, -1 | 1] => [foll[0] - 1, prec[1]],
        [-1 | 1, -2] => [prec[0], foll[1] - 1],
        [2, -1 | 1] => [foll[0] + 1, prec[1]],
        [-1 | 1, 2] => [prec[0], foll[1] + 1],
        [-2, -2] => [foll[0] - 1, foll[1] - 1],
        [2, -2] => [foll[0] + 1, foll[1] - 1],
        [-2, 2] => [foll[0] - 1, foll[1] + 1],
        [2, 2] => [foll[0] + 1, foll[1] + 1],
        _ => panic!(),
    }
}

fn _dbg_scanout(rope: &[[i16; 2]], gmin: [i16; 2], gmax: [i16; 2]) {
    for y in (gmin[1]..gmax[1]).rev() {
        'celli: for x in gmin[0]..gmax[0] {
            if rope[0] == [x, y] {
                print!("H");
                continue 'celli;
            }
            for i in 1..rope.len() {
                if rope[i] == [x, y] {
                    print!("{i}");
                    continue 'celli;
                }
            }
            print!(".");
        }
        println!();
    }
    println!();
}

fn _dbg_showvis(vis: &HashSet<[i16; 2]>, gmin: [i16; 2], gmax: [i16; 2]) {
    for y in (gmin[1]..gmax[1]).rev() {
        for x in gmin[0]..gmax[0] {
            if [x, y] == [0, 0] {
                print!("s")
            } else if vis.contains(&[x, y]) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    }
    println!();
}
