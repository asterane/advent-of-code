// Advent of Code 2015
// Day 21

#[derive(Debug)]
struct Contender {
    hp: u8,
    dmg: u8,
    arm: u8,
}

const WEAPONS: [(u8, u8, u8); 5] = [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
const ARMOR: [(u8, u8, u8); 5] = [(13, 0, 1), (31, 0, 2), (53, 0, 3), (75, 0, 4), (102, 0, 5)];
const RINGS: [(u8, u8, u8); 6] = [
    (25, 1, 0),
    (50, 2, 0),
    (100, 3, 0),
    (20, 0, 1),
    (40, 0, 2),
    (80, 0, 3),
];

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut player = Contender {
        hp: 100,
        dmg: 0,
        arm: 0,
    };

    let boss: Contender = {
        let mut ini = Contender {
            hp: 0,
            dmg: 0,
            arm: 0,
        };

        let mut acc = 0;
        let (mut hp, mut dmg) = (false, false);

        for c in input {
            match c {
                b'\n' => {
                    if hp && dmg {
                        ini.arm = acc
                    } else if hp {
                        ini.dmg = acc
                    } else {
                        ini.hp = acc
                    }

                    acc = 0;
                    dmg = hp;
                    hp = true;
                }
                _ if c.is_ascii_digit() => {
                    acc *= 10;
                    acc += c & 15;
                }
                _ if c.is_ascii() => (),
                _ => panic!(),
            }
        }

        ini
    };

    let mut min_win = u16::MAX;
    let mut max_lose = 0;

    for w in WEAPONS {
        for a in std::iter::once(&(0, 0, 0)).chain(ARMOR.iter()) {
            for (i, r1) in std::iter::once(&(0, 0, 0)).chain(RINGS.iter()).enumerate() {
                for r2 in std::iter::once(&(0, 0, 0)).chain(RINGS[i..].iter()) {
                    // println!("{w:?}, {a:?}, {r1:?}, {r2:?}");

                    player.dmg = w.1 + a.1 + r1.1 + r2.1;
                    player.arm = w.2 + a.2 + r1.2 + r2.2;

                    let gold = w.0 as u16 + a.0 as u16 + r1.0 as u16 + r2.0 as u16;

                    if battle(&player, &boss) {
                        min_win = gold.min(min_win)
                    } else {
                        max_lose = gold.max(max_lose)
                    }
                }
            }
        }
    }

    println!("Least gold spent to win: {}", min_win);  // 121
    println!("Most gold spent to lose: {}", max_lose); // 201

    // Correct!
}

fn battle(c1: &Contender, c2: &Contender) -> bool {
    let (mut hp1, mut hp2) = (c1.hp as i16, c2.hp as i16);
    let mut turnp = false;

    while hp1 > 0 && hp2 > 0 {
        turnp ^= true;

        if turnp {
            let stk = if c2.arm >= c1.dmg { 1 } else { c1.dmg - c2.arm };
            hp2 -= stk as i16
        } else {
            let stk = if c1.arm >= c2.dmg { 1 } else { c2.dmg - c1.arm };
            hp1 -= stk as i16
        }

        // println!("p: {hp1}, b: {hp2}");
    }

    if turnp {
        assert!(hp1 > 0)
    } else {
        assert!(hp2 > 0)
    }

    turnp
}
