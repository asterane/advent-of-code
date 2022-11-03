// Advent of Code 2015
// Day 2

fn main() {
    let input = std::include_bytes!("../input.txt");

    let (mut sqft, mut lnft) = (0, 0);

    let (mut acc, mut lid) = (0, 0);
    let mut dims = [0, 0, 0];

    let mut cursor = input.iter();
    while let Some(b) = cursor.next() {
        match b {
            b'0'..=b'9' => {
                acc *= 10;
                acc += (b & 15) as u32;
            }
            b'x' => {
                dims[lid] = acc;
                acc = 0;
                lid += 1;
            }
            b'\n' => {
                if lid != 2 {
                    continue;
                }

                dims[lid] = acc;
                sqft += paper(&dims);
                lnft += ribbon(&dims);

                acc = 0;
                lid = 0;
            }
            _ => panic!(),
        }
    }

    println!("Paper Square Feet: {}", sqft); // 1586300

    println!("Ribbon Linear Feet: {}", lnft); // 3737498

    // Correct!
}

fn paper(dims: &[u32; 3]) -> u32 {
    let sides = [dims[0] * dims[1], dims[1] * dims[2], dims[2] * dims[0]];
    let min = *sides.iter().min().unwrap();
    2 * sides.iter().sum::<u32>() + min
}

fn ribbon(dims: &[u32; 3]) -> u32 {
    let faces = [
        2 * (dims[0] + dims[1]),
        2 * (dims[1] + dims[2]),
        2 * (dims[2] + dims[0]),
    ];
    let min = *faces.iter().min().unwrap();
    dims[0] * dims[1] * dims[2] + min
}
