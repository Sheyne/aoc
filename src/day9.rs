use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<usize> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn check_summing(src: &[usize], target: usize) -> bool {
    for (idx, x) in src.iter().enumerate() {
        for y in src.iter().skip(idx + 1) {
            if x != y && x + y == target {
                return true;
            }
        }
    }

    return false;
}

fn find_first_non_summing(lookbehind: usize, input: &[usize]) -> usize {
    for idx in lookbehind..input.len() {
        if !check_summing(&input[idx - lookbehind..idx], input[idx]) {
            return input[idx];
        }
    }
    panic!("No non-summing")
}

fn find_range_with_sum(target_sum: usize, input: &[usize]) -> &[usize] {
    for (idx, fst) in input.iter().enumerate() {
        let mut sum = *fst;
        for (last_idx, next) in input.iter().enumerate().skip(idx + 1) {
            sum += next;
            if sum == target_sum {
                return &input[idx..=last_idx];
            }
        }
    }

    panic!("No continuous range with sum found");
}

fn find_weakness(lookbehind: usize, input: &[usize]) -> usize {
    let non_summing = find_first_non_summing(lookbehind, input);
    let range = find_range_with_sum(non_summing, input);
    range.iter().min().unwrap() + range.iter().max().unwrap()
}

#[aoc(day9, part1)]
fn solve_part1(input: &[usize]) -> usize {
    find_first_non_summing(25, input)
}

#[aoc(day9, part2)]
fn solve_part2(input: &[usize]) -> usize {
    find_weakness(25, input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            find_first_non_summing(
                5,
                &[
                    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                    309, 576,
                ]
            ),
            127
        );
    }
    #[test]
    fn part2() {
        assert_eq!(
            find_weakness(
                5,
                &[
                    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                    309, 576,
                ]
            ),
            62
        );
    }
}
