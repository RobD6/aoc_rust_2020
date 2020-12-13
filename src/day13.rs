#[aoc(day13, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut start_time: u32 = 0;
    let mut bus_times: Vec<u32> = Vec::new();
    for (index, line) in input.lines().enumerate() {
        match index {
            0 => {
                start_time = line.trim().parse().unwrap();
            }
            1 => {
                bus_times = line.trim().split(',').filter(|st| st != &"x").map(|num|num.parse().unwrap()).collect();
            }
            _ => unreachable!()
        }
    }

    let mut least_wait = u32::MAX;
    let mut best_bus= 0;

    for time in bus_times {
        let wait = time - (start_time % time);
        if wait < least_wait {
            least_wait = wait;
            best_bus = time;
        }
    }

    best_bus * least_wait
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &str) -> u64 {
    let mut consts: Vec<(u64, u64)> = Vec::new();
    for (index, val) in input.lines().nth(1).unwrap().trim().split(',').enumerate() {
        match val.parse::<u64>() {
            Ok(x) => {
                consts.push((x, index as u64));
            }
            Err(_) => {}
        }
    }
    
    let mut inc = consts[0].0;
    let mut sum = consts[0].0;

    for i in 1..consts.len() {
        while ((sum + consts[i].1) % consts[i].0) != 0 {
            sum += inc;
        }

        inc *= consts[i].0;
    }

    sum
}

#[test]
fn day13_part1_test1() {
    let test_data = "939
7,13,x,x,59,x,31,19";
    assert_eq!(solve_part1(test_data), 295);
}

#[test]
fn day13_part1_test2() {
    let test_data = "939
7,13,x,x,59,x,31,19";
    assert_eq!(solve_part2(test_data), 1068781);
    
}
