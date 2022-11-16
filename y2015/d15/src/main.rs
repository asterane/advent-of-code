// Advent of Code 2015
// Day 15

#[derive(Debug)]
struct Ingredient {
    capacity: i8,
    durability: i8,
    flavor: i8,
    texture: i8,
    calories: i8,
}

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut neg = false;
    let mut acc = 0;
    let mut ind = 0;

    let mut cur = Ingredient {
        capacity: 0,
        durability: 0,
        flavor: 0,
        texture: 0,
        calories: 0,
    };

    let mut ingredients: Vec<Ingredient> = Vec::new();

    for c in input {
        match c {
            b'\n' => {
                if neg {
                    acc = -acc
                }
                match ind {
                    0 => cur.capacity = acc,
                    1 => cur.durability = acc,
                    2 => cur.flavor = acc,
                    3 => cur.texture = acc,
                    4 => cur.calories = acc,
                    _ => panic!(),
                }

                ingredients.push(cur);
                cur = Ingredient {
                    capacity: 0,
                    durability: 0,
                    flavor: 0,
                    texture: 0,
                    calories: 0,
                };

                neg = false;
                acc = 0;
                ind = 0;
            }
            b',' => {
                if neg {
                    acc = -acc
                }
                match ind {
                    0 => cur.capacity = acc,
                    1 => cur.durability = acc,
                    2 => cur.flavor = acc,
                    3 => cur.texture = acc,
                    4 => cur.calories = acc,
                    _ => panic!(),
                }

                neg = false;
                acc = 0;
                ind += 1;
            }
            b'-' => neg = true,
            _ if c.is_ascii_digit() => {
                acc *= 10;
                acc += (c & 15) as i8;
            }
            _ if c.is_ascii() => (),
            _ => panic!(),
        }
    }

    // println!("{:?}", ingredients);

    let best = top_cookie_score(&ingredients);
    let b_500 = cal_cookie_score(&ingredients);

    println!("Highest score: {}", best);    // 13882464
    println!("500-cal highest: {}", b_500); // 11171160

    // Correct!
}

fn top_cookie_score(ing_list: &[Ingredient]) -> i32 {
    let options = tsp_gen(ing_list.len());

    let mut best = 0;
    for o in options {
        let (mut c, mut d, mut f, mut t) = (0, 0, 0, 0);
        for (i, ing) in ing_list.iter().enumerate() {
            c += o[i] as i32 * ing.capacity as i32;
            d += o[i] as i32 * ing.durability as i32;
            f += o[i] as i32 * ing.flavor as i32;
            t += o[i] as i32 * ing.texture as i32;
        }
        (c, d, f, t) = (c.max(0), d.max(0), f.max(0), t.max(0));
        let score = c * d * f * t;
        if score > best {
            best = score
        }
    }

    best
}

fn cal_cookie_score(ing_list: &[Ingredient]) -> i32 {
    const CALS: i32 = 500;

    let options = tsp_gen(ing_list.len());

    let mut best_c = 0;
    for o in options {
        let (mut c, mut d, mut f, mut t) = (0, 0, 0, 0);
        let mut cals = 0;
        for (i, ing) in ing_list.iter().enumerate() {
            c += o[i] as i32 * ing.capacity as i32;
            d += o[i] as i32 * ing.durability as i32;
            f += o[i] as i32 * ing.flavor as i32;
            t += o[i] as i32 * ing.texture as i32;
            cals += o[i] as i32 * ing.calories as i32;
        }
        (c, d, f, t) = (c.max(0), d.max(0), f.max(0), t.max(0));
        let score = c * d * f * t;
        if cals == CALS && score > best_c {
            best_c = score
        }
    }

    best_c
}

fn tsp_gen(len: usize) -> Vec<Vec<i8>> {
    const TEASPOONS: i8 = 100;

    let mut sels = Vec::new();

    fn tsp_layer(ct: usize, accs: &[i8], opts: &mut Vec<Vec<i8>>) {
        if ct - 1 == accs.len() {
            let rem = TEASPOONS - accs.iter().sum::<i8>();
            let mut out = Vec::from(accs);
            out.push(rem);
            assert_eq!(out.len(), ct);
            opts.push(out)
        } else {
            let mut back = Vec::from(accs);
            back.push(0);
            for i in 0..=(TEASPOONS - accs.iter().sum::<i8>()) {
                back[accs.len()] = i;
                tsp_layer(ct, &back, opts)
            }
        }
    }

    tsp_layer(len, &[], &mut sels);

    sels
}

#[test]
fn test() {
    let ings = [
        Ingredient {
            capacity: -1,
            durability: -2,
            flavor: 6,
            texture: 3,
            calories: 8,
        },
        Ingredient {
            capacity: 2,
            durability: 3,
            flavor: -2,
            texture: -1,
            calories: 3,
        },
    ];

    let best = top_cookie_score(&ings);
    let b_500 = cal_cookie_score(&ings);

    assert_eq!(best, 62842880);
    assert_eq!(b_500, 57600000);
}
