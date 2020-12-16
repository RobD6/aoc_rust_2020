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

fn is_valid_for_any_rule(rules: &Vec<Rule>, num: u32) -> bool {
    for rule in rules {
        for range in &rule.ranges {
            if num >= range.0 && num <= range.1 {
                return true;
            }
        }
    }

    false
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let parsed = parse_input(input);
    let rules = parsed.0;
    let my_ticket = parsed.1;
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
