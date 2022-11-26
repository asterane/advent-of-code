// Advent of Code 2015
// Day 22

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
/// Discriminant is mana cost to cast spell
enum Spell {
    /// Does 4 damage
    MagicMissile = 53,
    /// Does 2 damage, heals 2 HP
    Drain = 73,
    /// Effect, 6 turns: increase armor by 7
    Shield = 113,
    /// Effect, 6 turns: deal 3 damage
    Poison = 173,
    /// Effect, 5 turns: add 101 mana
    Recharge = 229,
}

#[derive(Debug, Clone, PartialEq)]
enum Effect {
    Shield,
    Poison,
    Recharge,
}

#[derive(Debug, Clone)]
struct ActiveEffect {
    effect: Effect,
    turns_left: u8,
}

#[derive(Debug, Clone)]
struct Status {
    player_hp: i16,
    boss_hp: i16,

    player_armor: u8,
    boss_dmg: u8,

    player_mana: u16,
    mana_spent: u16,

    active_effects: Vec<ActiveEffect>,
}

fn avail_spells(status: &Status) -> Vec<Spell> {
    let mut spls = vec![Spell::MagicMissile, Spell::Drain];

    let (mut sh, mut pn, mut rc) = (true, true, true);

    for e in status.active_effects.iter() {
        if e.turns_left > 1 {
            match e.effect {
                Effect::Shield => sh = false,
                Effect::Poison => pn = false,
                Effect::Recharge => rc = false,
            };
        };
    }

    if sh {
        spls.push(Spell::Shield)
    }
    if pn {
        spls.push(Spell::Poison)
    }
    if rc {
        spls.push(Spell::Recharge)
    }

    spls.retain(|s| *s as u16 <= status.player_mana);

    spls
}

fn next_turn(status: &Status, spell: Spell, hard: bool) -> Status {
    let mut next = status.clone();

    if hard {
        next.player_hp -= 1;
        if next.player_hp <= 0 {
            return next;
        }
    }

    next.active_effects = next
        .active_effects
        .into_iter()
        .filter_map(|e| {
            match e.effect {
                Effect::Shield => next.player_armor = 7,
                Effect::Poison => next.boss_hp -= 3,
                Effect::Recharge => next.player_mana += 101,
            }
            if e.turns_left == 1 {
                if e.effect == Effect::Shield {
                    next.player_armor = 0;
                }
                return None;
            }
            Some(ActiveEffect {
                turns_left: e.turns_left - 1,
                ..e
            })
        })
        .collect();

    if next.boss_hp <= 0 {
        return next;
    }

    assert!(next.player_mana >= spell as u16);
    next.player_mana -= spell as u16;
    next.mana_spent += spell as u16;

    match spell {
        Spell::Recharge => {
            assert!(!next
                .active_effects
                .iter()
                .any(|e| e.effect == Effect::Recharge));
            next.active_effects.push(ActiveEffect {
                effect: Effect::Recharge,
                turns_left: 5,
            });
        }
        Spell::Poison => {
            assert!(!next
                .active_effects
                .iter()
                .any(|e| e.effect == Effect::Poison));
            next.active_effects.push(ActiveEffect {
                effect: Effect::Poison,
                turns_left: 6,
            });
        }
        Spell::Shield => {
            assert!(!next
                .active_effects
                .iter()
                .any(|e| e.effect == Effect::Shield));
            next.player_armor = 7;
            next.active_effects.push(ActiveEffect {
                effect: Effect::Shield,
                turns_left: 6,
            });
        }
        Spell::Drain => {
            next.boss_hp -= 2;
            next.player_hp += 2;
        }
        Spell::MagicMissile => {
            next.boss_hp -= 4;
        }
    }

    if next.boss_hp > 0 {
        next.active_effects = next
            .active_effects
            .into_iter()
            .filter_map(|e| {
                match e.effect {
                    Effect::Shield => next.player_armor = 7,
                    Effect::Poison => next.boss_hp -= 3,
                    Effect::Recharge => next.player_mana += 101,
                }
                if e.turns_left == 1 {
                    if e.effect == Effect::Shield {
                        next.player_armor = 0;
                    }
                    return None;
                }
                Some(ActiveEffect {
                    turns_left: e.turns_left - 1,
                    ..e
                })
            })
            .collect();

        if next.boss_hp <= 0 {
            return next;
        }

        let real_dmg = (next.boss_dmg - next.player_armor) as i16;
        next.player_hp -= real_dmg;
    }

    next
}

const BASE_HP: i16 = 50;
const BASE_MANA: u16 = 500;

fn main() {
    let input = std::include_bytes!("../input.txt");

    let (boss_hp, boss_dmg) = {
        let (mut hp, mut dmg) = (0, 0);
        let mut acc = 0;
        let mut hp_p = false;

        for c in input {
            match c {
                b'\n' => {
                    if hp_p {
                        dmg = acc
                    } else {
                        hp = acc
                    }
                    acc = 0;
                    hp_p = true;
                }
                _ if c.is_ascii_digit() => {
                    acc *= 10;
                    acc += c & 15;
                }
                _ if c.is_ascii() => (),
                _ => panic!(),
            }
        }

        (hp as i16, dmg)
    };

    let initial_status = Status {
        player_hp: BASE_HP,
        boss_hp,
        player_armor: 0,
        boss_dmg,
        player_mana: BASE_MANA,
        mana_spent: 0,
        active_effects: Vec::new(),
    };

    let mut active_fights = vec![initial_status.clone()];
    let mut min_mana = u16::MAX;

    while !active_fights.is_empty() {
        active_fights = active_fights
            .into_iter()
            .filter_map(|f| {
                if f.mana_spent >= min_mana || (f.player_hp <= 0 && f.boss_hp > 0) {
                    None
                } else if f.player_hp > 0 && f.boss_hp <= 0 {
                    min_mana = f.mana_spent.min(min_mana);
                    None
                } else {
                    assert!(f.boss_hp > 0 && f.player_hp > 0);
                    // println!("{f:?}");
                    let mut acc = Vec::new();
                    let poss = avail_spells(&f);
                    // println!("{poss:?}");
                    for s in poss {
                        acc.push(next_turn(&f, s, false))
                    }
                    Some(acc.into_iter())
                }
            })
            .flatten()
            .collect();

        // println!("{}", active_fights.len());
    }

    let mut active_fights_hard = vec![initial_status];
    let mut min_mana_hard = u16::MAX;

    while !active_fights_hard.is_empty() {
        active_fights_hard = active_fights_hard
            .into_iter()
            .filter_map(|f| {
                if f.mana_spent >= min_mana_hard || (f.player_hp <= 0 && f.boss_hp > 0) {
                    None
                } else if f.player_hp > 0 && f.boss_hp <= 0 {
                    min_mana_hard = f.mana_spent.min(min_mana_hard);
                    None
                } else {
                    assert!(f.boss_hp > 0 && f.player_hp > 0);
                    // println!("{f:?}");
                    let mut acc = Vec::new();
                    let poss = avail_spells(&f);
                    // println!("{poss:?}");
                    for s in poss {
                        acc.push(next_turn(&f, s, true))
                    }
                    Some(acc.into_iter())
                }
            })
            .flatten()
            .collect();

        // println!("{}", active_fights_hard.len());
    }

    println!("Least mana to win: {}", min_mana);            // 953
    println!("Least mana to win; hard: {}", min_mana_hard); // 1289

    // Correct!
}
