// Advent of Code 2022
// Day 1: Calorie Counting

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut cal_acc = 0;
    let mut elf_acc = 0;

    let mut top_cal_elves = [0, 0, 0];
    let mut newl = false;

    for c in input {
        match c {
            b'\n' => {
                if newl {
                    if elf_acc >= top_cal_elves[0] {
                        top_cal_elves[2] = top_cal_elves[1];
                        top_cal_elves[1] = top_cal_elves[0];
                        top_cal_elves[0] = elf_acc
                    } else if elf_acc >= top_cal_elves[1] {
                        top_cal_elves[2] = top_cal_elves[1];
                        top_cal_elves[1] = elf_acc
                    } else if elf_acc >= top_cal_elves[2] {
                        top_cal_elves[2] = elf_acc
                    }
                    elf_acc = 0;
                } else {
                    elf_acc += cal_acc;
                    cal_acc = 0;
                }
                newl = true;
            }
            _ if c.is_ascii_digit() => {
                cal_acc *= 10;
                cal_acc += (c & 15) as u32;
                newl = false;
            }
            _ => panic!(),
        }
    }

    let top_elf_sum: u32 = top_cal_elves.iter().sum();

    // Part 1
    println!("Most caloric Elf's sum: {}", top_cal_elves[0]); // 70720

    // Part 2
    println!("Top 3 calorie-carriers' sum: {}", top_elf_sum); // 207148

    // Correct!
}
