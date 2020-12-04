use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Record> {
    input
        .split_terminator("\n\n")
        .map(|x| {
            Record(
                x.split_whitespace()
                    .map(|x| {
                        let mut bits = x.split(":");
                        (
                            bits.next().unwrap().to_owned(),
                            bits.next().unwrap().to_owned(),
                        )
                    })
                    .collect(),
            )
        })
        .collect()
}

#[derive(Debug)]
struct Record(HashMap<String, String>);

impl Record {
    pub fn contains_required_fields(&self) -> bool {
        self.0.contains_key("byr")
            && self.0.contains_key("iyr")
            && self.0.contains_key("eyr")
            && self.0.contains_key("hgt")
            && self.0.contains_key("hcl")
            && self.0.contains_key("ecl")
            && self.0.contains_key("pid")
        // && self.0.contains_key("cid")
    }

    pub fn is_valid(&self) -> bool {
        if self.contains_required_fields() {
            let byr = self.0["byr"].parse().unwrap();
            let iyr = self.0["iyr"].parse().unwrap();
            let eyr = self.0["eyr"].parse().unwrap();
            let hgt = &self.0["hgt"];
            let hcl = &self.0["hcl"];
            let ecl = &self.0["ecl"];
            let pid = &self.0["pid"];
            let hgt_valid = if hgt.ends_with("cm") {
                let hgt: usize = hgt[..hgt.len() - 2].parse().unwrap();
                hgt >= 150 && hgt <= 193
            } else if hgt.ends_with("in") {
                let hgt: usize = hgt[..hgt.len() - 2].parse().unwrap();
                hgt >= 59 && hgt <= 76
            } else {
                false
            };

            let ecl_valid = match &ecl[..] {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            };

            1920 <= byr
                && byr <= 2002
                && 2010 <= iyr
                && iyr <= 2020
                && 2020 <= eyr
                && eyr <= 2030
                && hgt_valid
                && hcl.len() == 7
                && hcl.starts_with("#")
                && hcl[1..].chars().all(|x| x.is_digit(16))
                && ecl_valid
                && pid.len() == 9
                && pid.chars().all(|x| x.is_digit(10))
        } else {
            false
        }
    }
}

#[aoc(day4, part1)]
fn solve_part1(input: &[Record]) -> usize {
    input
        .iter()
        .filter(|x| x.contains_required_fields())
        .count()
}

#[aoc(day4, part2)]
fn solve_part2(input: &[Record]) -> usize {
    input.iter().filter(|x| x.is_valid()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = input_generator(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in",
        );

        assert_eq!(solve_part1(&input), 2);
    }

    #[test]
    fn part2_example() {
        assert!(input_generator(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
        )
        .iter()
        .all(|x| x.is_valid()));

        assert!(input_generator(
            "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"
        )
        .iter()
        .all(|x| !x.is_valid()));
    }
}
