use aoc_runner_derive::{aoc, aoc_generator};

struct Notes {
    departure_time: usize,
    busses: Vec<Option<usize>>,
}

#[aoc_generator(day13)]
fn input_generator(input: &str) -> Notes {
    let mut lines = input.lines();

    Notes {
        departure_time: lines.next().unwrap().parse().unwrap(),
        busses: lines
            .next()
            .unwrap()
            .split(",")
            .map(|x| {
                if x == "x" {
                    None
                } else {
                    Some(x.parse().unwrap())
                }
            })
            .collect(),
    }
}

#[aoc(day13, part1)]
fn solve_part1(input: &Notes) -> usize {
    let waiting_times = input
        .busses
        .iter()
        .filter_map(|x| *x)
        .map(|bus| (bus - input.departure_time % bus, bus));
    let (min_waiting_time, best_bus) = waiting_times.min().unwrap();
    min_waiting_time * best_bus
}

#[aoc(day13, part2)]
fn solve_part2(input: &Notes) -> usize {
    let mut pairs: Vec<_> = input
        .busses
        .iter()
        .enumerate()
        .filter_map(|(idx, bus)| bus.map(|bus| (idx, bus)))
        .collect();

    pairs.sort_by_key(|(_, bus)| *bus);

    let (largest_idx, largest_bus) = pairs[pairs.len() - 1];
    let rest = &pairs[..pairs.len() - 1];

    for t in 1.. {
        let t = t * largest_bus - largest_idx;
        if rest.iter().all(|(idx, bus)| (t + *idx) % *bus == 0) {
            return t;
        }
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let notes = Notes {
            departure_time: 939,
            busses: vec![
                Some(7),
                Some(13),
                None,
                None,
                Some(59),
                None,
                Some(31),
                Some(19),
            ],
        };
        assert_eq!(solve_part1(&notes), 295);
    }
    #[test]
    fn part2() {
        let inputs = [
            (vec![Some(17), None, Some(13), Some(19)], 3417),
            (vec![Some(67), Some(7), Some(59), Some(61)], 754018),
            (vec![Some(67), None, Some(7), Some(59), Some(61)], 779210),
            (vec![Some(67), Some(7), None, Some(59), Some(61)], 1261476),
            (vec![Some(1789), Some(37), Some(47), Some(1889)], 1202161486),
        ];

        for (input, output) in inputs.iter() {
            assert_eq!(
                *output,
                solve_part2(&Notes {
                    departure_time: 0,
                    busses: input.clone()
                })
            )
        }
    }
}
