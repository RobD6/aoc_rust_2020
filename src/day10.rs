#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut nums: Vec<u32> = input.lines().map(|line| line.trim().parse().unwrap()).collect();
    nums.sort();

    let mut one_gaps = 0;
    let mut three_gaps = 0;

    if nums[0] == 1 {
        one_gaps += 1;
    }
    else if nums[0] == 3 {
        three_gaps += 1;
    }

    for index in 0..nums.len()-1 {
        let gap = nums[index+1] - nums[index];

        if gap == 1 {
            one_gaps += 1;
        }
        else if gap == 3 {
            three_gaps += 1;
        }
    }

    three_gaps += 1;

    one_gaps * three_gaps
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &str) -> u32 {
0
}

#[test]
fn day10_part1_test1() {
    let test_data = "16
10
15
5
1
11
7
19
6
12
4";
    assert_eq!(solve_part1(test_data), 35);
}

#[test]
fn day10_part1_test2() {
    let test_data = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
    assert_eq!(solve_part1(test_data), 220);
}

#[test]
fn day10_part2_test1() {
    let test_data = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
    assert_eq!(solve_part2(test_data), 0);
}
