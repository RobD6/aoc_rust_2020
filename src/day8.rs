use crate::console::Console;
use crate::console::Op;
use std::collections::VecDeque;
use std::{collections::HashSet, fs};

#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut cons = Console::new(input);
    let mut used = HashSet::new();

    while !used.contains(&cons.get_pc()) {
        used.insert(cons.get_pc());
        cons.step();
    }

    cons.get_acc()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let orig = Console::new(input);
    let mut queue = VecDeque::new();

    queue.push_back(orig);
    let mut is_first_run = true;

    while queue.len() > 0 {
        let mut cons = queue.pop_front().unwrap();
        let mut used = HashSet::new();
        println!("Trying new run...");

        while !cons.terminated() && !used.contains(&cons.get_pc()) {
            if is_first_run && (cons.get_next_op() == Op::Jmp || cons.get_next_op() == Op::Nop) {
                let mut copy = cons.clone();
                let new_op = if cons.get_next_op() == Op::Jmp {
                    Op::Nop
                } else {
                    Op::Jmp
                };
                copy.set_next_op(new_op);
                queue.push_back(copy);
            }

            used.insert(cons.get_pc());
            cons.step();
        }

        is_first_run = false;

        if cons.terminated() {
            return cons.get_acc();
        }
    }

    unreachable!()
}

#[test]
fn day2_part1_test1() {
    let test_data = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    assert_eq!(solve_part1(test_data), 5);
}

#[test]
fn day8_part2_test1() {
    let test_data = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    assert_eq!(solve_part2(test_data), 8);
}

#[test]
fn day8_part2_test2() {
    let test_data =
        fs::read_to_string(r"input/2020/day8.txt").expect("Something went wrong reading the file");
    assert_eq!(solve_part2(&test_data), 8);
}
