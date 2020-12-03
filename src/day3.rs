#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<bool>> {
    input.lines().map(|line| line.chars().
        map(|cr| cr == '#').collect()).collect()
}

#[aoc(day3, part1)]
pub fn part1 (input: &Vec<Vec<bool>>) -> u32{
    count_collisions(input, (3, 1))
}

#[aoc(day3, part2)]
pub fn part2 (input: &Vec<Vec<bool>>) -> u32{
    count_collisions(input, (1, 1)) *
    count_collisions(input, (3, 1)) * 
    count_collisions(input, (5, 1)) * 
    count_collisions(input, (7, 1)) * 
    count_collisions(input, (1, 2))
}

pub fn count_collisions(input: &Vec<Vec<bool>>, step: (usize, usize)) -> u32 {
    let mut coords = (0,0);
    let mut tree_count = 0;

    while coords.1 < input.len() {
        let x_coord = coords.0 % input[0].len();

        if input[coords.1][x_coord] == true {
            tree_count += 1;
        }

        coords = (coords.0 + step.0, coords.1 + step.1);
    }

    tree_count
}

#[test]
fn day3_part1_test1()
{
    let test_data = 
"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    assert_eq!(part1(&input_generator(test_data)), 7);
}

#[test]
fn day3_part2_test1()
{
    let test_data = 
    "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
    assert_eq!(part2(&input_generator(test_data)), 336);
}