// Advent of Code 2016
// Day 4: Security Through Obscurity

fn main() {
    let input = include_bytes!("../input.txt");

    let mut freq = [0; 26];
    let mut id = 0;

    let mut chk = false;
    let mut ci = 0;

    let (mut csum, mut rsum) = ([0u8; 5], [0u8; 5]);

    let mut name = vec![];
    let mut real_names = vec![];

    let mut sector_sum = 0;

    for b in input {
        match b {
            b'\n' => {
                // println!(
                //     "Calculated: {}; Listed: {}",
                //     String::from_utf8_lossy(&csum),
                //     String::from_utf8_lossy(&rsum)
                // );

                if csum == rsum {
                    sector_sum += id;

                    name.pop();

                    real_names.push((
                        name.clone()
                            .into_iter()
                            .map(|b| {
                                if b == b'-' {
                                    b' '
                                } else {
                                    (((b - b'a') as u32 + id) % 26) as u8 + b'a'
                                }
                            })
                            .collect::<Vec<u8>>(),
                        id,
                    ));
                }

                name.clear();

                freq = [0; 26];
                id = 0;

                chk = false;
                ci = 0;
            }
            b'[' => {
                let mut srt = freq
                    .into_iter()
                    .enumerate()
                    .filter(|e| e.1 > 0)
                    .collect::<Vec<(usize, u32)>>();
                srt.sort_by_key(|e| 26 - e.1);
                srt.truncate(5);

                for (i, (l, _)) in srt.into_iter().enumerate() {
                    csum[i] = l as u8 + b'a';
                }

                chk = true;
            }
            b']' => {
                assert!(chk);
                assert_eq!(ci, 5);
            }
            b'-' => name.push(*b),
            _ if b.is_ascii_lowercase() => {
                if chk {
                    rsum[ci] = *b;
                    ci += 1;
                } else {
                    freq[(b - b'a') as usize] += 1;
                    name.push(*b);
                }
            }
            _ if b.is_ascii_digit() => {
                id *= 10;
                id += (b & 0xF) as u32;
            }
            _ => panic!(),
        }
    }

    // for (n, _) in real_names.iter() {
    //     println!("{}", String::from_utf8_lossy(n));
    // }

    let target_id = real_names
        .into_iter()
        .find_map(|(n, id)| {
            if &n == &b"northpole object storage" {
                Some(id)
            } else {
                None
            }
        })
        .unwrap();

    println!("Real room sector ID sum: {}", sector_sum); // 278221

    println!("North Pole object storage ID: {}", target_id); // 267
}
