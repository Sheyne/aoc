use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Instruction {
    Direction(Direction),
    Rotation(Rotation),
    F,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    N,
    S,
    E,
    W,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Rotation {
    L,
    R,
}

use Direction::*;
use Rotation::*;

#[aoc_generator(day12)]
fn input_generator(input: &str) -> Vec<(Instruction, i32)> {
    input
        .lines()
        .map(|x| {
            (
                match x.chars().next().unwrap() {
                    'N' => Instruction::Direction(N),
                    'S' => Instruction::Direction(S),
                    'E' => Instruction::Direction(E),
                    'W' => Instruction::Direction(W),
                    'L' => Instruction::Rotation(L),
                    'R' => Instruction::Rotation(R),
                    'F' => Instruction::F,
                    _ => todo!(),
                },
                x[1..].parse().unwrap(),
            )
        })
        .collect()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Vec2D(i32, i32);

impl Vec2D {
    fn rotate(self, rotation: Rotation) -> Vec2D {
        let Vec2D(n, e) = self;
        match rotation {
            R => Vec2D(-e, n),
            L => Vec2D(n, -e),
        }
    }
}

impl Direction {
    fn rotate(self, rotation: Rotation) -> Direction {
        match (self, rotation) {
            (N, L) => W,
            (W, L) => S,
            (S, L) => E,
            (E, L) => N,
            (N, R) => E,
            (W, R) => N,
            (S, R) => W,
            (E, R) => S,
        }
    }
}

fn follow_1(directions: &[(Instruction, i32)]) -> (i32, i32) {
    let mut facing = E;
    let mut n = 0;
    let mut e = 0;

    for (i, amt) in directions {
        match i {
            Instruction::Rotation(r) => {
                assert_eq!(0, amt % 90);
                let steps = (amt / 90) % 4;
                for _ in 0..steps {
                    facing = facing.rotate(*r);
                }
            }
            Instruction::Direction(d) => match d {
                N => n += amt,
                S => n -= amt,
                E => e += amt,
                W => e -= amt,
            },
            Instruction::F => match facing {
                N => n += amt,
                S => n -= amt,
                E => e += amt,
                W => e -= amt,
            },
        }
    }

    (n, e)
}

fn follow_2(directions: &[(Instruction, i32)]) -> (i32, i32) {
    let mut ship = Vec2D(0, 0);
    let mut way = Vec2D(1, 10);

    for (i, amt) in directions {
        match i {
            Instruction::Rotation(r) => {
                assert_eq!(0, amt % 90);
                let steps = (amt / 90) % 4;
                for _ in 0..steps {
                    way = way.rotate(*r);
                }
            }
            Instruction::Direction(d) => {
                let Vec2D(n, e) = way;
                way = match d {
                    N => Vec2D(n + amt, e),
                    S => Vec2D(n - amt, e),
                    E => Vec2D(n, e + amt),
                    W => Vec2D(n, e - amt),
                }
            }
            Instruction::F => {
                ship.0 += way.0 * amt;
                ship.1 += way.1 * amt;
            }
        }
    }

    (ship.0, ship.1)
}

#[aoc(day12, part1)]
fn solve_part1(input: &[(Instruction, i32)]) -> i32 {
    let (n, e) = follow_1(input);
    n.abs() + e.abs()
}

#[aoc(day12, part2)]
fn solve_part2(input: &[(Instruction, i32)]) -> i32 {
    let (n, e) = follow_2(input);
    n.abs() + e.abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use Direction::*;
    use Rotation::*;

    #[test]
    fn part1() {
        assert_eq!(
            (-8, 17),
            follow_1(&[
                (Instruction::F, 10),
                (Instruction::Direction(N), 3),
                (Instruction::F, 7),
                (Instruction::Rotation(R), 90),
                (Instruction::F, 11)
            ])
        );
    }
    #[test]
    fn part2() {
        assert_eq!(
            (-72, 214),
            follow_2(&[
                (Instruction::F, 10),
                (Instruction::Direction(N), 3),
                (Instruction::F, 7),
                (Instruction::Rotation(R), 90),
                (Instruction::F, 11)
            ])
        );
    }
}
