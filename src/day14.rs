use std::collections::HashMap;
use regex::Regex;

#[aoc(day14, part1)]
pub fn solve_part1(input: &str) -> u64 {
    let mask_regex = Regex::new(r"^mask = ([X01]+)").unwrap();
    let mem_regex = Regex::new(r"^mem\[(\d+)\] = (\d+)").unwrap();
    let mut mem = HashMap::new();

    let mut and_mask: u64 = !0;
    let mut or_mask = 0u64;

    for line in input.lines() {
        if mask_regex.is_match(line.trim()) {
            let masks = parse_mask(&mask_regex.captures(line.trim()).unwrap()[1]);
            and_mask = masks.0;
            or_mask = masks.1;
        }
        else if (mem_regex.is_match(line.trim())) {
            let caps = mem_regex.captures(line.trim()).unwrap();

            let addr: u64 = caps[1].parse().unwrap();
            let mut val: u64 = caps[2].parse().unwrap();

            val = val & and_mask;
            val = val | or_mask;

            mem.insert(addr, val);
        }
    }

    let mut sum = 0u64;
    for (addr, val) in mem {
        sum += val;
    }

    sum
}

fn parse_mask(mask: &str) -> (u64, u64) {
    let mut and_mask: u64 = !0;
    let mut or_mask = 0u64;
    let num_bits = mask.len();
    
    for (index, cr) in mask.chars().enumerate() {
        match cr {
            '0' => {
                and_mask &= !(1 << (num_bits - index - 1));
            }
            '1' => {
                or_mask |= (1 << (num_bits - index - 1));
            }
            'X' => {

            }
            _ => unreachable!()
        }
    }

    (and_mask, or_mask)
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &str) -> u64 {
0
}

#[test]
fn day14_part1_test1() {
    let test_data = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    assert_eq!(solve_part1(test_data), 165);
}

#[test]
fn day2_part1_test2() {
    let test_data = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
    assert_eq!(solve_part2(test_data), 1);
}
