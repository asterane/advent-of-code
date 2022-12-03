// Advent of Code 2022
// Day 3: Rucksack Reorganization

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut sacks: Vec<Vec<u8>> = vec![Vec::new()];
    let mut line = 0;

    for c in input {
        match c {
            b'\n' => {
                sacks.push(Vec::new());
                line += 1;
            }
            _ if c.is_ascii_lowercase() => sacks[line].push(c - b'a' + 1),
            _ if c.is_ascii_uppercase() => sacks[line].push(c - b'A' + 27),
            _ => panic!(),
        }
    }

    sacks.pop();

    let mut misplaced_priority_sum: u16 = 0;
    let mut badge_priority_sum: u16 = 0;
    let mut to_check: Vec<u8> = Vec::new();
    for (i, s) in sacks.into_iter().enumerate() {
        let (comp1, comp2) = s[..].split_at(s.len() / 2);
        for it in comp1 {
            if comp2.contains(it) {
                misplaced_priority_sum += *it as u16;
                break;
            }
        }
        to_check = if i % 3 == 0 {
            let mut temp = s.clone();
            temp.sort();
            temp.dedup();
            temp
        } else if i % 3 == 1 {
            to_check.into_iter().filter(|p| s.contains(p)).collect()
        } else {
            badge_priority_sum += to_check.into_iter().find(|p| s.contains(p)).unwrap() as u16;
            Vec::new()
        };
    }

    // Part 1
    println!("Sum of common item priorities: {}", misplaced_priority_sum); // 7701

    // Part 2
    println!("Sum of badge item priorities: {}", badge_priority_sum); // 2644

    // Correct!
}
