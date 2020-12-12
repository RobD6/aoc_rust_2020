#[derive(Clone, Copy, PartialEq)]
enum Direction {
    East = 0,
    South,
    West,
    North
}

fn from_i32(n: i32) -> Option<Direction> {
    match n {
        0 => Some(Direction::East),
        1 => Some(Direction::South),
        2 => Some(Direction::West),
        3 => Some(Direction::North),
        _ => None,
    }
}

enum Instruction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward
}

type Operation = (Instruction, i32);
type State = ((i32, i32), Direction);

#[aoc(day12, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let ops: Vec<Operation> = input.lines().map(|line|{
        let inst = match line.trim().chars().nth(0).unwrap() {
            'N' => Instruction::North,
            'S' => Instruction::South,
            'E' => Instruction::East,
            'W' => Instruction::West,
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            'F' => Instruction::Forward,
            _ => unreachable!("Unknown instruction")
        };
        let num: i32 = line.trim()[1..].parse().unwrap();
        (inst, num)
    }).collect();
    
    let mut state: State = ((0, 0), Direction::East);

    for op in ops {
        process_instruction(&mut state, op)
    }
    
    state.0.0.abs() + state.0.1.abs()
}

fn process_instruction(mut state: &mut State, op: Operation) {
    match op.0 {
        Instruction::North => {
            state.0.1 += op.1;
        }
        Instruction::South => {
            state.0.1 -= op.1;
        }
        Instruction::East => {
            state.0.0 += op.1;
        }
        Instruction::West => {
            state.0.0 -= op.1;
        }
        Instruction::Left => {
            let rot = op.1 / 90;
            let dir = from_i32(((state.1 as i32) + (4-rot)) % 4).unwrap();
            state.1 = dir;
        }
        Instruction::Right => {
            let rot = op.1 / 90;
            let dir = from_i32(((state.1 as i32) + rot) % 4).unwrap();
            state.1 = dir;
        }
        Instruction::Forward => {
            match state.1 {
                Direction::East => { state.0.0 += op.1;}
                Direction::South => {state.0.1 -= op.1;}
                Direction::West => {state.0.0 -= op.1;}
                Direction::North => {state.0.1 += op.1;}
            }
        }
    }
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &str) -> u32 {
    0
}

#[test]
fn day12_part1_test1() {
    let test_data = "F10
N3
F7
R90
F11";
    assert_eq!(solve_part1(test_data), 25);
}

#[test]
fn day12_part1_test2() {
    let test_data = "F10
N3
F7
R90
F11";
    assert_eq!(solve_part2(test_data), 286);
}
