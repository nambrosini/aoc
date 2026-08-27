use std::collections::HashMap;
use std::sync::LazyLock;

use regex::Regex;

static FIELD_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\w+):(\S+)\s*").unwrap());
static HCL_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^#(\d|[a-z]){6}$").unwrap());
static PID_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d{9}$").unwrap());

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn parse(input: &str) -> Vec<HashMap<String, String>> {
    input
        .split("\n\n")
        .map(|s| {
            FIELD_RE
                .captures_iter(s)
                .map(|c| (c[1].to_owned(), c[2].to_owned()))
                .collect()
        })
        .collect()
}

fn has_required_fields(passport: &HashMap<String, String>) -> bool {
    REQUIRED_FIELDS.iter().all(|f| passport.contains_key(*f))
}

fn is_valid(passport: &HashMap<String, String>) -> bool {
    if !has_required_fields(passport) {
        return false;
    }

    let byr: u32 = passport["byr"].parse().unwrap();
    if !(1920..=2020).contains(&byr) {
        return false;
    }

    let iyr: u32 = passport["iyr"].parse().unwrap();
    if !(2010..=2020).contains(&iyr) {
        return false;
    }

    let eyr: u32 = passport["eyr"].parse().unwrap();
    if !(2020..=2030).contains(&eyr) {
        return false;
    }

    let hgt = &passport["hgt"];
    if hgt.len() < 2 {
        return false;
    }
    let unit = &hgt[hgt.len() - 2..];
    let Ok(val) = hgt[..hgt.len() - 2].parse::<u32>() else {
        return false;
    };

    if unit != "in" && unit != "cm" {
        return false;
    }
    if unit == "in" && !(59..=76).contains(&val) || unit == "cm" && !(150..=193).contains(&val) {
        return false;
    }

    if !HCL_RE.is_match(&passport["hcl"]) {
        return false;
    }

    if !EYE_COLORS.contains(&&passport["ecl"][..]) {
        return false;
    }

    PID_RE.is_match(&passport["pid"])
}

fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|p| has_required_fields(p))
        .count()
}

fn part2(input: &str) -> usize {
    parse(input).iter().filter(|p| is_valid(p)).count()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2020/04.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        assert_eq!(part1(s), 2);
    }

    #[test]
    fn test_part2() {
        let invalid = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        assert_eq!(part2(invalid), 0);

        let valid = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert_eq!(part2(valid), 4);
    }
}
