use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::collections::HashSet;

struct Rule {
    name: String,
    contains: Vec<(usize, String)>,
}

fn extract_name(s: &str) -> &str {
    if s.ends_with(" bags") {
        &s[..s.len() - 5]
    } else if s.ends_with(" bag") {
        &s[..s.len() - 4]
    } else {
        panic!("Wrong suffix")
    }
}

fn extract_name_number(s: &str) -> (usize, &str) {
    let mut parts = s.splitn(2, " ");
    (
        parts.next().unwrap().parse().unwrap(),
        extract_name(parts.next().unwrap()),
    )
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Vec<Rule> {
    input
        .lines()
        .map(|x| {
            assert!(x.ends_with("."));
            let x = &x[..x.len() - 1];
            let mut bits = x.splitn(2, " contain ");
            let name = extract_name(bits.next().unwrap());
            let content = bits.next().unwrap();

            let content = if content == "no other bags" {
                vec![]
            } else {
                content
                    .split(", ")
                    .map(|x| extract_name_number(x))
                    .map(|(num, name)| (num, name.to_owned()))
                    .collect()
            };
            Rule {
                name: name.to_owned(),
                contains: content,
            }
        })
        .collect()
}

#[aoc(day7, part1)]
fn solve_part1(input: &[Rule]) -> usize {
    let mut parents: HashMap<String, Vec<String>> = HashMap::new();

    for rule in input {
        for (_, child) in &rule.contains {
            let p = parents.entry(child.to_owned()).or_insert(vec![]);
            p.push(rule.name.to_owned());
        }
    }

    let mut visited = HashSet::new();

    fn traverse(visited: &mut HashSet<String>, parents: &HashMap<String, Vec<String>>, node: &str) {
        if visited.contains(node) {
            return;
        }
        visited.insert(node.to_owned());
        if let Some(parent_list) = parents.get(node) {
            for parent in parent_list {
                traverse(visited, parents, parent);
            }
        }
    };

    traverse(&mut visited, &parents, "shiny gold");

    visited.len() - 1
}

#[aoc(day7, part2)]
fn solve_part2(input: &[Rule]) -> usize {
    let children: HashMap<_, _> = input
        .iter()
        .map(|x| (x.name.to_owned(), x.contains.clone()))
        .collect();

    fn traverse(children: &HashMap<String, Vec<(usize, String)>>, node: &str) -> usize {
        if let Some(content) = children.get(node) {
            content
                .iter()
                .map(|(repetition, child)| repetition * traverse(children, child))
                .sum::<usize>()
                + 1
        } else {
            1
        }
    }

    traverse(&children, "shiny gold") - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let rules = input_generator(
            "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.",
        );

        assert_eq!(solve_part1(&rules), 4);
    }

    #[test]
    fn part2_example1() {
        let rules = input_generator(
            "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.",
        );

        assert_eq!(solve_part2(&rules), 32);
    }

    #[test]
    fn part2_example2() {
        let rules = input_generator(
            "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.",
        );

        assert_eq!(solve_part2(&rules), 126);
    }
}
