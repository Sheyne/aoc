use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Square {
    Empty,
    Tree,
}

#[derive(Debug, PartialEq, Eq)]
struct Line {
    squares: Vec<Square>,
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| Line {
            squares: line
                .chars()
                .map(|c| match c {
                    '.' => Square::Empty,
                    '#' => Square::Tree,
                    _ => panic!(),
                })
                .collect(),
        })
        .collect()
}

fn check_slope(input: &[Line], rise: usize, run: usize) -> usize {
    let mut position = 0;
    let len = input[0].squares.len();

    let mut get_position = || {
        let p = position;
        position = (position + run) % len;
        p
    };

    input
        .iter()
        .step_by(rise)
        .map(|line| line.squares[get_position()])
        .filter(|x| *x == Square::Tree)
        .count()
}

#[aoc(day3, part1)]
fn solve_part1(input: &[Line]) -> usize {
    check_slope(input, 1, 3)
}

#[aoc(day3, part2)]
fn solve_part2(input: &[Line]) -> usize {
    check_slope(input, 1, 1)
        * check_slope(input, 1, 3)
        * check_slope(input, 1, 5)
        * check_slope(input, 1, 7)
        * check_slope(input, 2, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = input_generator(
            "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
        );

        assert_eq!(solve_part1(&input), 7);
    }

    #[test]
    fn part2_example() {
        let input = input_generator(
            "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
        );

        assert_eq!(solve_part2(&input), 336);
    }
}
