// Advent of Code 2022
// Day 8: Treetop Tree House

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut row: Vec<u8> = Vec::new();

    for c in input {
        match c {
            b'\n' => {
                grid.push(row.clone());
                assert_eq!(row.len(), grid[0].len());
                row.clear();
            }
            _ if c.is_ascii_digit() => row.push(c & 15),
            _ => panic!(),
        }
    }

    let mut tree_count = 2 * grid.len() + 2 * grid[0].len() - 4;
    let mut top_scenic = 0;

    for y in 1..grid.len() - 1 {
        let row = &grid[y];
        for x in 1..row.len() - 1 {
            let tree = row[x];
            let mut v = false;
            let mut iv = false;
            let mut sacc = 1;
            let mut sct = 0;

            for u in (0..y).rev() {
                sct += 1;
                if grid[u][x] >= tree {
                    iv = true;
                    break;
                }
            }

            if !iv {
                tree_count += 1;
                v = true;
            }

            sacc *= sct;
            sct = 0;
            iv = false;

            for r in x + 1..row.len() {
                sct += 1;
                if grid[y][r] >= tree {
                    iv = true;
                    break;
                }
            }

            if !(v | iv) {
                tree_count += 1;
                v = true;
            }

            sacc *= sct;
            sct = 0;
            iv = false;

            for d in y + 1..grid.len() {
                sct += 1;
                if grid[d][x] >= tree {
                    iv = true;
                    break;
                }
            }

            if !(v | iv) {
                tree_count += 1;
                v = true;
            }

            sacc *= sct;
            sct = 0;
            iv = false;

            for l in (0..x).rev() {
                sct += 1;
                if grid[y][l] >= tree {
                    iv = true;
                    break;
                }
            }

            if !(v | iv) {
                tree_count += 1;
            }

            sacc *= sct;

            top_scenic = sacc.max(top_scenic);
        }
    }

    // Part 1
    println!("Visible trees: {}", tree_count); // 1681

    // Part 2
    println!("Highest scenic score: {}", top_scenic); // 201684

    // Correct!
}
