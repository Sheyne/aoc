use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn find_target(input: &[u32], set: &HashSet<u32>, num: u32) -> Option<u32> {
    input
        .iter()
        .filter_map(|x| {
            num.checked_sub(*x)
                .and_then(|diff| set.get(&diff))
                .map(|y| x * y)
        })
        .next()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    let options: HashSet<u32> = input.iter().cloned().collect();

    find_target(input, &options, 2020).unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    let options: HashSet<u32> = input.iter().cloned().collect();

    input
        .iter()
        .filter_map(|x| find_target(input, &options, 2020 - x).map(|prod| prod * x))
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&[1721, 979, 366, 299, 675, 1456]), 514579);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&[1721, 979, 366, 299, 675, 1456]), 241861950);
    }
}
