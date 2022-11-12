// Advent of Code 2015
// Day 9

/// Stores city name
type Node = Box<[u8]>;

#[derive(Debug)]
struct Edge {
    a: u8,
    b: u8,
    dist: u16,
}

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut nodes: Vec<Node> = Vec::new();
    let mut edges: Vec<Edge> = Vec::new();

    let (mut acc_a, mut acc_b) = (Vec::new(), Vec::new());
    let mut dist = 0;
    let mut in_name = false;
    for c in input {
        match c {
            b'\n' => {
                // println!(
                //     "{}, {} -> {}",
                //     String::from_utf8(acc_a.clone()).unwrap(),
                //     String::from_utf8(acc_b.clone()).unwrap(),
                //     dist
                // );
                if !nodes.iter().any(|elt| elt[..] == acc_a[..]) {
                    nodes.push(acc_a.clone().into_boxed_slice());
                }
                if !nodes.iter().any(|elt| elt[..] == acc_b[..]) {
                    nodes.push(acc_b.clone().into_boxed_slice());
                }
                edges.push(Edge {
                    a: nodes.iter().position(|elt| elt[..] == acc_a[..]).unwrap() as u8,
                    b: nodes.iter().position(|elt| elt[..] == acc_b[..]).unwrap() as u8,
                    dist,
                });
                acc_a.clear();
                acc_b.clear();
                dist = 0;
            }
            b'=' => (),
            b' ' => in_name = false,
            c if c.is_ascii_digit() => {
                assert!(!acc_b.is_empty());
                dist *= 10;
                dist += (c & 15) as u16;
            }
            c if c.is_ascii_lowercase() => {
                if !in_name {
                    continue;
                }
                if acc_b.is_empty() {
                    acc_a.push(*c)
                } else {
                    acc_b.push(*c)
                }
            }
            c if c.is_ascii_uppercase() => {
                assert!(acc_b.is_empty());
                if in_name || acc_a.is_empty() {
                    acc_a.push(*c);
                } else {
                    acc_b.push(*c);
                }
                in_name = true;
            }
            _ => panic!(),
        }
    }

    // println!("{:?}", edges);

    let mut short = u16::MAX;
    let mut long = 0;

    let routes = permute(nodes.len() as u8);
    for r in routes.iter() {
        let mut acc = 0;
        for pair in r.windows(2) {
            acc += edges
                .iter()
                .find(|eg| {
                    (eg.a == pair[0] && eg.b == pair[1]) || (eg.a == pair[1] && eg.b == pair[0])
                })
                .unwrap()
                .dist
        }
        if acc < short {
            short = acc;
        }
        if acc > long {
            long = acc;
        }
    }

    println!("Shortest route length: {}", short); // 141
    println!("Longest route length: {}", long);   // 736

    // Correct!
}

// Adapted from Heap's algorithm code on Wikipedia
fn permute(n: u8) -> Vec<Vec<u8>> {
    let mut acc = Vec::new();

    let mut c = vec![0; n as usize];

    let mut p: Vec<u8> = (0..n).collect();
    acc.push(p.clone());

    let mut i = 1;
    while i < n as usize {
        if c[i] < i {
            if i & 1 == 0 {
                p.swap(0, i);
            } else {
                p.swap(c[i], i);
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
