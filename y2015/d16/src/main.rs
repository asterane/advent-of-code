// Advent of Code 2015
// Day 16

#[derive(Debug)]
struct Sue {
    children: Option<u8>,
    cats: Option<u8>,
    samoyeds: Option<u8>,
    pomeranians: Option<u8>,
    akitas: Option<u8>,
    vizslas: Option<u8>,
    goldfish: Option<u8>,
    trees: Option<u8>,
    cars: Option<u8>,
    perfumes: Option<u8>,
}

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut str_acc: Vec<u8> = Vec::new();
    let mut num_acc = 0;
    let mut in_fld = false;

    let mut aunts: Vec<Sue> = Vec::new();
    let mut cur = Sue {
        children: None,
        cats: None,
        samoyeds: None,
        pomeranians: None,
        akitas: None,
        vizslas: None,
        goldfish: None,
        trees: None,
        cars: None,
        perfumes: None,
    };

    for c in input {
        match c {
            b',' | b'\n' => {
                match &str_acc[..] {
                    b"children" => cur.children = Some(num_acc),
                    b"cats" => cur.cats = Some(num_acc),
                    b"samoyeds" => cur.samoyeds = Some(num_acc),
                    b"pomeranians" => cur.pomeranians = Some(num_acc),
                    b"akitas" => cur.akitas = Some(num_acc),
                    b"vizslas" => cur.vizslas = Some(num_acc),
                    b"goldfish" => cur.goldfish = Some(num_acc),
                    b"trees" => cur.trees = Some(num_acc),
                    b"cars" => cur.cars = Some(num_acc),
                    b"perfumes" => cur.perfumes = Some(num_acc),
                    _ => panic!(),
                }

                str_acc.clear();
                num_acc = 0;

                if *c == b'\n' {
                    aunts.push(cur);
                    cur = Sue {
                        children: None,
                        cats: None,
                        samoyeds: None,
                        pomeranians: None,
                        akitas: None,
                        vizslas: None,
                        goldfish: None,
                        trees: None,
                        cars: None,
                        perfumes: None,
                    };
                }
            }
            b'S' => in_fld = false,
            b':' => in_fld = true,
            b' ' => (),
            _ if c.is_ascii_lowercase() => {
                if in_fld {
                    str_acc.push(*c);
                }
            }
            _ if c.is_ascii_digit() => {
                if in_fld {
                    num_acc *= 10;
                    num_acc += c & 15;
                }
            }
            _ => panic!(),
        }
    }

    // println!("{}", aunts.len());

    let query = Sue {
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    macro_rules! check {
        ($aunt:ident, $query:ident, ==$field:ident) => {
            if $aunt.$field.is_none() || $query.$field.is_none() {
                true
            } else {
                $aunt.$field == $query.$field
            }
        };
        ($aunt:ident, $query:ident, >$field:ident) => {
            if $aunt.$field.is_none() || $query.$field.is_none() {
                true
            } else {
                $aunt.$field > $query.$field
            }
        };
        ($aunt:ident, $query:ident, <$field:ident) => {
            if $aunt.$field.is_none() || $query.$field.is_none() {
                true
            } else {
                $aunt.$field < $query.$field
            }
        };
    }

    let mut match_1 = Vec::new();
    let mut match_2 = Vec::new();
    for (i, a) in aunts.iter().enumerate() {
        let mut match_p = true;
        let mut real_p = true;

        match_p &= check!(a, query, ==children);
        match_p &= check!(a, query, ==cats);
        match_p &= check!(a, query, ==samoyeds);
        match_p &= check!(a, query, ==pomeranians);
        match_p &= check!(a, query, ==akitas);
        match_p &= check!(a, query, ==vizslas);
        match_p &= check!(a, query, ==goldfish);
        match_p &= check!(a, query, ==trees);
        match_p &= check!(a, query, ==cars);
        match_p &= check!(a, query, ==perfumes);

        real_p &= check!(a, query, ==children);
        real_p &= check!(a, query, ==samoyeds);
        real_p &= check!(a, query, ==akitas);
        real_p &= check!(a, query, ==vizslas);
        real_p &= check!(a, query, ==cars);
        real_p &= check!(a, query, ==perfumes);
        real_p &= check!(a, query, >cats);
        real_p &= check!(a, query, >trees);
        real_p &= check!(a, query, <pomeranians);
        real_p &= check!(a, query, <goldfish);

        if match_p {
            match_1.push(i + 1)
        }
        if real_p {
            match_2.push(i + 1)
        }
    }

    assert_eq!(match_1.len(), 1);
    assert_eq!(match_2.len(), 1);

    println!("Gift-giving Sue: {}", match_1[0]); // 40
    println!("Real Sue: {}", match_2[0]);        // 241

    // Correct!
}
