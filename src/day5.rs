use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|x| x.to_owned()).collect()
}

fn get_row_col(s: &str) -> (i32, i32) {
    let row = &s[..7];
    let col = &s[7..];

    fn custom_from_bin(inp: &str, letter: char) -> i32 {
        inp.chars().fold(0, |acc, num| acc * 2 + if num == letter { 1 } else { 0 })
    }

    let row = custom_from_bin(row, 'B');
    let col = custom_from_bin(col, 'R');

    (row, col)
}

fn get_id(s: &str) -> i32 {
    let (row, col) = get_row_col(s);
    row * 8 + col
}

#[aoc(day5, part1)]
fn solve_part1(input: &[String]) -> i32 {
    input.iter().map(|x| get_id(x)).max().unwrap()
}

#[aoc(day5, part2)]
fn solve_part2(input: &[String]) -> i32 {
    let min = input.iter().map(|x| get_id(x)).min().unwrap();
    let max = input.iter().map(|x| get_id(x)).max().unwrap();
    let sum = input.iter().map(|x| get_id(x)).fold(0, |acc, x| acc + x);

    (max * (max + 1)) / 2 - (min * (min - 1)) / 2 - sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(get_row_col("BFFFBBFRRR"), (70, 7));
    }
    #[test]
    fn part1_example2() {
        assert_eq!(get_row_col("FFFBBBFRRR"), (14, 7));
    }
    #[test]
    fn part1_example3() {
        assert_eq!(get_row_col("BBFFBBFRLL"), (102, 4));
    }
}
