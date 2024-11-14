// Advent of Code 2016
// Day 10: Balance Bots

#[derive(Clone, Debug)]
enum Tgt {
    Bot(usize),
    Out(usize),
}

#[derive(Clone, Debug)]
struct BotInst {
    lowtgt: Tgt,
    hitgt: Tgt,
}

fn main() {
    let input = include_bytes!("../input.txt");

    let mut bot_inst_list: Vec<Option<BotInst>> = vec![];
    let mut chips: Vec<(Option<usize>, Option<usize>)> = vec![];

    let mut outputs: Vec<usize> = vec![];

    let mut bytes = input.iter();
    loop {
        let op;
        match bytes.next() {
            Some(b'b') => {
                op = false;
                bytes.nth(1);
            }
            Some(b'v') => {
                op = true;
                bytes.nth(3);
            }
            Some(_) => panic!(),
            None => break,
        }

        assert_eq!(b' ', *bytes.next().unwrap());

        let mut acc = 0;
        loop {
            match bytes.next().unwrap() {
                b if b.is_ascii_whitespace() => break,
                b if b.is_ascii_digit() => acc = 10 * acc + (b & 0xF) as usize,
                _ => panic!(),
            }
        }

        if op {
            let val = acc;

            assert_eq!(b' ', *bytes.nth(11).unwrap());

            acc = 0;
            loop {
                match bytes.next().unwrap() {
                    b if b.is_ascii_whitespace() => break,
                    b if b.is_ascii_digit() => acc = 10 * acc + (b & 0xF) as usize,
                    _ => panic!(),
                }
            }

            let bot = acc;

            if bot_inst_list.len() <= bot {
                bot_inst_list.resize(bot + 1, None);
                chips.resize(bot + 1, (None, None));
            }

            if chips[bot].0.is_some() {
                assert!(chips[bot].1.is_none());
                chips[bot].1 = Some(val);
            } else {
                chips[bot].0 = Some(val);
            }
        } else {
            let bot = acc;

            assert_eq!(b' ', *bytes.nth(12).unwrap());

            let outp;
            match bytes.next().unwrap() {
                b'o' => {
                    outp = true;
                    bytes.nth(4);
                }
                b'b' => {
                    outp = false;
                    bytes.nth(1);
                }
                _ => panic!(),
            }

            assert_eq!(b' ', *bytes.next().unwrap());

            acc = 0;
            loop {
                match bytes.next().unwrap() {
                    b if b.is_ascii_whitespace() => break,
                    b if b.is_ascii_digit() => acc = 10 * acc + (b & 0xF) as usize,
                    b => panic!("{}", *b as char),
                }
            }

            let lowtgt = if outp { Tgt::Out(acc) } else { Tgt::Bot(acc) };

            assert_eq!(b' ', *bytes.nth(11).unwrap());

            let outp;
            match bytes.next().unwrap() {
                b'o' => {
                    outp = true;
                    bytes.nth(4);
                }
                b'b' => {
                    outp = false;
                    bytes.nth(1);
                }
                _ => panic!(),
            }

            assert_eq!(b' ', *bytes.next().unwrap());

            acc = 0;
            loop {
                match bytes.next().unwrap() {
                    b if b.is_ascii_whitespace() => break,
                    b if b.is_ascii_digit() => acc = 10 * acc + (b & 0xF) as usize,
                    _ => panic!(),
                }
            }

            let hitgt = if outp { Tgt::Out(acc) } else { Tgt::Bot(acc) };

            if bot_inst_list.len() <= bot {
                bot_inst_list.resize(bot + 1, None);
                chips.resize(bot + 1, (None, None));
            }

            bot_inst_list[bot] = Some(BotInst { lowtgt, hitgt });
        }
    }

    let bots_final: Vec<BotInst> = bot_inst_list.into_iter().map(|e| e.unwrap()).collect();
    // println!("{bots_final:?}");

    let mut bot_61_17_compare = None;

    let mut inflight: Vec<(usize, Tgt)> = vec![];
    loop {
        for (i, e) in chips.iter_mut().enumerate() {
            if let (Some(l), Some(r)) = e {
                let (low, hi) = if l > r { (*r, *l) } else { (*l, *r) };

                if low == 17 && hi == 61 {
                    bot_61_17_compare = Some(i);
                }

                inflight.push((low, bots_final[i].lowtgt.clone()));
                inflight.push((hi, bots_final[i].hitgt.clone()));

                e.0 = None;
                e.1 = None;
            }
        }

        if inflight.is_empty() {
            break;
        }

        for o in inflight.drain(..) {
            match o.1 {
                Tgt::Out(to) => {
                    if outputs.len() <= to {
                        outputs.resize(to + 1, 0);
                    }
                    outputs[to] = o.0;
                }
                Tgt::Bot(tb) => {
                    if chips[tb].0.is_some() {
                        assert!(chips[tb].1.is_none());
                        chips[tb].1 = Some(o.0);
                    } else {
                        chips[tb].0 = Some(o.0);
                    }
                }
            }
        }
    }

    // println!("{outputs:?}");

    println!(
        "Bot which compares val-61 chips with val-17 chips: {}",
        bot_61_17_compare.unwrap()
    ); // 98

    let out_0_1_2_product = outputs[0] * outputs[1] * outputs[2];

    println!(
        "Product of chip values in outputs 0 thru 2: {}",
        out_0_1_2_product
    ); // 4042
}
