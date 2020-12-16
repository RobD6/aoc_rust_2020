use std::collections::HashMap;
use regex::Regex;

struct Rule {
    name: String,
    ranges: Vec<(u32, u32)>
}

fn parse_input(input: &str) -> (Vec<Rule>, Vec<u32>, Vec<Vec<u32>>) {
    let mut list = Vec::new();
    let mut your_ticket = Vec::new();
    let mut other_tickets = Vec::new();

    let reg = Regex::new(r"(\w+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    let mut next_line_is_your_ticket = false;
    let mut is_in_nearby_tickets = false;

    for line in input.lines() {
        if next_line_is_your_ticket {
            your_ticket = line.trim().split(',').map(|num| num.parse().unwrap()).collect();
            next_line_is_your_ticket = false;
        } 
        else if is_in_nearby_tickets {
            other_tickets.push(line.trim().split(',').map(|num| num.parse().unwrap()).collect());
        }
        else if reg.is_match(line.trim()) {
            let caps = reg.captures(line.trim()).unwrap();
            let mut ranges = Vec::new();
            ranges.push((caps[2].parse().unwrap(), caps[3].parse().unwrap()));
            ranges.push((caps[4].parse().unwrap(), caps[5].parse().unwrap()));
            
            let rule = Rule {name: caps[1].to_string(), ranges};
            list.push(rule);
        }
        else if line.trim().contains("your ticket") {
            next_line_is_your_ticket = true;
        }
        else if line.trim().contains("nearby tickets") {
            is_in_nearby_tickets = true;
        }
    }

    (list, your_ticket, other_tickets)
}

fn is_valid_for_rule(rule: &Rule, num: u32) -> bool {
    for range in &rule.ranges {
        if num >= range.0 && num <= range.1 {
            return true;
        }
    }

    return false;
}

fn is_valid_for_any_rule(rules: &Vec<Rule>, num: u32) -> bool {
    for rule in rules {
        if is_valid_for_rule(rule, num) {
            return true;
        }
    }

    false
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let parsed = parse_input(input);
    let rules = parsed.0;
    let other_tickets = parsed.2;

    let mut error_rate = 0;
    for ticket in other_tickets {
        for num in ticket {
            if !is_valid_for_any_rule(&rules, num) {
                error_rate += num;
            }
        }
    }

    error_rate
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let parsed = parse_input(input);
    let rules = parsed.0;
    let my_ticket = parsed.1;
    let other_tickets = parsed.2;
    let mut valid_tickets = Vec::new();

    for ticket in other_tickets {
        let mut valid = true;
        for num in &ticket {
            if !is_valid_for_any_rule(&rules, *num) {
                valid = false;
                break;
            }
        }

        if valid {
            //Keep rust happy...
            let mut tick_copy = Vec::new();
            for num in ticket {
                tick_copy.push(num);
            }
            valid_tickets.push(tick_copy);
        }
    }

    let mut possible_field_rules = HashMap::new();
    let mut all_rules = Vec::new();
    for rule in 0..rules.len() {
        all_rules.push(rule);
    }

    for field in 0..my_ticket.len() {
        possible_field_rules.insert(field, all_rules.clone());
    }

    for ticket in valid_tickets {
        for (field, num) in ticket.iter().enumerate() {
            for rule in rules.iter().enumerate() {
                if possible_field_rules.get(&field).unwrap().contains(&rule.0) {
                    if !is_valid_for_rule(rule.1, *num)
                    {
                        let mut rules_list = possible_field_rules.get_mut(&field).unwrap();
                        rules_list.remove(rules_list.iter().position(|x| *x == rule.0).expect("rule not found"));
                    }
                }
            }
        }
    }

    let mut needs_reduction = true;

    while needs_reduction {
        let mut used_rules = Vec::new();
        for possible in &possible_field_rules {
            if possible.1.len() == 1 {
                used_rules.push(possible.1[0]);
            }
        }
    }


    0
}


#[test]
fn day2_part1_test1() {
    let test_data = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
    assert_eq!(solve_part1(test_data), 71);
}

#[test]
fn day2_part2_test1() {
    let test_data = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
    assert_eq!(solve_part2(test_data), 71);
}