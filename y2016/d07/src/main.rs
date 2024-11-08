// Advent of Code 2016
// Day 7: Internet Protocol Version 7

fn main() {
    let input = include_bytes!("../input.txt");

    let mut spw = 0;
    let mut span = [0u8; 4];
    let mut bseq = false;

    let mut abba = false;
    let mut prec = false;

    let mut res_aba_bab = false;
    let mut abs: Vec<(u8, u8, bool)> = vec![];

    let mut support_tls = 0;
    let mut support_ssl = 0;

    for b in input.iter() {
        match b {
            b'\n' => {
                spw = 0;
                assert!(!bseq);

                if abba && !prec {
                    support_tls += 1;
                }
                abba = false;
                prec = false;

                if res_aba_bab {
                    support_ssl += 1;
                }
                res_aba_bab = false;
                abs.clear();
            }
            b'[' => {
                spw = 0;
                bseq = true;
            }
            b']' => {
                spw = 0;
                bseq = false;
            }
            _ if b.is_ascii_alphabetic() => {
                span[0] = span[1];
                span[1] = span[2];
                span[2] = span[3];
                span[3] = *b;

                if !res_aba_bab && spw > 1 && span[1] != span[2] && span[1] == span[3] {
                    if abs.contains(&(span[2], span[3], !bseq)) {
                        res_aba_bab = true;
                    } else {
                        abs.push((span[3], span[2], bseq));
                    }
                }

                if spw < 3 {
                    spw += 1;
                } else if !prec && span[0] != span[1] && span[1] == span[2] && span[0] == span[3] {
                    if bseq {
                        prec = true;
                    } else {
                        abba = true;
                    }
                }
            }
            _ => panic!(),
        }
    }

    println!("IPs with TLS support: {}", support_tls); // 115

    println!("IPs with SSL support: {}", support_ssl); // 231
}
