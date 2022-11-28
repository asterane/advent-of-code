// Advent of Code 2015
// Day 24

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut weights: Vec<u8> = Vec::new();

    let mut acc = 0;
    for c in input {
        match c {
            b'\n' => {
                weights.push(acc);
                acc = 0;
            }
            _ if c.is_ascii_digit() => {
                acc *= 10;
                acc += *c & 15;
            }
            _ => panic!(),
        }
    }

    weights.sort();

    let best_qe_3 = ideal_first_grp_qe(&weights, 3);
    let best_qe_4 = ideal_first_grp_qe(&weights, 4);

    println!(
        "First group's quantum entanglement (of three): {}",
        best_qe_3
    ); // 11266889531
    println!(
        "First group's quantum entanglement (of four): {}",
        best_qe_4
    ); // 77387711

    // Correct!
}

fn ideal_first_grp_qe(weights: &[u8], groups: u16) -> usize {
    let all_sum = weights.iter().fold(0, |a, w| a + *w as u16);
    let grp_wgt = all_sum / groups;

    let fwst_pkgs = first_group_len(&weights, grp_wgt).unwrap();
    let least_pkg_grps = get_min_groups(&weights, grp_wgt, fwst_pkgs);

    least_pkg_grps.iter().fold(usize::MAX, |a, e| {
        a.min(e.iter().fold(1, |b, f| b * weights[*f] as usize))
    })
}

fn first_group_len(weights: &[u8], target: u16) -> Option<usize> {
    let mut x = weights.len() - 1;
    let mut acc = 0;
    let mut fst_grp_len = 0;

    loop {
        acc += weights[x] as u16;
        fst_grp_len += 1;
        if acc > target {
            acc -= weights[x] as u16;
            fst_grp_len -= 1;
        } else if acc == target {
            break;
        }
        if x == 0 {
            return None;
        }
        x -= 1;
    }

    Some(fst_grp_len)
}

fn get_min_groups(weights: &[u8], target: u16, size: usize) -> Vec<Vec<usize>> {
    let mut cptr = 0;
    let mut acc = 0;

    let mut addends = Vec::new();
    let mut min_lens = Vec::new();

    'fill: loop {
        let cv = weights[cptr];
        acc += cv as u16;

        if acc < target {
            addends.push(cptr)
        } else if acc == target {
            // for a in &addends {
            //     print!("{} + ", weights[*a]);
            // }
            // println!("{}", cv);
            let len = addends.len() + 1;
            if len <= size {
                let mut nxt = addends.clone();
                nxt.push(cptr);
                min_lens.push(nxt);
            }
            acc -= cv as u16;
        } else if acc > target {
            acc -= cv as u16
        }

        cptr += 1;

        while cptr >= weights.len() {
            let Some(lasti) = addends.pop() else { break 'fill };
            acc -= weights[lasti] as u16;
            cptr = lasti + 1;
        }
    }

    min_lens
}
