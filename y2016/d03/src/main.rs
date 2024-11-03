// Advent of Code 2016
// Day 3: Squares With Three Sides

fn main() {
    let input = include_bytes!("../input.txt");

    let mut tri_rows = vec![[0; 3]];
    let mut tri_cols = vec![[0; 3]; 3];

    let mut row = 0;
    let mut col = 0;

    let mut acc = 0;

    for b in input {
        match b {
            b'\n' => {
                tri_rows.last_mut().unwrap()[row] = acc;
                tri_rows.push([0; 3]);

                let tcl = tri_cols.len();
                tri_cols[tcl - 3 + row][col % 3] = acc;

                row = 0;
                col += 1;

                if col % 3 == 0 {
                    tri_cols.extend_from_slice(&[[0; 3]; 3]);
                }

                acc = 0;
            }
            b' ' => {
                if acc != 0 {
                    tri_rows.last_mut().unwrap()[row] = acc;

                    let tcl = tri_cols.len();
                    tri_cols[tcl - 3 + row][col % 3] = acc;

                    row += 1;
                    acc = 0;
                }
            }
            _ if b.is_ascii_digit() => {
                acc *= 10;
                acc += (b & 0xF) as u32;
            }
            _ => panic!(),
        }
    }

    tri_rows.pop();
    tri_cols.truncate(col);

    let poss_rows = tri_check(&tri_rows);
    let poss_cols = tri_check(&tri_cols);

    println!("Possible triangles in rows: {}", poss_rows); // 993

    println!("Possible triangles in columns: {}", poss_cols); // 1849
}

fn tri_check(coords: &[[u32; 3]]) -> u32 {
    let mut possible = 0;
    for tr in coords {
        if tr[0] + tr[1] > tr[2] && tr[1] + tr[2] > tr[0] && tr[2] + tr[0] > tr[1] {
            possible += 1;
        }
    }
    possible
}
