use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|x| x.to_owned()).collect()
}

fn get_row_col(s: &str) -> (i32, i32) {
    let row = &s[..7];
    let col = &s[7..];

    let row = i32::from_str_radix(&row.replace("B", "1").replace("F", "0"), 2).unwrap();
    let col = i32::from_str_radix(&col.replace("R", "1").replace("L", "0"), 2).unwrap();

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
    let ids: Vec<_> = input.iter().map(|x| get_id(x)).collect();

    let min = ids.iter().min().unwrap();
    let max = ids.iter().max().unwrap();
    let sum = ids.iter().fold(0, |acc, x| acc + x);

    (max * (max + 1)) / 2 - (min * (min - 1)) / 2 - sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(convert("BFFFBBFRRR"), (70, 7));
    }
    #[test]
    fn part1_example2() {
        assert_eq!(convert("FFFBBBFRRR"), (14, 7));
    }
    #[test]
    fn part1_example3() {
        assert_eq!(convert("BBFFBBFRLL"), (102, 4));
    }
}
