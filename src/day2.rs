use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::{tag, take, take_while, take_while1},
    combinator::map_res,
    IResult,
};
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Eq)]
struct Row {
    rule: RangeInclusive<usize>,
    letter: char,
    password: String,
}

impl Row {
    fn is_valid_part_1(&self) -> bool {
        let num_inst = self.password.chars().filter(|c| *c == self.letter).count();
        self.rule.contains(&num_inst)
    }
    fn is_valid_part_2(&self) -> bool {
        let position_a = self.password.as_bytes()[*self.rule.start() - 1];
        let match_a = position_a as char == self.letter;

        let position_a = self.password.as_bytes()[*self.rule.end() - 1];
        let match_b = position_a as char == self.letter;

        match_a as u8 + match_b as u8 == 1
    }
}

fn get_alpha(input: &str) -> IResult<&str, char> {
    let (input, letters) = take(1usize)(input)?;
    let letter = letters.chars().next().unwrap();
    if letter.is_alphabetic() {
        Ok((input, letter))
    } else {
        Err(nom::Err::Error(nom::error::Error {
            code: nom::error::ErrorKind::Alpha,
            input: letters,
        }))
    }
}

fn get_usize(input: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_digit(10)), str::parse)(input)
}

fn parse_row(input: &str) -> IResult<&str, Row> {
    let (input, min) = get_usize(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, max) = get_usize(input)?;
    let (input, _) = take_while1(|c: char| c.is_whitespace())(input)?;
    let (input, letter) = get_alpha(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = take_while(|c: char| c.is_whitespace())(input)?;
    let (input, password) = take_while1(|c: char| c.is_alphabetic())(input)?;
    Ok((
        input,
        Row {
            rule: min..=max,
            letter: letter,
            password: password.to_owned(),
        },
    ))
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Row> {
    input
        .lines()
        .map(|line| parse_row(line).unwrap().1)
        .collect()
}

#[aoc(day2, part1)]
fn solve_part1(input: &[Row]) -> usize {
    input.iter().filter(|r| r.is_valid_part_1()).count()
}

#[aoc(day2, part2)]
fn solve_part2(input: &[Row]) -> usize {
    input.iter().filter(|r| r.is_valid_part_2()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let (rest, row) = parse_row("18-19 p: fvpkgfkfjgwllqwhrjd").unwrap();

        assert_eq!(rest, "");

        assert_eq!(
            row,
            Row {
                rule: 18..=19,
                letter: 'p',
                password: "fvpkgfkfjgwllqwhrjd".to_owned(),
            }
        )
    }

    #[test]
    fn part1_example() {
        assert_eq!(
            solve_part1(&[
                Row {
                    rule: 1..=3,
                    letter: 'a',
                    password: "abcde".to_owned()
                },
                Row {
                    rule: 1..=3,
                    letter: 'b',
                    password: "cdefg".to_owned()
                },
                Row {
                    rule: 2..=9,
                    letter: 'c',
                    password: "ccccccccc".to_owned()
                },
            ]),
            2
        );
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(
            true,
            Row {
                rule: 1..=3,
                letter: 'a',
                password: "abcde".to_owned(),
            }
            .is_valid_part_2(),
        );
    }
    #[test]
    fn part2_example_2() {
        assert_eq!(
            false,
            Row {
                rule: 1..=3,
                letter: 'b',
                password: "cdefg".to_owned(),
            }
            .is_valid_part_2(),
        );
    }
    #[test]
    fn part2_example_3() {
        assert_eq!(
            false,
            Row {
                rule: 2..=9,
                letter: 'c',
                password: "ccccccccc".to_owned(),
            }
            .is_valid_part_2(),
        );
    }
}
