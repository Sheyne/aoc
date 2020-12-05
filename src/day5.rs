use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn input_generator(input: &str) -> String {
    input.to_owned()
}

fn get_row_col(s: &[u8]) -> (i32, i32) {
    let row = &s[..7];
    let col = &s[7..];

    fn custom_from_bin(inp: &[u8], letter: u8) -> i32 {
        inp.iter()
            .fold(0, |acc, num| acc * 2 + if *num == letter { 1 } else { 0 })
    }

    let row = custom_from_bin(row, b'B');
    let col = custom_from_bin(col, b'R');

    (row, col)
}

fn get_id(s: &[u8]) -> i32 {
    let (row, col) = get_row_col(s);
    row * 8 + col
}

#[aoc(day5, part1)]
fn solve_part1(input: &str) -> i32 {
    input
        .as_bytes()
        .chunks(11)
        .map(|x| get_id(&x[..10]))
        .max()
        .unwrap()
}

#[aoc(day5, part2)]
fn solve_part2(input: &str) -> i32 {
    let mut min = 10000;
    let mut max = 0;
    let mut sum = 0;

    for v in input.as_bytes().chunks(11).map(|x| get_id(&x[..10])) {
        min = v.min(min);
        max = v.max(max);
        sum += v;
    }

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
