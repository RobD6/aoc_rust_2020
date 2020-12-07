use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::cell::Cell;
use std::collections::HashMap;

lazy_static! {
    static ref SRC_REGEX: Regex = Regex::new(r"^([\w\s]+) bags").unwrap();
    static ref DEST_REGEX: Regex = Regex::new(r"^ (\d+) ([\w\s]+) bag").unwrap();
}

struct BagRules {
    colour: String,
    colours: HashMap<String, u32>,
    processed: Cell<bool>,
    contains_gold: Cell<bool>,
}

impl BagRules {
    pub fn new(line: &str) -> BagRules {
        let (src, dests) = line.split("contain").collect_tuple().unwrap();

        let colour = SRC_REGEX.captures(src).expect("failed to parse bag colour")[1]
            .trim()
            .to_owned();

        let dest_rules = dests.split(',');
        let mut map = HashMap::new();

        for dest in dest_rules {
            let caps = DEST_REGEX.captures(dest);
            match caps {
                Some(cap) => {
                    let num: u32 = cap[1].parse().unwrap();
                    map.insert(cap[2].trim().to_owned(), num);
                }
                _ => {}
            }
        }

        BagRules {
            colour: colour,
            colours: map,
            processed: Cell::new(false),
            contains_gold: Cell::new(false),
        }
    }

    pub fn does_contains_gold(&self, rules: &HashMap<String, BagRules>) -> bool {
        if self.processed.get() {
            return self.contains_gold.get();
        } else {
            if self.colours.contains_key("shiny gold") {
                self.contains_gold.set(true);
            } else if self.colours.is_empty() {
                self.contains_gold.set(false);
            } else {
                let mut has_gold = false;

                for (col, _count) in &self.colours {
                    has_gold = has_gold || rules.get(col).unwrap().does_contains_gold(rules);
                }

                self.contains_gold.set(has_gold);
            }
        }

        self.processed.set(true);
        self.contains_gold.get()
    }

    pub fn get_contents(&self, rules: &HashMap<String, BagRules>) -> u32 {
        let mut total = 0;

        for (col, count) in &self.colours {
            total = total + count + count * rules[col].get_contents(rules);
        }

        total
    }

    pub fn copy(&self) -> BagRules {
        BagRules {
            colour: self.colour.to_owned(),
            colours: self.colours.to_owned(),
            processed: Cell::new(self.processed.get()),
            contains_gold: Cell::new(self.contains_gold.get()),
        }
    }
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut rules = HashMap::new();
    for line in input.lines() {
        let rule = BagRules::new(line);
        rules.insert(rule.colour.to_owned(), rule.copy());
    }

    rules
        .iter()
        .filter(|rule| rule.1.does_contains_gold(&rules))
        .count() as u32
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let mut rules = HashMap::new();
    for line in input.lines() {
        let rule = BagRules::new(line);
        rules.insert(rule.colour.to_owned(), rule.copy());
    }

    rules["shiny gold"].get_contents(&rules)
}

#[test]
fn day7_part1_test1() {
    let test_data = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    assert_eq!(solve_part1(test_data), 4);
}

#[test]
fn day7_part2_test1() {
    let test_data = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    assert_eq!(solve_part2(test_data), 32);
}

#[test]
fn day7_part2_test2() {
    let test_data = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
    assert_eq!(solve_part2(test_data), 126);
}
