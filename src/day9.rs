use std::{cmp::min, collections::HashMap};

#[aoc(day9, part1)]
pub fn solve_part1(input: &str) -> u64 {
    let window = 25u32;
    find_first_invalid(input, window)
}

fn find_first_invalid(input: &str, window: u32) -> u64 {
    let nums: Vec<u64> = input.lines().map(|line| line.trim().parse().unwrap()).collect();
    
    let mut sum_hash: HashMap<u64, Vec<(u32, u32)>> = HashMap::new();

    for i in 0..nums.len() {
        for j in i+1..min(i+window as usize, nums.len()) {
            let sum = (nums[i]+nums[j]) as u64;
            if sum_hash.contains_key(&sum) {
                sum_hash.get_mut(&sum).unwrap().push((i as u32, j as u32));
            }
            else {
                let mut new_vec = Vec::new();
                new_vec.push((i as u32, j as u32));
                sum_hash.insert(sum, new_vec);
            }
        }
    }

    for index in (window as usize)..nums.len() {
        let u_index = index as u32;
        let val = nums[index];
        let vals = sum_hash.get(&val);

        match vals {
            Some(list) => {
                let mut found_valid = false;
                for entry in list {
                    
                    if (entry.0 >= u_index - window && entry.0 < u_index) &&
                        (entry.1 >= u_index - window && entry.1 < u_index) {
                            found_valid = true;
                            break;
                        }
                }

                if !found_valid {
                    return val;
                }
            }
            None => {return val;}
        }
    }

    unreachable!();
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &str) -> u64 {
    let target = find_first_invalid(input, 25);

    let mut window = (0, 0);
    let nums: Vec<u64> = input.lines().map(|line| line.trim().parse().unwrap()).collect();
    let mut sum = nums[0];

    while sum != target {
        if sum < target {
            window.1 += 1;
            sum += nums[window.1];
        }
        else {
            window.0 += 1;
            sum -= nums[window.0-1];
        }
    }
    let mut min: u64 = nums[window.0];
    let mut max: u64 = nums[window.1];

    for val in &nums[window.0 .. window.1] {
        min = std::cmp::min(*val, min);
        max = std::cmp::max(*val, max);
    }
    min + max
}

#[test]
fn day2_part9_test1() {
    let test_data = "35
    20
    15
    25
    47
    40
    62
    55
    65
    95
    102
    117
    150
    182
    127
    219
    299
    277
    309
    576";
    assert_eq!(find_first_invalid(test_data, 5), 127);
}

// #[test]
// fn day2_part9_test2() {
//     let test_data = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
//     assert_eq!(solve_part2(test_data), 1);
// }
