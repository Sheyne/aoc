use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day11, part1)]
fn solve_part1(input: &[i32]) -> i32 {
    todo!()
}

#[aoc(day11, part2)]
fn solve_part2(input: &[i32]) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {}
    #[test]
    fn part2() {}
}
