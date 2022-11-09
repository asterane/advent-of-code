// Advent of Code 2015
// Day 7

type Val = u16;
type Name = (u8, u8);
type Id = u16;

#[derive(Debug, PartialEq)]
struct Wire {
    name: Name,
    val: Option<Val>,
}

#[derive(Debug, Clone)]
enum Sig {
    Val(Val),
    Wire(Id),
}

impl Sig {
    fn val(&self, net: &Vec<Wire>) -> Val {
        match self {
            Sig::Val(v) => *v,
            Sig::Wire(id) => net[*id as usize].val.unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
enum Gate {
    Asn(Sig),
    Not(Sig),
    And(Sig, Sig),
    Or(Sig, Sig),
    Lhs(Sig, Sig),
    Rhs(Sig, Sig),
}

#[derive(Debug, Clone)]
struct Elt {
    gate: Gate,
    tgt: Id,
}

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut nodes: Vec<Wire> = Vec::new();
    let mut comp1: Vec<Elt> = Vec::new();

    let mut cursor = input.iter().peekable();
    while let Some(elt) = read_elt(&mut cursor, &mut nodes) {
        comp1.push(elt);
    }

    let mut comp2 = comp1.clone();

    // println!("{:#?}", comps);

    simulate(comp1, &mut nodes);

    // println!("{:#?}", nodes);

    let a_sig_1 = nodes
        .iter()
        .find(|w| w.name == (b'a', 0))
        .unwrap()
        .val
        .unwrap();

    println!("First signal on wire [a]: {}", a_sig_1); // 956

    for w in nodes.iter_mut() {
        w.val = if w.name == (b'b', 0) {
            Some(a_sig_1)
        } else {
            None
        };
    }

    comp2.remove(
        comp2
            .iter()
            .position(|elt| {
                elt.tgt as usize == nodes.iter().position(|w| w.name == (b'b', 0)).unwrap()
            })
            .unwrap(),
    );

    simulate(comp2, &mut nodes);

    let a_sig_2 = nodes
        .iter()
        .find(|w| w.name == (b'a', 0))
        .unwrap()
        .val
        .unwrap();

    println!("Second signal on wire [a]: {}", a_sig_2); // 40149

    // Correct!
}

fn simulate(mut cmps: Vec<Elt>, net: &mut Vec<Wire>) {
    while cmps.len() > 0 {
        // println!("{}", cmps.len());
        cmps = cmps
            .into_iter()
            .filter_map(|elt| {
                if active_p(&elt.gate, &net) {
                    net[elt.tgt as usize].val = Some(match elt.gate {
                        Gate::Asn(s) => s.val(&net),
                        Gate::Not(s) => !s.val(&net),
                        Gate::And(l, r) => l.val(&net) & r.val(&net),
                        Gate::Or(l, r) => l.val(&net) | r.val(&net),
                        Gate::Lhs(l, r) => l.val(&net) << r.val(&net),
                        Gate::Rhs(l, r) => l.val(&net) >> r.val(&net),
                    });
                    None
                } else {
                    Some(elt)
                }
            })
            .collect();
    }
}

fn live_p(sig: &Sig, net: &Vec<Wire>) -> bool {
    match sig {
        Sig::Val(_) => true,
        Sig::Wire(id) => net[*id as usize].val.is_some(),
    }
}

fn active_p(gate: &Gate, net: &Vec<Wire>) -> bool {
    match gate {
        Gate::Asn(s) | Gate::Not(s) => live_p(s, net),
        Gate::And(l, r) | Gate::Or(l, r) | Gate::Lhs(l, r) | Gate::Rhs(l, r) => {
            live_p(l, net) && live_p(r, net)
        }
    }
}

fn read_elt(
    cur: &mut std::iter::Peekable<std::slice::Iter<u8>>,
    net: &mut Vec<Wire>,
) -> Option<Elt> {
    let gate = match cur.peek() {
        Some(b'N') => Gate::Not(read_sig(cur, net)),
        Some(c) if c.is_ascii_digit() || c.is_ascii_lowercase() => {
            let sig_a = read_sig(cur, net);
            match cur.next() {
                Some(b'-') => Gate::Asn(sig_a),
                Some(b'A') => Gate::And(sig_a, read_sig(cur, net)),
                Some(b'O') => Gate::Or(sig_a, read_sig(cur, net)),
                Some(b'L') => Gate::Lhs(sig_a, read_sig(cur, net)),
                Some(b'R') => Gate::Rhs(sig_a, read_sig(cur, net)),
                _ => panic!(),
            }
        }
        Some(_) => panic!(),
        None => return None,
    };

    let tgt = if let Sig::Wire(id) = read_sig(cur, net) {
        id
    } else {
        panic!()
    };

    Some(Elt { gate, tgt })
}

fn read_sig(cur: &mut std::iter::Peekable<std::slice::Iter<u8>>, net: &mut Vec<Wire>) -> Sig {
    let (mut val_acc, mut name_acc) = (None, None);
    loop {
        match cur.next() {
            Some(b' ' | b'\n') => {
                if !(val_acc == None && name_acc == None) {
                    break;
                }
            }
            Some(c) if c.is_ascii_digit() => {
                if val_acc == None {
                    val_acc = Some(0);
                }
                val_acc = Some((val_acc.unwrap() * 10) + (c & 15) as Val);
            }
            Some(c) if c.is_ascii_lowercase() => {
                if name_acc == None {
                    name_acc = Some((0, 0));
                }
                name_acc = Some((*c, name_acc.unwrap().0));
            }
            Some(c) if c.is_ascii() => (),
            _ => panic!(),
        }
    }

    assert!((val_acc == None) ^ (name_acc == None));

    if val_acc.is_some() {
        Sig::Val(val_acc.unwrap())
    } else {
        let wire = Wire {
            name: name_acc.unwrap(),
            val: None,
        };

        let id = match net.iter().position(|elt| *elt == wire) {
            Some(idx) => idx,
            None => {
                net.push(wire);
                net.len() - 1
            }
        } as Id;

        Sig::Wire(id)
    }
}
