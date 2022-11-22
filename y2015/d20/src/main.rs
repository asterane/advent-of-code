// Advent of Code 2015
// Day 20

// const MULT: u32 = 2; // option for more cases
const MULT: u32 = 32;

fn main() {
    let input = 33100000;

    // The concept of harmonic series may apply here
    // let mut house_no = 0; // option for more cases
    let mut house_no = (input / 43) & (u32::MAX - (MULT - 1));

    let mut acc = 0;
    while acc < input {
        house_no += MULT;

        acc = 10 * (house_no + house_no / 2) + 30;
        for i in 3.. {
            let (quo, rem) = (house_no / i, house_no % i);
            if i >= quo {
                break;
            }
            if rem == 0 {
                acc += 10 * (i + quo)
            }
        }

        // println!("{}: {}", house_no, acc);
    }

    println!("First house; first pattern: {}", house_no); // 776160

    let mut acc = 0;
    while acc < input {
        house_no += MULT;

        acc = 11 * (house_no + house_no / 2);
        for i in 3..50 {
            let (quo, rem) = (house_no / i, house_no % i);
            if rem == 0 {
                acc += 11 * quo
            }
        }

        // println!("{}: {}", house_no, acc);
    }

    println!("First house; second pattern: {}", house_no); // 786240

    // Correct!
}

#[test]
fn test() {
    let house_no = 6;
    assert_eq!(
        {
            let mut acc = 10 * house_no + 10;
            for i in 2..=(house_no / 2) {
                if house_no % i == 0 {
                    acc += 10 * i
                }
            }
            acc
        },
        120
    )
}
