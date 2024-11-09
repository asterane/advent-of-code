// Advent of Code 2016
// Day 9: Explosives in Cyberspace

fn main() {
    let input = include_bytes!("../input.txt");

    let decomp_once_len = decompress_span(input, false);
    let decomp_full_len = decompress_span(input, true);

    println!("Decompressed length (once): {}", decomp_once_len); // 98135

    println!("Decompressed length (full): {}", decomp_full_len); // 10964557606
}

fn decompress_span(inp: &[u8], recur: bool) -> usize {
    let mut out_len = 0;

    let mut section = vec![];
    let mut pos = 0;

    let mut in_section = false;
    let mut in_marker = false;

    let mut acc = 0;
    let (mut len, mut reps) = (0, 0);

    for b in inp {
        match b {
            _ if b.is_ascii_whitespace() => (),
            _ if in_section => {
                assert!(!in_marker);

                section.push(*b);
                pos += 1;

                if pos >= len {
                    let full_len = if recur {
                        decompress_span(&section, true)
                    } else {
                        section.len()
                    };

                    out_len += reps * full_len;

                    section.clear();

                    in_section = false;
                }
            }
            b'(' => {
                assert!(!in_section);
                in_marker = true;
            }
            b')' if in_marker => {
                reps = acc;
                acc = 0;
                in_marker = false;

                pos = 0;
                in_section = true;
            }
            b'x' if in_marker => {
                len = acc;
                acc = 0;
            }
            _ if b.is_ascii_digit() && in_marker => {
                acc *= 10;
                acc += (b & 0xF) as usize;
            }
            _ => out_len += 1,
        }
    }

    assert_eq!(section.len(), 0);

    out_len
}
