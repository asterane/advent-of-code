// Advent of Code 2015
// Day 12

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut sum: i32 = 0;
    let red_sum;

    let mut neg = false;
    let mut acc = 0;

    let mut obj_depth = 0;
    let mut obj_sum = vec![0];

    let mut in_prop = false;
    let mut redi: u8 = 0;
    let mut redf = Vec::new();

    macro_rules! handle_acc {
        () => {
            if neg {
                acc = -acc;
            }

            sum += acc;
            obj_sum[obj_depth] += acc;

            acc = 0;
            neg = false;
        };
    }

    for c in input {
        match c {
            b'{' => {
                obj_depth += 1;
                obj_sum.push(0);
                redf.push(false);
            }
            b'}' => {
                handle_acc!();

                let blk_sum = obj_sum.pop().unwrap();

                obj_depth -= 1;
                if !redf.pop().unwrap() {
                    obj_sum[obj_depth] += blk_sum
                }

                in_prop = false;
            }
            b':' => in_prop = true,
            b'[' => in_prop = false,
            b',' => {
                handle_acc!();

                in_prop = false;
                redi = 0;
            }
            b'r' => {
                if in_prop && redi == 0 {
                    redi = 1
                }
            }
            b'e' => {
                if in_prop && redi == 1 {
                    redi = 2
                }
            }
            b'd' => {
                if in_prop && redi == 2 {
                    redi = 0;
                    redf[obj_depth - 1] = true;
                }
            }
            b'-' => neg = true,
            _ if c.is_ascii_digit() => {
                acc *= 10;
                acc += (c & 15) as i32;
            }
            _ => {
                handle_acc!();
            }
        }
    }

    assert_eq!(obj_sum.len(), 1);
    red_sum = obj_sum[0];

    println!("Sum of all: {}", sum);          // 119433
    println!("Sum without red: {}", red_sum); // 68466

    // Correct!
}
