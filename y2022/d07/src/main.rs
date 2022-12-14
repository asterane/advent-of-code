// Advent of Code 2022
// Day 7: No Space Left On Device

enum Node {
    File(u32),
    Dir(Vec<u16>),
}

enum Cmd {
    _U,
    Cd,
    Ls,
}

enum Mode {
    Cmd,
    Arg,
    Siz,
    Nam,
}

const P1_MAX_SIZE: u32 = 100000;
const SPACE_AVAIL: u32 = 70000000;
const SPACE_REQRD: u32 = 30000000;

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut names: Vec<(u16, Box<[u8]>)> = vec![(0, Box::from(*b"/"))];
    let mut nodes: Vec<Node> = vec![Node::Dir(Vec::new())];

    let mut cmd = Cmd::_U;

    let mut dir_p = false;
    let mut dir_within = 0;

    let mut name_acc: Vec<u8> = Vec::new();
    let mut size_acc = 0;

    let mut mode = Mode::Cmd;

    for c in input {
        match (c, &mode) {
            (b'$', _) => {
                mode = Mode::Cmd;
                cmd = Cmd::_U;
            }
            (b' ', Mode::Cmd) => {
                mode = match cmd {
                    Cmd::_U => mode,
                    Cmd::Cd => Mode::Arg,
                    Cmd::Ls => panic!(),
                };
            }
            (b' ', Mode::Siz) => mode = Mode::Nam,
            (b'\n', Mode::Cmd) => {
                assert!(matches!(cmd, Cmd::Ls));
                mode = Mode::Siz;
            }
            (b'\n', Mode::Arg) => {
                dir_within = if &name_acc[..] == b".." {
                    nodes
                        .iter()
                        .position(|n| match n {
                            Node::Dir(cts) => cts.contains(&dir_within),
                            Node::File(_) => false,
                        })
                        .unwrap() as u16
                } else {
                    names
                        .iter()
                        .position(|n| n.0 == dir_within && &name_acc[..] == &n.1[..])
                        .unwrap() as u16
                };

                mode = Mode::Siz;
                name_acc.clear();
            }
            (b'\n', Mode::Nam) => {
                let id = names
                    .iter()
                    .position(|n| n.0 == dir_within && &name_acc[..] == &n.1[..])
                    .unwrap_or_else(|| {
                        nodes.push(if dir_p {
                            Node::Dir(Vec::new())
                        } else {
                            Node::File(size_acc)
                        });

                        let name = name_acc.clone().into_boxed_slice();
                        names.push((dir_within, name));

                        assert_eq!(names.len(), nodes.len());
                        nodes.len() - 1
                    }) as u16;

                let Node::Dir(cts) = &mut nodes[dir_within as usize] else { panic!() };

                if !cts.contains(&id) {
                    cts.push(id)
                }

                mode = Mode::Siz;
                dir_p = false;

                name_acc.clear();
                size_acc = 0;
            }
            (b'c', Mode::Cmd) => cmd = Cmd::Cd,
            (b'l', Mode::Cmd) => cmd = Cmd::Ls,
            (b'd' | b's', Mode::Cmd) => (),
            (b'd' | b'i' | b'r', Mode::Siz) => dir_p = true,
            (_, Mode::Siz) if c.is_ascii_digit() => {
                size_acc *= 10;
                size_acc += (c & 15) as u32;
            }
            (b'/' | b'.', Mode::Arg | Mode::Nam) => name_acc.push(*c),
            (_, Mode::Arg | Mode::Nam) if c.is_ascii_alphabetic() => name_acc.push(*c),
            _ => panic!(),
        }
    }

    let mut layer = 1;
    let mut path = vec![0];
    let mut elts = vec![0];

    let mut accs = vec![0; 2];
    let mut sizs: Vec<u32> = vec![];

    let mut space_used = 0;

    // println!("- / (dir)");
    loop {
        let id_within = *(path.last().unwrap());
        let current_node_id = if let Node::Dir(cts) = &nodes[id_within] {
            if elts.last().unwrap() >= &cts.len() {
                let final_size = accs.pop().unwrap();
                sizs.push(final_size);
                *(accs.last_mut().unwrap()) += final_size;

                if layer == 1 {
                    break;
                }

                layer -= 1;
                path.pop();
                elts.pop();
                continue;
            }

            cts[*(elts.last().unwrap())]
        } else {
            panic!()
        } as usize;

        // for _ in 0..layer {
        //     print!("  ")
        // }

        // print!(
        //     "- {} ",
        //     String::from_utf8(names[current_node_id].1.to_vec()).unwrap()
        // );

        *(elts.last_mut().unwrap()) += 1;

        match &nodes[current_node_id] {
            Node::Dir(c) => {
                if !c.is_empty() {
                    layer += 1;
                    path.push(current_node_id);
                    elts.push(0);
                    accs.push(0);
                }
                // print!("(dir)")
            }
            Node::File(s) => {
                *(accs.last_mut().unwrap()) += s;
                space_used += s;
                // print!("(file, size={s})")
            }
        }
        // println!();
    }

    let must_free = SPACE_REQRD - (SPACE_AVAIL - space_used);

    let sum_of_dirs_with_size_up_to_max = sizs.iter().filter(|s| **s <= P1_MAX_SIZE).sum::<u32>();
    let size_of_smallest_dir_to_free = sizs.iter().filter(|s| **s > must_free).min().unwrap();

    // Part 1
    println!(
        "Sum of sizes for all directories up to size {P1_MAX_SIZE}: {}",
        sum_of_dirs_with_size_up_to_max
    ); // 1491614

    // Part 2
    println!(
        "Size of smallest directory to delete for update space: {}",
        size_of_smallest_dir_to_free
    ); // 6400111

    // Correct!
}
