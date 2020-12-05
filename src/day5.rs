use std::collections::HashSet;

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> u32 {
    input.lines().map(
        |line| 
            get_seat_id(line.trim())).max().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let mut occupied = HashSet::new();
    for line in input.lines() {
        occupied.insert(get_seat_id(line.trim()));
    }

    for i in 0.. {
        //Not completely optimal, but quick enough...
        if occupied.contains(&(i-1)) && 
            occupied.contains(&(i+1)) && 
            !occupied.contains(&i) {
            return i;
        }
    }

    unreachable!("Failed to find unnocupied seat!")
}

fn get_seat_id(input: &str) -> u32 {
    let mut row: u32 = 0;
    let mut seat: u32 = 0;

    for char in input.chars() {
        match char {
            'F' => {row = row << 1;}
            'B' => {row = (row << 1) | 1;}
            'L' => {seat = seat << 1;}
            'R' => {seat = (seat << 1) | 1;}
            _ => {unreachable!("Unknown char");}
        }
    }

    row * 8 + seat
}

#[test]
fn day5_part1_test1()
{
    assert_eq!(get_seat_id("BFFFBBFRRR"), 567);
    assert_eq!(get_seat_id("FFFBBBFRRR"), 119);
    assert_eq!(get_seat_id("BBFFBBFRLL"), 820);
}