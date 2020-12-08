use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn parse_num(x: &str) -> i32 {
    if x.starts_with("+") { &x[1..] } else { x }
        .parse()
        .unwrap()
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|x| {
            let mut i = x.split(" ");
            let ins = i.next().unwrap();
            let arg = parse_num(i.next().unwrap());
            match ins {
                "nop" => Instruction::Nop(arg),
                "acc" => Instruction::Acc(arg),
                "jmp" => Instruction::Jmp(arg),
                _ => panic!("Unknown instruction"),
            }
        })
        .collect()
}

enum EmulationResult {
    InfiniteLoop(i32),
    Termination(i32),
    InvalidJump,
}

fn emulate(program: &[Instruction]) -> EmulationResult {
    let mut visited = HashSet::new();
    let mut acc = 0;
    let mut iptr = 0;

    loop {
        if visited.contains(&iptr) {
            return EmulationResult::InfiniteLoop(acc);
        }
        visited.insert(iptr);

        let next = program.get(iptr);
        if let Some(next) = next {
            iptr = match next {
                Instruction::Nop(_) => iptr + 1,
                Instruction::Acc(x) => {
                    acc += x;
                    iptr + 1
                }
                Instruction::Jmp(x) => (iptr as i32 + x) as usize,
            }
        } else if iptr == program.len() {
            return EmulationResult::Termination(acc);
        } else {
            return EmulationResult::InvalidJump;
        }
    }
}

#[aoc(day8, part1)]
fn solve_part1(input: &[Instruction]) -> i32 {
    match emulate(input) {
        EmulationResult::InfiniteLoop(x) => x,
        _ => panic!("Invalid result"),
    }
}

#[aoc(day8, part2)]
fn solve_part2(input: &[Instruction]) -> i32 {
    for iptr in 0..input.len() {
        let mut program: Vec<_> = input.iter().map(|x| x.clone()).collect();
        program[iptr] = match input[iptr] {
            Instruction::Nop(x) => Instruction::Jmp(x),
            Instruction::Acc(x) => Instruction::Acc(x),
            Instruction::Jmp(x) => Instruction::Nop(x),
        };

        match emulate(&program[..]) {
            EmulationResult::Termination(x) => {
                return x;
            }
            _ => continue,
        }
    }

    panic!("Should have found solution")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let rules = input_generator(
            "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6",
        );

        assert_eq!(solve_part1(&rules), 5);
    }
    #[test]
    fn part2() {
        let rules = input_generator(
            "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6",
        );

        assert_eq!(solve_part2(&rules), 8);
    }
}
