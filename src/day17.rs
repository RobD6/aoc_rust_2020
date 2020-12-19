use std::collections::HashSet;

#[aoc(day17, part1)]
pub fn solve_part1(input: &str) -> u32 {
    solve(input, 3, 6)
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &str) -> u32 {
    solve(input, 4, 6)
}

fn solve(input: &str, dims: usize, cycles: u32) -> u32 {
    let mut state = HashSet::new();

    let mut y = 0;

    for line in input.lines() {
        let mut x = 0;
        for cr in line.trim().chars() {
            match cr {
                '#' => {
                    let mut key = Vec::new();
                    key.push(x);
                    key.push(y);
                    for _i in 2..dims {
                        key.push(0);
                    }

                    state.insert(key);
                }
                _ => {}
            }
            x += 1;
        }
        y += 1;
    }

    let mut min = -1;
    let mut max = y+1;

    println!("After 0 cycle count is {}", state.len());

    for i in 0..cycles {
        state = process(&state, dims, &mut min, &mut max);

        println!("After {} cycle count is {}", i+1, state.len());
    }

    state.len() as u32
}

fn process(state: &HashSet<Vec<i32>>, dims: usize, min: &mut  i32, max: &mut  i32) -> HashSet<Vec<i32>> {
    let mut new_state = HashSet::new();

    process_slice(state, &mut new_state, dims, *min, *max, Vec::new());

    *min -= 1;
    *max += 1;

    new_state
}

fn process_slice(old_state: &HashSet<Vec<i32>>, new_state: &mut HashSet<Vec<i32>>, dims: usize, min: i32, max: i32, partial_coord: Vec<i32>) {
    for i in min..max {
        let mut new_coord = partial_coord.clone();
        new_coord.push(i); 

        if new_coord.len() == dims {
            let neighbours = count_neighbours(old_state, &new_coord, &Vec::new());
            let was_active = old_state.contains(&new_coord);
            let mut new_active = false;
            if was_active && (neighbours == 2 || neighbours == 3) {
                new_active = true;
            }
            else if !was_active && neighbours == 3 {
                new_active = true;
            }

            if new_active {
                new_state.insert(new_coord);
            }
        }
        else
        {
            process_slice(old_state, new_state, dims, min, max, new_coord);
        }
    }
}

fn count_neighbours(state: &HashSet<Vec<i32>>, center: &Vec<i32>, partial_coord: &Vec<i32> ) -> u32 {
    let mut sum = 0;
    let dim = partial_coord.len();
    for i in center[dim]-1..center[dim]+2 {
        let mut new_coord = partial_coord.clone();
        new_coord.push(i);

        if new_coord.len() == center.len() {
            if *center != new_coord && state.contains(&new_coord) {
                sum += 1;
            }
        }
        else
        {
            sum += count_neighbours(state, center, &new_coord);
        }
    }
    sum
}


#[test]
fn day2_part1_test1() {
    assert_eq!(solve(".#.
..#
###", 3, 6), 112);
}

