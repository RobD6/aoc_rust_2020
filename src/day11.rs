use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
pub enum Seat {
    Empty,
    Full,
    Floor
}

type State = Vec<Vec<Seat>>;

pub fn input_generator(input: &str) -> State {
    input.lines().map(
        |line| {
            line.trim().chars().map(|cr| {
                match cr {
                    'L' => Seat::Empty,
                    '.' => Seat::Floor,
                    '#' => Seat::Full,
                    _ => unreachable!("Unknown seat state")
                }
            }).collect()
        }).collect()
}

fn count_cell(state: &State, pos: (usize, usize)) -> bool {
    if pos.0 >= 0 && pos.1 >= 0 &&
        pos.1 < state.len() &&
        pos.0 < state[pos.1].len() {
            return state[pos.1][pos.0] == Seat::Full;
        }

    return false;
}

pub fn count_occupied_neighbours(state: &State, pos: (usize, usize)) -> u32 {
    let mut count = 0;

    for x in -1i32..2 {
        for y in -1i32..2 {
            if x != 0 || y != 0 {
                if count_cell(state, ((pos.0 as i32 + x) as usize, (pos.1 as i32 + y) as usize)) {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn step_state(state: &mut State) -> bool {
    let mut changed = false;
    let state_copy = state.clone();

    for y in 0..state.len() {
        for x in 0..state[y].len() {
            let seat = state[y][x];
            match seat {
                Seat::Empty => {
                    if count_occupied_neighbours(&state_copy, (x, y)) == 0 {
                        state[y][x] = Seat::Full;
                        changed = true;
                    }
                },
                Seat::Full => {
                    if count_occupied_neighbours(&state_copy, (x, y)) >= 4 {
                        state[y][x] = Seat::Empty;
                        changed = true;
                    }
                },
                _ => {}
            }
        }
    }

    changed
}

fn count_occupied_seats(state: State) -> u32 {
    state.iter().map(|column| 
        column.iter().filter(|seat| 
            **seat == Seat::Full).count() as u32).sum()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut state = input_generator(input);

    let mut changed = true;

    while changed {
        changed = step_state(&mut state);
    }

    count_occupied_seats(state)
}

type CellCache = HashMap<((usize, usize), (i32, i32)), (bool, (usize, usize))>;

fn get_seat_in_dir(state: &State, pos: (usize, usize), dir: (i32, i32), mut cellcache: &mut CellCache) -> Seat {
    //See if it's in the cache
    match cellcache.get(&(pos, dir)) {
        Some((has_val, new_pos)) => {
            if *has_val {return state[new_pos.1][new_pos.0];}
            return Seat::Floor;
        }
        None => {}
    }

    let moved = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);

    if moved.0 >= 0 && moved.1 >= 0 &&
        moved.1 < state.len() as i32 &&
        moved.0 < state[moved.1 as usize].len() as i32 {
            if state[moved.1 as usize][moved.0 as usize] == Seat::Floor {
                let result = get_seat_in_dir(state, (moved.0 as usize, moved.1 as usize), dir, &mut cellcache);
                cellcache.insert((pos, dir), cellcache[&((moved.0 as usize, moved.1 as usize), dir)]);
                return result;
            }
            else
            {
                cellcache.insert((pos, dir), (true, (moved.0 as usize, moved.1 as usize)));
            }
    }
    else {
        cellcache.insert((pos, dir), (false, (0, 0)));
    }

    Seat::Floor
}

pub fn count_occupied_neighbours2(state: &State, pos: (usize, usize), mut cellcache: &mut CellCache ) -> u32 {
    let mut count = 0;

    for x in -1..2 {
        for y in -1..2 {
            if x != 0 || y != 0 {
                if get_seat_in_dir(state, (pos.0, pos.1), (x, y), &mut cellcache) == Seat::Full {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn step_state2(state: &mut State, mut cellcache: &mut CellCache ) -> bool {
    let mut changed = false;
    let state_copy = state.clone();

    for y in 0..state.len() {
        for x in 0..state[y].len() {
            let seat = state[y][x];
            match seat {
                Seat::Empty => {
                    if count_occupied_neighbours2(&state_copy, (x, y), &mut cellcache) == 0 {
                        state[y][x] = Seat::Full;
                        changed = true;
                    }
                },
                Seat::Full => {
                    if count_occupied_neighbours2(&state_copy, (x, y), &mut cellcache) >= 5 {
                        state[y][x] = Seat::Empty;
                        changed = true;
                    }
                },
                _ => {}
            }
        }
    }

    changed
}

fn print_state(state: &State) {
    for row in state {
        for cell in row {
            print!( "{}", match cell{
                Seat::Floor => ".",
                Seat::Full => "#",
                Seat::Empty => "L"
            });
        }
        print!("\n");
    }
    print!("\n\n");
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let mut state = input_generator(input);
    let mut cellcache: CellCache = HashMap::new();

    let mut changed = true;

    // print_state(&state);

    while changed {
        changed = step_state2(&mut state, &mut cellcache);
        // print_state(&state);
    }

    count_occupied_seats(state)
}

#[test]
fn day11_part1_test1() {
    let test_data = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    assert_eq!(solve_part1(test_data), 37);
}

#[test]
fn day11_part1_test2() {
    let test_data = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    assert_eq!(solve_part2(test_data), 26);
}
