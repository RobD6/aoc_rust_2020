
#[aoc(day18, part1)]
pub fn solve_part1(input: &str) -> u64 {
    let mut sum = 0;

    for line in input.lines() {
        let mut parser = Parser::new(line.trim());

        sum += get_value(&mut parser)
    }
    sum
}

fn get_value(parser: &mut Parser) -> u64 {
    let mut sum = 0;
    let mut op = Op::Add;

    loop {
        let tok = match parser.get_next_token() {
            Some(x) => {x}
            None => {break;}
        };

        if tok == ")" {
            break;
        }
        else if tok == "*" {
            op = Op::Mul;
        }
        else if tok == "+" {
            op = Op::Add;
        }
        else
        {
            let num;
            if tok == "(" {
                num = get_value(parser);
            }
            else
            {
                num = tok.parse().expect(&format!("Couldn't parse int {}", tok));
            }

            sum = match op {
                Op::Add => {sum + num}
                Op::Mul => {sum * num}
            }
        }
    }

    sum
}

struct Parser {
    input: String,
    pos: usize
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        Parser {input: input.to_string(), pos: 0}
    }

    pub fn get_next_token(&mut self) -> Option<&str> {
        if self.pos >= self.input.len() {
            return None;
        }
        let mut tok_start = self.pos;
        while self.input.as_bytes()[tok_start] as char == ' ' {
            tok_start += 1;
        }

        let mut tok_end = tok_start + 1;

        if self.input.as_bytes()[tok_start] as char >= '0' && self.input.as_bytes()[tok_start] as char <= '9' {
            while tok_end < self.input.len() && self.input.as_bytes()[tok_end] as char >= '0' && self.input.as_bytes()[tok_end] as char <= '9' {
                tok_end += 1;
            }
        }

        self.pos = tok_end;

        return Some(&self.input[tok_start..tok_end]);
    }
}

enum Op {
    Add,
    Mul
}



#[test]
fn day2_part1_test1() {
    assert_eq!(solve_part1("1 + 2 * 3 + 4 * 5 + 6"), 71);
    assert_eq!(solve_part1("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    assert_eq!(solve_part1("2 * 3 + (4 * 5)"), 26);
}
