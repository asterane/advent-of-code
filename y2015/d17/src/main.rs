// Advent of Code 2015
// Day 17

const NOG_VOL: u8 = 150;

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut containers: Vec<u8> = Vec::new();
    let mut acc = 0;

    for c in input {
        match c {
            b'\n' => {
                containers.push(acc);
                acc = 0;
            }
            _ if c.is_ascii_digit() => {
                acc *= 10;
                acc += c & 15
            }
            _ => panic!(),
        }
    }

    containers.sort();
    containers.reverse();

    // println!("{:?}", containers);

    let (count, min_ct) = count_eggnog_options(&containers, NOG_VOL);

    println!("Container combinations: {}", count);          // 4372
    println!("Ways to use minimum containers: {}", min_ct); // 4

    // Correct!
}

fn count_eggnog_options(containers: &[u8], volume: u8) -> (u32, u32) {
    let mut cptr = 0;
    let mut acc = 0;
    let mut count = 0;

    let mut addends = Vec::new();
    let mut con_cts = Vec::new();

    'fill: loop {
        let cv = containers[cptr];
        acc += cv;

        if acc < volume {
            addends.push(cptr)
        } else if acc == volume {
            // for a in &addends {
            //     print!("{} + ", containers[*a]);
            // }
            // println!("{}", cv);
            con_cts.push(addends.len() + 1);
            count += 1;
            acc -= cv;
        } else if acc > volume {
            acc -= cv
        }

        cptr += 1;

        while cptr >= containers.len() {
            let Some(lasti) = addends.pop() else { break 'fill };
            acc -= containers[lasti];
            cptr = lasti + 1;
        }
    }

    let min_con = *(con_cts.iter().min().unwrap());
    let min_ct = con_cts
        .into_iter()
        .fold(0, |a, e| if min_con == e { a + 1 } else { a }) as u32;

    (count, min_ct)
}

#[test]
fn test_fill() {
    let cont = [20, 15, 10, 5, 5];
    let vol = 25;

    let (count, min_ct) = count_eggnog_options(&cont, vol);

    assert_eq!(count, 4);
    assert_eq!(min_ct, 3);
}
