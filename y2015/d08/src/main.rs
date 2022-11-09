// Advent of Code 2015
// Day 8

fn main() {
    let input = std::include_bytes!("../input.txt");

    let (mut code, mut data, mut encd) = (0, 0, 0);

    let mut cursor = input.iter();
    loop {
        match cursor.next() {
            Some(b'\n') => encd += 2,
            Some(b'\"') => {
                code += 1;
                encd += 2;
            }
            Some(b'\\') => match cursor.next() {
                Some(b'x') => {
                    cursor.next();
                    cursor.next();
                    code += 4;
                    data += 1;
                    encd += 5;
                }
                Some(b'\"' | b'\\') => {
                    code += 2;
                    data += 1;
                    encd += 4;
                }
                Some(_) => {
                    code += 2;
                    data += 1;
                    encd += 3;
                }
                None => panic!(),
            },
            Some(_) => {
                code += 1;
                data += 1;
                encd += 1;
            }
            None => break,
        }
    }

    let diff_cd = code - data;
    let diff_ec = encd - code;

    println!("Code chars minus data chars: {}", diff_cd); // 1371
    println!("Encoded chars minus code chars: {}", diff_ec); // 2117

    // Correct!
}
