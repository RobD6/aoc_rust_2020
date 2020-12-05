use regex::Regex;

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let pass_regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)").unwrap();
    let mut ok_count: u32 = 0;

    for line in input.lines() {
        let caps = pass_regex.captures(line.trim());
        let matches = caps.unwrap();
        let min = matches[1].parse::<usize>().unwrap();
        let max = matches[2].parse::<usize>().unwrap();
        let cr = &matches[3];
        let passwd = &matches[4];

        let count = passwd.matches(cr).count();

        if (min <= count) && (count <= max) {
            ok_count += 1;
        }
    }

    ok_count
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let pass_regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)").unwrap();
    let mut ok_count: u32 = 0;

    for line in input.lines() {
        let caps = pass_regex.captures(line.trim());
        let matches = caps.unwrap();
        let min = matches[1].parse::<usize>().unwrap();
        let max = matches[2].parse::<usize>().unwrap();
        let cr = matches[3].chars().nth(0).unwrap();
        let passwd = &matches[4];

        let mut found = false;

        for (index, test_cr) in passwd.chars().enumerate() {
            if index + 1 == min {
                found = test_cr == cr;
            }

            if index + 1 == max {
                let is_char = test_cr == cr;

                if (found && !is_char) || (!found && is_char) {
                    ok_count += 1;
                }

                break;
            }
        }
    }

    ok_count
}

#[test]
fn day2_part1_test1() {
    let test_data = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
    assert_eq!(solve_part1(test_data), 2);
}

#[test]
fn day2_part1_test2() {
    let test_data = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
    assert_eq!(solve_part2(test_data), 1);
}
