use regex::Regex;

#[derive(Clone, Copy, PartialEq)]
pub enum Op {
    Nop,
    Acc,
    Jmp,
}
#[derive(Clone)]
pub struct Console {
    prog: Vec<(Op, i32)>,
    acc: i32,
    pc: usize,
}

impl Console {
    pub fn new(input: &str) -> Console {
        let mut prog = Vec::new();
        let inst_regex = Regex::new(r"^(\w+) ([+-]\d+)").unwrap();

        for line in input.lines() {
            let caps = inst_regex
                .captures(line.trim())
                .expect("Failed to parse input line {}");
            let op = match &caps[1] {
                "nop" => Op::Nop,
                "acc" => Op::Acc,
                "jmp" => Op::Jmp,
                _ => unreachable!(),
            };

            let param: i32 = caps[2].parse().expect("Failed to parse parameter");

            prog.push((op, param));
        }

        Console {
            prog,
            acc: 0,
            pc: 0,
        }
    }

    pub fn get_pc(&self) -> usize {
        self.pc
    }

    pub fn get_acc(&self) -> i32 {
        self.acc
    }

    pub fn step(&mut self) {
        let (op, param) = &self.prog[self.pc];

        match op {
            Op::Nop => {
                self.pc += 1;
            }
            Op::Jmp => {
                self.pc = ((self.pc as i32) + *param) as usize;
            }
            Op::Acc => {
                self.acc += param;
                self.pc += 1;
            }
        }
    }

    pub fn get_next_op(&self) -> Op {
        self.prog[self.pc].0
    }

    pub fn set_next_op(&mut self, op: Op) {
        self.prog[self.pc].0 = op;
    }

    pub fn terminated(&self) -> bool {
        self.pc >= self.prog.len()
    }
}
