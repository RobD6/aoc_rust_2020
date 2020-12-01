#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    for i in 0..input.len(){
        let num = input[i];
        for j in i..input.len() {
            let num2 = input[j];

            if num+num2 == 2020 {
                return num * num2;
            }
        }
    }

    unreachable!("No solution!");
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    for i in 0..input.len(){
        let num = input[i];
        for j in i..input.len() {
            let num2 = input[j];

            for k in j..input.len() {
                let num3 = input[k];
                if num+num2+num3 == 2020 {
                    return num * num2 * num3;
                }
            }
        }
    }

    unreachable!("No solution!");
}