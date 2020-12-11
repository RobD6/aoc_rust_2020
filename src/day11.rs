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
    if pos.0 > 0 && pos.1 > 0 &&
        pos.1 < state.len() &&
        pos.0 < state[pos.1].len() {
            return state[pos.1][pos.0] == Seat::Full;
        }

    return false;
}

pub fn count_occupied_neighbours(state: &State, pos: (usize, usize)) -> u32 {
    let mut count = 0;

    for x in 0..3 {
        for y in 0..3 {
            if x != 1 || y != 1 {
                if count_cell(state, (pos.0 + x, pos.1 + y)) {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn step_state(state: &mut State) -> bool {
    let mut changed = false;

    for y in 0..state.len() {
        for x in 0..state[y].len() {
            let seat = state[y][x];
            match seat {
                Seat::Empty => {
                    if count_occupied_neighbours(state, (x, y)) == 0 {
                        state[y][x] = Seat::Full;
                        changed = true;
                    }
                },
                Seat::Full => {
                    if count_occupied_neighbours(state, (x, y)) >= 4 {
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
    state.iter().map(|column| column.iter().filter(|seat| **seat == Seat::Full).count()).count() as u32
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut state = input_generator(input);

    let mut changed = true;

    while changed {
        println!("Stepped");
        changed = step_state(&mut state);
    }

    count_occupied_seats(state)
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> u32 {
    0
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
    let test_data = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
    assert_eq!(solve_part2(test_data), 1);
}
