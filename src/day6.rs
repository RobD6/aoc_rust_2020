use std::collections::HashMap;

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut total: u32 = 0;

    let mut cur_group: HashMap<char, u32> = HashMap::new();
    for line in input.lines() {
        let trimmed = line.trim();

        if trimmed.len() == 0 {
            total += cur_group.len() as u32;
            cur_group = HashMap::new();
            continue;
        }

        for cr in trimmed.chars() {
            if !cur_group.contains_key(&cr) {
                cur_group.insert(cr, 1);
            } else {
                cur_group.insert(cr, cur_group[&cr] + 1);
            }
        }
    }

    total += cur_group.len() as u32;

    total
}

fn get_group_count(num: u32, counts: HashMap<char, u32>) -> u32 {
    counts.iter().filter(|(_cr, val)| num == **val).count() as u32
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let mut total = 0;

    let mut cur_group: HashMap<char, u32> = HashMap::new();
    let mut members = 0;
    for line in input.lines() {
        let trimmed = line.trim();

        if trimmed.len() == 0 {
            total += get_group_count(members, cur_group);

            members = 0;
            cur_group = HashMap::new();
            continue;
        }

        for cr in trimmed.chars() {
            if !cur_group.contains_key(&cr) {
                cur_group.insert(cr, 1);
            } else {
                cur_group.insert(cr, cur_group[&cr] + 1);
            }
        }
        members += 1;
    }

    total += get_group_count(members, cur_group);

    total
}

#[test]
fn day6_part1_test1() {
    let test_data = "abc

    a
    b
    c
    
    ab
    ac
    
    a
    a
    a
    a
    
    b";
    assert_eq!(solve_part1(test_data), 11);
}

#[test]
fn day6_part1_test2() {
    let test_data = "abc

    a
    b
    c
    
    ab
    ac
    
    a
    a
    a
    a
    
    b";
    assert_eq!(solve_part2(test_data), 6);
}
