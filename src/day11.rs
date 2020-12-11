use aoc_runner_derive::{aoc, aoc_generator};
use core::convert::TryInto;
use vec2d::{Coord, Size, Vec2D};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

#[aoc_generator(day11)]
fn input_generator(input: &str) -> Vec2D<Seat> {
    let mut width = None;

    let vec: Vec<Seat> = input
        .lines()
        .flat_map(|x| {
            if let Some(width) = width {
                assert_eq!(width, x.len());
            } else {
                width = Some(x.len());
            }
            x.chars().map(|c| match c {
                '.' => Seat::Floor,
                'L' => Seat::Empty,
                '#' => Seat::Occupied,
                _ => panic!("Bad input"),
            })
        })
        .collect();

    Vec2D::from_vec(Size::new(width.unwrap(), vec.len() / width.unwrap()), vec).unwrap()
}

fn count_occupied_coords(
    i: impl Iterator<Item = (Option<usize>, Option<usize>)>,
    grid: &Vec2D<Seat>,
) -> usize {
    i.map(|p| {
        match p {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        }
        .map(|(x, y)| Coord::new(x, y))
        .and_then(|x| grid.get(x))
    })
    .map(|x| match x {
        Some(Seat::Occupied) => 1,
        _ => 0,
    })
    .sum()
}

fn checked_add(u: usize, d: i8) -> Option<usize> {
    if d >= 0 {
        d.try_into().ok().and_then(|d| u.checked_add(d))
    } else {
        (-d).try_into().ok().and_then(|d| u.checked_sub(d))
    }
}

fn count_neighbors(c: Coord, grid: &Vec2D<Seat>) -> usize {
    count_occupied_coords(
        DIRECTIONS
            .iter()
            .map(|(dx, dy)| (checked_add(c.x, *dx), checked_add(c.y, *dy))),
        grid,
    )
}

const DIRECTIONS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn count_sights(c: Coord, grid: &Vec2D<Seat>) -> usize {
    fn fly(c: Coord, d: (i8, i8), grid: &Vec2D<Seat>) -> (Option<usize>, Option<usize>) {
        fn add(c: Coord, d: (i8, i8)) -> Option<Coord> {
            checked_add(c.x, d.0).and_then(|x| checked_add(c.y, d.1).map(|y| Coord::new(x, y)))
        }

        let mut glider = add(c, d);

        while let Some(c) = glider {
            if let Some(v) = grid.get(c) {
                if *v != Seat::Floor {
                    return (Some(c.x), Some(c.y));
                }
                glider = add(c, d);
            } else {
                return (None, None);
            }
        }

        (None, None)
    }

    count_occupied_coords(
        DIRECTIONS.iter().map(|(dx, dy)| (fly(c, (*dx, *dy), grid))),
        grid,
    )
}

fn transition_1(current: Seat, neighbors: usize) -> (bool, Seat) {
    match (current, neighbors) {
        (Seat::Empty, 0) => (true, Seat::Occupied),
        (Seat::Occupied, 4..=8) => (true, Seat::Empty),
        _ => (false, current),
    }
}

fn transition_2(current: Seat, neighbors: usize) -> (bool, Seat) {
    match (current, neighbors) {
        (Seat::Empty, 0) => (true, Seat::Occupied),
        (Seat::Occupied, 5..=8) => (true, Seat::Empty),
        _ => (false, current),
    }
}

fn step<N, T>(src: &Vec2D<Seat>, dest: &mut Vec2D<Seat>, n: N, t: T) -> bool
where
    N: Fn(Coord, &Vec2D<Seat>) -> usize,
    T: Fn(Seat, usize) -> (bool, Seat),
{
    let mut changed = false;
    for ((changed_pt, src_pt), (_, dest_pt)) in src
        .iter()
        .map(|(coord, value)| t(*value, n(coord, src)))
        .zip(dest.iter_mut())
    {
        if changed_pt {
            changed = true;
        }
        *dest_pt = src_pt;
    }

    changed
}

fn run_till_stable<N, T>(input: &Vec2D<Seat>, n: N, t: T) -> Vec2D<Seat>
where
    N: Copy + Fn(Coord, &Vec2D<Seat>) -> usize,
    T: Copy + Fn(Seat, usize) -> (bool, Seat),
{
    let mut a = input.clone();
    let mut b = input.clone();

    while step(&a, &mut b, n, t) {
        core::mem::swap(&mut a, &mut b);
    }

    b
}

fn count_occupied(i: impl Iterator<Item = Seat>) -> usize {
    i.map(|x| match x {
        Seat::Occupied => 1,
        _ => 0,
    })
    .sum()
}

#[aoc(day11, part1)]
fn solve_part1(input: &Vec2D<Seat>) -> usize {
    count_occupied(
        run_till_stable(input, count_neighbors, transition_1)
            .iter()
            .map(|(_, x)| *x),
    )
}

#[aoc(day11, part2)]
fn solve_part2(input: &Vec2D<Seat>) -> usize {
    count_occupied(
        run_till_stable(input, count_sights, transition_2)
            .iter()
            .map(|(_, x)| *x),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_neighbors_1() {
        let a = input_generator(
            "L#LLL
#LLLL
LLLLL",
        );

        assert_eq!(count_neighbors(Coord::new(0, 0), &a), 2);
    }

    #[test]
    fn test_count_neighbors_2() {
        let a = input_generator(
            "L#LLL
#L#LL
L#LLL",
        );

        assert_eq!(count_neighbors(Coord::new(1, 1), &a), 4);
    }

    fn assert_steps_to(a: &str, b: &str) {
        let a = input_generator(a);
        let b = input_generator(b);
        let mut res = Vec2D::from_example(a.size(), &Seat::Floor);

        step(&a, &mut res, count_neighbors, transition_1);
        assert_eq!(res, b);
    }

    #[test]
    fn part1_step_1() {
        assert_steps_to(
            "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
            "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
        );
    }

    #[test]
    fn part1_step_2() {
        assert_steps_to(
            "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
            "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##
",
        );
    }
    #[test]
    fn part1_step_4() {
        assert_steps_to(
            "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##",
            "#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##",
        );
    }
    #[test]
    fn part1_step_5() {
        assert_steps_to(
            "#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##",
            "#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##
",
        );
    }
    #[test]
    fn part1_step_6() {
        assert_steps_to(
            "#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##",
            "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##",
        );
    }
    #[test]
    fn part1_step_7() {
        assert_steps_to(
            "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##",
            "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##",
        );
    }
    #[test]
    fn part1() {
        assert_eq!(
            solve_part1(&input_generator(
                "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            )),
            37
        );
    }

    #[test]
    fn part_2() {
        let inputs = [
            "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
            "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
            "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#",
            "#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#",
            "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#",
            "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#",
            "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#",
            "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#",
        ];

        for (a, b) in inputs.iter().zip(inputs.iter().skip(1)) {
            let a = input_generator(a);
            let b = input_generator(b);
            let mut res = Vec2D::from_example(a.size(), &Seat::Floor);

            step(&a, &mut res, count_sights, transition_2);
            assert_eq!(res, b);
        }
    }
}
