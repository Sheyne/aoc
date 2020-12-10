use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn count_gaps(input: &[i32]) -> (i32, i32) {
    let mut input = input.to_owned();
    input.sort();
    let mut ones = 0;
    let mut threes = 1; // the computer is exactly 3 jolts higher than the last one
    for diff in [0] // the wall starts at 0J
        .iter()
        .chain(input.iter())
        .zip(input.iter())
        .map(|(a, b)| b - a)
    {
        match diff {
            1 => ones += 1,
            3 => threes += 1,
            _ => panic!("Unknown difference"),
        }
    }

    (ones, threes)
}

#[aoc(day10, part1)]
fn solve_part1(input: &[i32]) -> i32 {
    let (ones, threes) = count_gaps(input);
    ones * threes
}

#[aoc(day10, part2)]
fn solve_part2(input: &[i32]) -> u64 {
    let mut input: Vec<u64> = [0].iter().chain(input.iter()).map(|x| *x as u64).collect();
    input.sort();

    fn valid_arrangements(list: &[u64]) -> u64 {
        if list.len() <= 2 {
            1
        } else {
            let first = list[0];
            let second = list[1];
            let third = list[2];
            

            ...1, 2, 3...
        }
    }

    input[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(count_gaps(&[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]), (7, 5));
    }
    #[test]
    fn part1_example2() {
        assert_eq!(
            count_gaps(&[
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3,
            ]),
            (22, 10)
        );
    }
    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2(&[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]), 8);
    }
    #[test]
    fn part2_example2() {
        assert_eq!(
            solve_part2(&[
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3,
            ]),
            19208
        );
    }
}
