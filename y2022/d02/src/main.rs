// Advent of Code 2022
// Day 2: Rock Paper Scissors

#[derive(Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut score_1 = 0;
    let mut score_2 = 0;

    let mut cur = input.iter();
    loop {
        let opp_mv = match cur.next() {
            Some(b'A') => Move::Rock,
            Some(b'B') => Move::Paper,
            Some(b'C') => Move::Scissors,
            Some(_) => panic!(),
            None => break,
        };
        assert_eq!(*(cur.next().unwrap()), b' ');
        let (slf_mv_1, slf_mv_2) = match cur.next().unwrap() {
            b'X' => (Move::Rock, match opp_mv {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            }),
            b'Y' => (Move::Paper, opp_mv),
            b'Z' => (Move::Scissors, match opp_mv {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            }),
            _ => panic!(),
        };
        assert_eq!(*(cur.next().unwrap()), b'\n');

        score_1 += score_round(slf_mv_1, opp_mv);
        score_2 += score_round(slf_mv_2, opp_mv);
    }

    // Part 1
    println!("Final score, assumed: {}", score_1); // 11449

    // Part 2
    println!("Final score, correct: {}", score_2); // 13187

    // Correct!
}

fn score_round(slf_mv: Move, opp_mv: Move) -> u16 {
    match slf_mv {
        Move::Rock => {
            1 + match opp_mv {
                Move::Rock => 3,
                Move::Paper => 0,
                Move::Scissors => 6,
            }
        }
        Move::Paper => {
            2 + match opp_mv {
                Move::Rock => 6,
                Move::Paper => 3,
                Move::Scissors => 0,
            }
        }
        Move::Scissors => {
            3 + match opp_mv {
                Move::Rock => 0,
                Move::Paper => 6,
                Move::Scissors => 3,
            }
        }
    }
}
