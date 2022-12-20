// Advent of Code 2022
// Day 11: Monkey in the Middle

struct Op(Box<dyn Fn(u64) -> u64>);

use std::fmt::Debug;
impl Debug for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<operation>")
    }
}

#[derive(Debug)]
struct Monkey {
    op: Op,
    test_div: u64,
    truep: u8,
    falsep: u8,
}

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut item_levels: Vec<u64> = Vec::new();
    let mut monkey_items: Vec<Vec<u8>> = Vec::new();

    let mut cur = input.iter();
    loop {
        match cur.next() {
            Some(b'M' | b'\n') => (),
            Some(_) => panic!(),
            None => break,
        }

        while cur.next().unwrap() != &b' ' {}
        assert_eq!((cur.next().unwrap() & 15) as usize, monkeys.len());
        cur.next();

        while cur.next().unwrap() != &b':' {}

        let mut items_held = Vec::new();
        let mut hacc = 0;
        loop {
            match cur.next().unwrap() {
                b'\n' => {
                    items_held.push(item_levels.len() as u8);
                    item_levels.push(hacc);
                    break;
                }
                b' ' => (),
                b',' => {
                    items_held.push(item_levels.len() as u8);
                    item_levels.push(hacc);
                    hacc = 0;
                }
                c @ _ if c.is_ascii_digit() => {
                    hacc *= 10;
                    hacc += (*c & 15) as u64;
                }
                _ => panic!(),
            }
        }

        while cur.next().unwrap() != &b'd' {}
        cur.next();

        let opf = match cur.next().unwrap() {
            b'+' => false,
            b'*' => true,
            _ => panic!(),
        };

        cur.next();

        let op = match cur.next().unwrap() {
            b'o' => {
                cur.next();
                cur.next();
                assert_eq!(cur.next().unwrap(), &b'\n');

                Box::new(if opf {
                    fn m(old: u64) -> u64 {
                        old * old
                    }
                    m
                } else {
                    fn a(old: u64) -> u64 {
                        old + old
                    }
                    a
                })
            }
            c @ _ if c.is_ascii_digit() => {
                let mut oacc = (*c & 15) as u64;
                loop {
                    let next = cur.next().unwrap();
                    if next.is_ascii_digit() {
                        oacc *= 10;
                        oacc += (*next & 15) as u64;
                    } else if next == &b'\n' {
                        break;
                    } else {
                        panic!();
                    }
                }

                if opf {
                    Box::new(move |old| old * oacc) as Box<dyn Fn(u64) -> u64>
                } else {
                    Box::new(move |old| old + oacc) as Box<dyn Fn(u64) -> u64>
                }
            }
            _ => panic!(),
        };

        while cur.next().unwrap() != &b'y' {}
        cur.next();

        let test_div = {
            let mut tacc = 0;
            loop {
                let next = cur.next().unwrap();
                if next.is_ascii_digit() {
                    tacc *= 10;
                    tacc += (*next & 15) as u64;
                } else if next == &b'\n' {
                    break;
                } else {
                    panic!();
                }
            }
            tacc
        };

        while cur.next().unwrap() != &b'y' {}
        cur.next();
        let truep = cur.next().unwrap() & 15;

        while cur.next().unwrap() != &b'y' {}
        cur.next();
        let falsep = cur.next().unwrap() & 15;

        assert_eq!(cur.next().unwrap(), &b'\n');

        monkeys.push(Monkey {
            op: Op(op),
            test_div,
            truep,
            falsep,
        });

        monkey_items.push(items_held);
    }

    // println!("{items:?}");
    // println!("{monkeys:?}");

    let mut m_items_low = monkey_items.clone();
    let mut m_items_high = monkey_items;

    let mut item_lvls_low = item_levels.clone();
    let mut item_lvls_high = item_levels;

    let mut inspected_low = vec![0; monkeys.len()];
    let mut inspected_high = vec![0; monkeys.len()];

    let common = monkeys.iter().fold(1, |a, m| a * m.test_div);

    for _ in 0..20 {
        run_round(
            &mut monkeys,
            &mut m_items_low,
            &mut item_lvls_low,
            &mut inspected_low,
            None,
        )
    }

    for _ in 0..10000 {
        run_round(
            &mut monkeys,
            &mut m_items_high,
            &mut item_lvls_high,
            &mut inspected_high,
            Some(common),
        )
    }

    inspected_low.sort();
    inspected_high.sort();

    let monkey_business_low = inspected_low.pop().unwrap() * inspected_low.pop().unwrap();
    let monkey_business_high = inspected_high.pop().unwrap() * inspected_high.pop().unwrap();

    // Part 1
    println!("Monkey business after 20 rounds: {}", monkey_business_low); // 117624

    // Part 2
    println!(
        "Monkey business after 10000 rounds: {}",
        monkey_business_high
    ); // 16792940265

    // Correct!
}

fn run_round(
    monkeys: &mut Vec<Monkey>,
    items: &mut Vec<Vec<u8>>,
    levels: &mut Vec<u64>,
    inspects: &mut Vec<u64>,
    worryf: Option<u64>,
) {
    for (i, m) in monkeys.iter().enumerate() {
        for j in 0..items[i].len() {
            let k = items[i][j] as usize;
            levels[k] = (m.op.0)(levels[k]);

            match worryf {
                Some(f) => levels[k] %= f,
                None => levels[k] /= 3,
            }

            if levels[k] % m.test_div == 0 {
                &mut items[m.truep as usize]
            } else {
                &mut items[m.falsep as usize]
            }
            .push(k as u8);

            inspects[i] += 1;
        }
        items[i].clear();
    }
}
