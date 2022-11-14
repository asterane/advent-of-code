// Advent of Code 2015
// Day 13

struct Guest {
    _id: u8,
    prefs: Vec<i8>,
}

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut guests: Vec<Guest> = Vec::new();

    let mut names: Vec<Box<[u8]>> = Vec::new();
    let mut name_acc: Vec<u8> = Vec::new();

    let mut happ_acc = 0;
    let mut gain_p = false;

    let mut gues_id = 0;
    let mut pref_id;

    for c in input {
        match c {
            b' ' if !name_acc.is_empty() => {
                gues_id = get_id(&mut names, &mut guests, &name_acc);
                name_acc.clear();
            }
            b'.' => {
                assert!(!name_acc.is_empty());
                pref_id = get_id(&mut names, &mut guests, &name_acc);

                let gues_prefs = &mut guests[gues_id].prefs;
                while pref_id >= gues_prefs.len() {
                    gues_prefs.push(0)
                }

                gues_prefs[pref_id] = if gain_p { happ_acc } else { -happ_acc };

                name_acc.clear();
                happ_acc = 0;
            }
            b'\n' | b' ' => (),
            _ if c.is_ascii_uppercase() => name_acc.push(*c),
            _ if c.is_ascii_lowercase() => {
                if !name_acc.is_empty() {
                    name_acc.push(*c)
                } else if happ_acc == 0 {
                    match c {
                        b'g' => gain_p = true,
                        b'l' => gain_p = false,
                        _ => (),
                    }
                }
            }
            _ if c.is_ascii_digit() => {
                happ_acc *= 10;
                happ_acc += (c & 15) as i8;
            }
            _ => panic!(),
        }
    }

    let n_host = optimize(&guests);

    let cur_len = guests.len();
    for g in guests.iter_mut() {
        while g.prefs.len() <= cur_len {
            g.prefs.push(0)
        }
    }

    guests.push(Guest {
        _id: cur_len as u8,
        prefs: vec![0; cur_len],
    });

    let w_host = optimize(&guests);

    println!("Optimal w/o host: {}", n_host); // 664
    println!("Optimal w/ host: {}", w_host);  // 640

    // Correct!
}

fn get_id(names: &mut Vec<Box<[u8]>>, guests: &mut Vec<Guest>, target: &Vec<u8>) -> usize {
    if !names.iter().any(|nm| nm[..] == target[..]) {
        let id = names.len() as u8;
        guests.push(Guest {
            _id: id,
            prefs: Vec::new(),
        });
        names.push(target.clone().into_boxed_slice());
        id as usize
    } else {
        names.iter().position(|nm| nm[..] == target[..]).unwrap()
    }
}

fn optimize(guests: &Vec<Guest>) -> i32 {
    let mut best = 0;
    let gues_ct = guests.len();

    let charts = permute(gues_ct as u8);
    for ch in charts {
        let mut acc = 0;
        for i in 0..gues_ct {
            let (lft, slf, rht) = (
                ch[if i == 0 { gues_ct - 1 } else { i - 1 }] as usize,
                ch[i] as usize,
                ch[if i == gues_ct - 1 { 0 } else { i + 1 }] as usize,
            );

            acc += guests[slf].prefs[lft] as i32;
            acc += guests[slf].prefs[rht] as i32;
        }
        if acc > best {
            // println!("{:?}", ch);
            best = acc
        }
    }
    best
}

// Adapted from Heap's algorithm code on Wikipedia
fn permute(rn: u8) -> Vec<Vec<u8>> {
    let n = rn - 1;

    let mut acc = Vec::new();
    let mut c = vec![0; n as usize];

    let mut p: Vec<u8> = (0..rn).collect();
    acc.push(p.clone());

    let mut i = 1;
    while i < n as usize {
        if c[i] < i {
            if i & 1 == 0 {
                p.swap(0 + 1, i + 1);
            } else {
                p.swap(c[i] + 1, i + 1);
            }
            acc.push(p.clone());
            c[i] += 1;
            i = 1;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    acc
}
