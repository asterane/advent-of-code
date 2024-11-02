// Advent of Code 2016
// Day 2: Bathroom Security

fn main() {
    let input = include_bytes!("../input.txt");

    let mut code_9 = String::new();
    let mut c9dig = 5;

    let mut code_13 = String::new();
    let mut c13dig = 5;

    for b in input {
        match b {
            b'\n' => {
                code_9.push(char::from_digit(c9dig, 10).unwrap());
                code_13.push(char::from_digit(c13dig, 14).unwrap())
            }
            b'U' => {
                match c9dig {
                    1..=3 => (),
                    4..=9 => c9dig -= 3,
                    _ => panic!(),
                }
                match c13dig {
                    1 | 2 | 4 | 5 | 9 => (),
                    3 | 13 => c13dig -= 2,
                    6..=8 | 10..=12 => c13dig -= 4,
                    _ => panic!(),
                }
            }
            b'D' => {
                match c9dig {
                    1..=6 => c9dig += 3,
                    7..=9 => (),
                    _ => panic!(),
                }
                match c13dig {
                    5 | 9 | 10 | 12 | 13 => (),
                    1 | 11 => c13dig += 2,
                    2..=4 | 6..=8 => c13dig += 4,
                    _ => panic!(),
                }
            }
            b'L' => {
                match c9dig {
                    1 | 4 | 7 => (),
                    2 | 3 | 5 | 6 | 8 | 9 => c9dig -= 1,
                    _ => panic!(),
                }
                match c13dig {
                    1 | 2 | 5 | 10 | 13 => (),
                    3 | 4 | 6..=9 | 11 | 12 => c13dig -= 1,
                    _ => panic!(),
                }
            }
            b'R' => {
                match c9dig {
                    3 | 6 | 9 => (),
                    1 | 2 | 4 | 5 | 7 | 8 => c9dig += 1,
                    _ => panic!(),
                }
                match c13dig {
                    1 | 4 | 9 | 12 | 13 => (),
                    2 | 3 | 5..=8 | 10 | 11 => c13dig += 1,
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    println!("Code (9-digit keypad): {}", code_9); // 98575

    println!("Code (13-digit keypad): {}", code_13.to_uppercase()); // CD8D4
}
