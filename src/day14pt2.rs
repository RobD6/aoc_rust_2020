use std::collections::HashMap;
use regex::Regex;

#[aoc(day14, part2)]
pub fn solve_part2(input: &str) -> u64 {
    let mask_regex = Regex::new(r"^mask = ([X01]+)").unwrap();
    let mem_regex = Regex::new(r"^mem\[(\d+)\] = (\d+)").unwrap();
    let mut mem = HashMap::new();

    let mut or_mask = 0u64;
    let mut and_or_list = Vec::new();

    for line in input.lines() {
        if mask_regex.is_match(line.trim()) {
            let masks = parse_mask(&mask_regex.captures(line.trim()).unwrap()[1]);
            or_mask = masks.0;
            and_or_list = masks.1;
        }
        else if (mem_regex.is_match(line.trim())) {
            let caps = mem_regex.captures(line.trim()).unwrap();

            let mut addr: u64 = caps[1].parse().unwrap();
            let mut val: u64 = caps[2].parse().unwrap();

            addr = addr | or_mask;
            for (and, or) in &and_or_list {
                let mut final_addr = addr;
                final_addr |= or;
                final_addr &= and;

                mem.insert(final_addr, val);            
            }

        }
    }

    let mut sum = 0u64;
    for (addr, val) in mem {
        sum += val;
    }

    sum
}

fn parse_mask(mask: &str) -> (u64, Vec<(u64, u64)>) {
    let mut or_mask = 0u64;
    let num_bits = mask.len();
    let mut and_or_list = Vec::new();
    
    for (index, cr) in mask.chars().enumerate() {
        match cr {
            '0' => {
            }
            '1' => {
                or_mask |= (1 << (num_bits - index - 1));
            }
            'X' => {
                let mut new_list = Vec::new();
                if and_or_list.len() == 0 {
                    new_list.push((!(1 << (num_bits - index - 1)),
                                0));
                    new_list.push((!0,
                                1 << (num_bits - index - 1)));
                                
                }
                else
                {
                    for (and, or) in and_or_list {
                        new_list.push((and & !(1 << (num_bits - index - 1)),
                                        or));
                        new_list.push((and,
                                        or | 1 << (num_bits - index - 1)));
                    }
                }
                and_or_list = new_list;
            }
            _ => unreachable!()
        }
    }

    (or_mask, and_or_list)
}


#[test]
fn day14_part2_test1() {
    let test_data = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
    assert_eq!(solve_part2(test_data), 208);
}
