use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;
use itertools::Itertools;

lazy_static! {
static ref SRC_REGEX: Regex = Regex::new(r"^([\w\s]+) bags").unwrap();
static ref DEST_REGEX: Regex = Regex::new(r"^ (\d+) ([\w\s]+) bag").unwrap();
}

struct BagRules {
    colour: String,
    colours: HashMap<String, u32>,
    processed: bool,
    contains_gold: bool
}

impl  BagRules {
    pub fn new(line: &str) -> BagRules {
        let (src, dests) = line.split("contain").collect_tuple().unwrap();

        let colour = SRC_REGEX.captures(src).
            expect("failed to parse bag colour")[1].trim().to_owned();

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
        
        BagRules{
            colour: colour,
            colours: map,
            processed: false,
            contains_gold: false
        }
    }
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut rules = Vec::new();
    for line in input.lines() {
        let rule = BagRules::new(line);
        rules.push(rule)
    }

    0
}

// #[aoc(day6, part2)]
// pub fn solve_part2(input: &str) -> u32 {
// }

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

// #[test]
// fn day6_part1_test2() {
//     let test_data = "abc

//     a
//     b
//     c
    
//     ab
//     ac
    
//     a
//     a
//     a
//     a
    
//     b";
//     assert_eq!(solve_part2(test_data), 6);
// }