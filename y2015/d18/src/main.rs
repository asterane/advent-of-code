// Advent of Code 2015
// Day 18

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut grid = [[false; 100]; 100];

    let (mut row, mut col) = (0, 0);

    for c in input {
        match c {
            b'.' => col += 1,
            b'#' => {
                grid[row][col] = true;
                col += 1;
            }
            b'\n' => {
                col = 0;
                row += 1;
            }
            _ => panic!(),
        }
    }

    let mut stk_grid = grid;
    stk_grid[0][0] = true;
    stk_grid[0][99] = true;
    stk_grid[99][0] = true;
    stk_grid[99][99] = true;

    fn cell_ct(grid: &[[bool; 100]; 100], r: usize, c: usize) -> u8 {
        let mut neighbor_ct = 0;

        if r > 0 && c > 0 {
            neighbor_ct += grid[r - 1][c - 1] as u8
        }
        if r > 0 {
            neighbor_ct += grid[r - 1][c] as u8
        }
        if r > 0 && c < 99 {
            neighbor_ct += grid[r - 1][c + 1] as u8
        }
        if c < 99 {
            neighbor_ct += grid[r][c + 1] as u8
        }
        if r < 99 && c < 99 {
            neighbor_ct += grid[r + 1][c + 1] as u8
        }
        if r < 99 {
            neighbor_ct += grid[r + 1][c] as u8
        }
        if r < 99 && c > 0 {
            neighbor_ct += grid[r + 1][c - 1] as u8
        }
        if c > 0 {
            neighbor_ct += grid[r][c - 1] as u8
        }

        neighbor_ct
    }

    for _ in 0..100 {
        let mut next = [[false; 100]; 100];

        for r in 0..100 {
            for c in 0..100 {
                let neighbor_ct = cell_ct(&grid, r, c);
                next[r][c] = if grid[r][c] {
                    neighbor_ct == 2 || neighbor_ct == 3
                } else {
                    neighbor_ct == 3
                };
            }
        }

        grid = next;
    }

    let lights_on: u16 = grid.iter().flatten().fold(0, |a, e| a + *e as u16);

    for _ in 0..100 {
        let mut next = [[false; 100]; 100];

        for r in 0..100 {
            for c in 0..100 {
                let neighbor_ct = cell_ct(&stk_grid, r, c);
                next[r][c] = if stk_grid[r][c] {
                    neighbor_ct == 2 || neighbor_ct == 3
                } else {
                    neighbor_ct == 3
                };
            }
        }

        stk_grid = next;

        stk_grid[0][0] = true;
        stk_grid[0][99] = true;
        stk_grid[99][0] = true;
        stk_grid[99][99] = true;
    }

    let stk_lights_on: u16 = stk_grid.iter().flatten().fold(0, |a, e| a + *e as u16);

    println!("Lights after 100 steps: {}", lights_on);        // 814
    println!("Lights with corners stuck: {}", stk_lights_on); // 924

    // Correct!
}
