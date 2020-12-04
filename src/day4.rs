use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;

pub fn input_generator(input: &str) -> Vec<HashMap<&str, &str>> {
    let mut res = Vec::new();
    let mut entry = HashMap::new();

    for full_line in input.lines() {
        let line = full_line.trim();

        if line.len() == 0 {
            res.push(entry);
            entry = HashMap::new();
            continue;
        }

        let pairs = line.split(" ");

        for pair in pairs {
            let (key, val) = pair.split(":").collect_tuple().expect("Failed to parse pair");
            entry.insert(key, val);
        }
    }

    res.push(entry);

    res
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let map = input_generator(input);

    map.iter().filter(
        |entry| 
            (entry.len() == 8 || 
             (entry.len() == 7 && !entry.contains_key("cid"))) &&
             check_entry(entry)
        ).count() as u32
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let map = input_generator(input);

    map.iter().filter(
        |entry| 
            entry.len() == 8 || 
            (entry.len() == 7 && !entry.contains_key("cid"))
        ).count() as u32
}

fn check_entry(entry: &HashMap<&str, &str>) -> bool {
    for (key, val) in entry {
        match *key {
            "byr" => {
                let yr: u32 = val.parse().expect("Failed to parse year");
                if yr < 1920 || yr > 2002 {
                    println!("Invalid byr: {}", val);
                    return false;
                }
            }

            "iyr" => {
                let yr: u32 = val.parse().expect("Failed to parse year");
                if yr < 2010 || yr > 2020 {
                    println!("Invalid iyr: {}", val);
                    return false;
                }
            }

            "eyr" => {
                let yr: u32 = val.parse().expect("Failed to parse year");
                if yr < 2020 || yr > 2030 {
                    println!("Invalid eyr: {}", val);
                    return false;
                }
            }

            "hgt" => {
                let hgt_regex = Regex::new(r"^(\d+)([a-z]+)").unwrap();
                if !hgt_regex.is_match(val) {return false;}
                let caps = hgt_regex.captures(val).unwrap();
                let num: u32 = caps[1].parse().unwrap();
                match &caps[2] {
                    "in" => {
                        if num < 59 || num > 76 {
                            println!("Invalid hgt: {}", val);
                            return false;
                        }
                    }
                    "cm" => {
                        if num < 150 || num > 193 {
                            println!("Invalid hgt: {}", val);
                            return false;
                    }
                }
                    _ => unreachable!("Unknown height units")
                }
            }
            "hcl" => {
                let hcl_regex = Regex::new(r"^#[a-f0-9]{6}").unwrap();
                if !hcl_regex.is_match(val) {
                    println!("Invalid hcl: {}", val);
                    return false;
                }
            }

            "ecl" => {
                if *val != "amb" && *val != "blu" && *val != "brn" && *val != "gry" &&
                    *val != "grn" && *val != "hzl" && *val != "oth" { 
                        println!("Invalid ecl: {}", val);
                        return false; 
                    }
            }
            "pid" => {
                let pid_regex = Regex::new(r"^[0-9]{9}").unwrap();
                if !pid_regex.is_match(val) {
                    println!("Invalid pid: {}", val);
                    return false;
                }
            }
            "cid" => {continue;}
            _ => unreachable!("Didn't recognise key")
        }
    }
    true
}

#[test]
fn day4_part1_test1()
{
    let test_data = 
"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    assert_eq!(solve_part1(test_data), 2);
}

#[test]
fn day4_part2_test1()
{
    let test_data = 
"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    assert_eq!(solve_part2(test_data), 0);
}

#[test]
fn day4_part2_test2()
{
    let test_data = 
"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    assert_eq!(solve_part2(test_data), 4);
}