use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<Vec<String>> {
    input
        .split_terminator("\n\n")
        .map(|x| x.lines().map(|x| x.to_owned()).collect())
        .collect()
}

fn count_yeses(input: &[String]) -> u32 {
    input
        .iter()
        .flat_map(|x| x.as_bytes().iter().map(|x| 1u32 << (x - b'a')))
        .fold(0, |acc, x| acc | x)
        .count_ones()
}

#[aoc(day6, part1)]
fn solve_part1(input: &[Vec<String>]) -> u32 {
    input.iter().map(|x| count_yeses(x)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(count_yeses(&["abc".to_owned()]), 3);
    }
    #[test]
    fn part1_example2() {
        assert_eq!(
            count_yeses(&["a".to_owned(), "b".to_owned(), "c".to_owned()]),
            3
        );
    }
    #[test]
    fn part1_example3() {
        assert_eq!(count_yeses(&["ab".to_owned(), "ac".to_owned()]), 3);
    }
    #[test]
    fn part1_example4() {
        assert_eq!(
            count_yeses(&[
                "a".to_owned(),
                "a".to_owned(),
                "a".to_owned(),
                "a".to_owned()
            ]),
            1
        );
    }
    #[test]
    fn part1_example5() {
        assert_eq!(count_yeses(&["b".to_owned()]), 1);
    }
}
