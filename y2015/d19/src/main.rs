// Advent of Code 2015
// Day 19

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut rplc: Vec<(Box<[u8]>, Box<[u8]>)> = Vec::new();
    let mut acc: Vec<u8> = Vec::new();

    let mut cursor = input.iter().peekable();

    while cursor.peek() != Some(&&b'\n') {
        acc.push(*cursor.next().unwrap());
        if cursor.peek().unwrap().is_ascii_lowercase() {
            acc.push(*cursor.next().unwrap());
        }
        let r_from = acc.clone().into_boxed_slice();
        acc.clear();
        cursor.next();
        cursor.next();
        cursor.next();
        cursor.next();

        loop {
            match cursor.next().unwrap() {
                b'\n' => break,
                c if c.is_ascii_alphabetic() => acc.push(*c),
                _ => panic!(),
            }
        }

        let r_to = acc.clone().into_boxed_slice();
        acc.clear();

        rplc.push((r_from, r_to))
    }

    cursor.next();

    for c in cursor {
        acc.push(*c)
    }

    assert_eq!(acc.pop(), Some(10));

    let molecule = acc.into_boxed_slice();

    // println!("{}", String::from_utf8(molecule.to_vec()).unwrap());

    // for r in &rplc {
    //     println!(
    //         "{} => {}",
    //         String::from_utf8(r.0.to_vec()).unwrap(),
    //         String::from_utf8(r.1.to_vec()).unwrap()
    //     )
    // }

    let mut poss = gen_mol(&rplc, &molecule);

    poss.sort();
    poss.dedup();

    println!("Distinct molecules: {}", poss.len()); // 535

    let mut generations = 0;
    let mut test_mol = molecule;
    while test_mol != Box::from(&b"e"[..]) {
        let next_gen = degen_mol(&rplc, &test_mol);

        let min_len = next_gen
            .iter()
            .fold(usize::MAX, |a, m| if m.len() < a { m.len() } else { a });

        test_mol = next_gen.into_iter().find(|m| m.len() == min_len).unwrap();
        generations += 1;
    }

    println!("Steps to medicine: {}", generations); // 212

    // Correct!
}

fn degen_mol(caps: &[(Box<[u8]>, Box<[u8]>)], from_mol: &[u8]) -> Vec<Box<[u8]>> {
    let mut results = Vec::new();

    let mut idx = 0;
    while idx < from_mol.len() {
        assert!(from_mol[idx].is_ascii_uppercase());
        for rplc in caps {
            let tar = &rplc.1[..];
            if idx + tar.len() > from_mol.len() {
                continue;
            }
            if tar == &from_mol[idx..(idx + tar.len())] {
                let mut new_mol = from_mol.to_vec();
                for i in 0..tar.len() {
                    assert_eq!(new_mol.remove(idx), tar[i])
                }
                for (i, c) in rplc.0.iter().enumerate() {
                    new_mol.insert(idx + i, *c)
                }
                results.push(new_mol.into_boxed_slice())
            }
        }

        idx = if idx == from_mol.len() - 1 {
            idx + 1
        } else {
            match from_mol[idx + 1] {
                c if c.is_ascii_uppercase() => idx + 1,
                c if c.is_ascii_lowercase() => idx + 2,
                _ => panic!(),
            }
        };
    }

    results
}

fn gen_mol(caps: &[(Box<[u8]>, Box<[u8]>)], from_mol: &[u8]) -> Vec<Box<[u8]>> {
    let mut results = Vec::new();

    let mut lidx = 0;
    while lidx < from_mol.len() {
        let atom = {
            let start = lidx;
            assert!(from_mol[start].is_ascii_uppercase());
            let end = if lidx == from_mol.len() - 1 {
                lidx + 1
            } else {
                match from_mol[start + 1] {
                    c if c.is_ascii_uppercase() => start + 1,
                    c if c.is_ascii_lowercase() => start + 2,
                    _ => panic!(),
                }
            };
            lidx = end;
            &from_mol[start..end]
        };

        for rplc in caps {
            let tar = &rplc.0[..];
            if tar == atom {
                let mut new_mol = from_mol.to_vec();
                let atln = atom.len();
                let idx = lidx - atln;
                assert_eq!(new_mol.remove(idx), atom[0]);
                if atln > 1 {
                    assert_eq!(new_mol.remove(idx), atom[1]);
                }
                for (i, c) in rplc.1.iter().enumerate() {
                    new_mol.insert(idx + i, *c)
                }
                results.push(new_mol.into_boxed_slice())
            }
        }
    }

    results
}
