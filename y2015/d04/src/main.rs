// Advent of Code 2015
// Day 4

#![allow(arithmetic_overflow)]

fn main() {
    let input = "bgvyzdsv";

    let mut n = 1;
    let result_5 = loop {
        let mut to_test = String::from(input);
        to_test.push_str(&n.to_string());
        let hash = md5_hash_str(&to_test);
        if hash >> (13 * 8 + 4) == 0 {
            // println!("{:032x}", hash);
            break n;
        }
        n += 1
    };

    println!("Five Leading Zeros: {}", result_5);   // 254575

    let mut n = 1;
    let result_6 = loop {
        let mut to_test = String::from(input);
        to_test.push_str(&n.to_string());
        let hash = md5_hash_str(&to_test);
        if hash >> (13 * 8) == 0 {
            // println!("{:032x}", hash);
            break n;
        }
        n += 1
    };

    println!("Six Leading Zeros: {}", result_6);

    // Correct!
}

// Based on MD5 pseudocode available on Wikipedia
fn md5_hash_str(msg: &str) -> u128 {
    let mut msg_bytes = msg.as_bytes().to_vec();
    let orig_bit_len = (msg_bytes.len() * 8) % usize::MAX;

    let s = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];

    let k = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
        0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
        0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
        0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
        0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
        0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
        0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
        0xeb86d391,
    ];

    let (mut a0, mut b0, mut c0, mut d0) = (0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476);

    msg_bytes.push(0x80);

    let overrun = msg_bytes.len() % 64;
    if overrun < 56 {
        msg_bytes.append(&mut vec![0x00; 56 - overrun]);
    } else if overrun > 56 {
        msg_bytes.append(&mut vec![0x00; 56 + (64 - overrun)]);
    }

    unsafe {
        msg_bytes.append(&mut std::mem::transmute::<usize, [u8; 8]>(orig_bit_len).to_vec());
    }

    assert_eq!(msg_bytes.len() % 64, 0);

    for chunk in msg_bytes.chunks_exact(64) {
        let (mut a, mut b, mut c, mut d) = (a0, b0, c0, d0);

        let m = unsafe {
            let mut smptr = std::mem::transmute::<&[u8], (*const (), usize)>(chunk);
            smptr.1 /= 4;
            std::mem::transmute::<(*const (), usize), &[u32]>(smptr)
        };

        assert_eq!(m.len() % 16, 0);

        for i in 0..64 {
            let (mut f, g);
            match i {
                0..=15 => {
                    f = (b & c) | ((!b) & d);
                    g = i;
                }
                16..=31 => {
                    f = (d & b) | ((!d) & c);
                    g = (5 * i + 1) % 16;
                }
                32..=47 => {
                    f = b ^ c ^ d;
                    g = (3 * i + 5) % 16;
                }
                48..=63 => {
                    f = c ^ (b | (!d));
                    g = (7 * i) % 16;
                }
                _ => unreachable!(),
            }
            f = f + a + k[i] + m[g];
            a = d;
            d = c;
            c = b;
            b += f.rotate_left(s[i]);
        }
        a0 += a;
        b0 += b;
        c0 += c;
        d0 += d;
    }

    let acc_cnct = unsafe { std::mem::transmute::<[u32; 4], u128>([a0, b0, c0, d0]) };
    acc_cnct.swap_bytes()
}

#[test]
fn hash_test() {
    assert_eq!(md5_hash_str(""), 0xd41d8cd98f00b204e9800998ecf8427e);
    assert_eq!(
        md5_hash_str(
            "12345678901234567890123456789012345678901234567890123456789012345678901234567890"
        ),
        0x57edf4a22be3c955ac49da2e2107b67a
    );
}
