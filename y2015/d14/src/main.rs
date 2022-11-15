// Advent of Code 2015
// Day 14

#[derive(Debug)]
struct Reindeer {
    speed: u16,
    flight_time: u16,
    rest_time: u16,
}

fn main() {
    let input = std::include_bytes!("../input.txt");

    let mut herd: Vec<Reindeer> = Vec::new();

    let (mut speed_acc, mut flight_acc, mut rest_acc) = (0, 0, 0);
    let (mut speed_p, mut flight_p) = (false, false);
    let mut in_num = false;

    for c in input {
        match c {
            b' ' => {
                if in_num {
                    flight_p = speed_p;
                    speed_p = true;
                }
            }
            b'\n' => {
                herd.push(Reindeer {
                    speed: speed_acc,
                    flight_time: flight_acc,
                    rest_time: rest_acc,
                });
                (speed_acc, flight_acc, rest_acc) = (0, 0, 0);
                (speed_p, flight_p) = (false, false);
            }
            _ if c.is_ascii_digit() => {
                let acc = if !flight_p && !speed_p {
                    &mut speed_acc
                } else if !flight_p {
                    &mut flight_acc
                } else {
                    &mut rest_acc
                };

                *acc *= 10;
                *acc += (c & 15) as u16;

                in_num = true;
            }
            _ if c.is_ascii() => in_num = false,
            _ => panic!(),
        }
    }

    // println!("{:?}", herd);

    let dist_rslt = race(&herd, 2503);
    let pts_rslt = score(&herd, 2503);

    let win_dist = dist_rslt.iter().max().unwrap();
    let win_pts = pts_rslt.iter().max().unwrap();

    println!("Winner's distance: {}", win_dist); // 2640
    println!("Winner's points: {}", win_pts);    // 1102

    // Correct!
}

fn race(reindeer: &[Reindeer], time: u16) -> Vec<u16> {
    let mut distance = Vec::new();

    for r in reindeer {
        let time_div = time / (r.flight_time + r.rest_time);
        let time_rem = time % (r.flight_time + r.rest_time);

        distance
            .push(((r.speed * r.flight_time) * time_div) + (r.speed * time_rem.min(r.flight_time)));
    }

    distance
}

fn score(reindeer: &[Reindeer], time: u16) -> Vec<u16> {
    let mut points = vec![0; reindeer.len()];
    let mut dist = vec![0; reindeer.len()];
    let mut modes = vec![(true, 0); reindeer.len()];

    for i in 1..=time {
        let mut lead = 0;
        for (j, r) in reindeer.iter().enumerate() {
            if modes[j].0 {
                dist[j] += r.speed;
                if modes[j].1 + r.flight_time <= i {
                    modes[j] = (false, i)
                }
            } else {
                if modes[j].1 + r.rest_time <= i {
                    modes[j] = (true, i)
                }
            }
            if dist[j] > lead {
                lead = dist[j]
            }
        }
        for (j, d) in dist.iter().enumerate() {
            assert!(*d <= lead);
            if *d == lead {
                points[j] += 1
            }
        }
    }

    points
}

#[cfg(test)]
mod tests {
    use super::Reindeer;
    use super::{race, score};

    const PAIR: [Reindeer; 2] = [
        Reindeer {
            speed: 14,
            flight_time: 10,
            rest_time: 127,
        },
        Reindeer {
            speed: 16,
            flight_time: 11,
            rest_time: 162,
        },
    ];

    #[test]
    fn trial_dist() {
        let results = race(&PAIR, 1000);

        assert_eq!(results[0], 1120);
        assert_eq!(results[1], 1056);
    }

    #[test]
    fn trial_scor() {
        let results = score(&PAIR, 1000);

        assert_eq!(results[0], 312);
        assert_eq!(results[1], 689);
    }
}
