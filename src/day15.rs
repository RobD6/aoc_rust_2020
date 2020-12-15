use std::collections::HashMap;

pub fn solve(input: &str, number: usize) -> u32 {
    let mut nums: Vec<u32> = input.split(',').map(|nums_str| nums_str.parse::<u32>().unwrap()).collect();

    let mut map = HashMap::new();

    let mut current_num = 0;

    for (index, num) in nums.iter().enumerate() {
        if index < nums.len()-1 {
            map.insert(*num, index);
        }
        else {
            current_num = *num;
        }
    }

    for index in nums.len()-1..number-1 {
        let last_num = current_num;

        current_num = match map.get(&current_num) {
            Some(x) => {(index - *x) as u32}
            None => {0}
        };

        map.insert(last_num, index);
    }

    current_num
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &str) -> u32 {
    solve(input, 2020)
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &str) -> u32 {
    solve(input, 30000000)
}

#[test]
fn day2_part1_test1() {
    let test_data = "0,3,6";
    assert_eq!(solve(test_data, 2020), 436);
    assert_eq!(solve(test_data, 30000000), 175594);
}

#[test]
fn day2_part1_test2() {
    let test_data = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
    assert_eq!(solve_part2(test_data), 1);
}
