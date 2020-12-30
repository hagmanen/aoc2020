use std::str::FromStr;

#[derive(Eq, Debug, PartialEq)]
enum Operation {
    ACC,
    JMP,
    NOP,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(input: &str) -> Result<Operation, Self::Err> {
        match input {
            "acc" => Ok(Operation::ACC),
            "jmp" => Ok(Operation::JMP),
            "nop" => Ok(Operation::NOP),
            _ => Err(()),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Instruction {
    op: Operation,
    arg: i32,
}

fn parse_program(text: String) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();
    for line in text.lines() {
        let mut tmp = line.split(' ');
        let op = tmp.next().unwrap();
        let arg = tmp.next().unwrap();
        program.push(Instruction {
            op: Operation::from_str(op).unwrap(),
            arg: arg.parse().unwrap(),
        });
    }
    return program;
}

fn add_int(u: &mut usize, i: &i32) {
    if *i < 0 {
        *u -= (-i) as usize;
    } else {
        *u += *i as usize;
    }
}

fn step(program: &Vec<Instruction>, pos: &mut usize, acc: &mut i32) {
    let inst = &program[*pos];
    match inst.op {
        Operation::ACC => {
            *acc += inst.arg;
            *pos += 1;
        }
        Operation::JMP => add_int(pos, &inst.arg),
        Operation::NOP => *pos += 1,
    }
}

fn run_program(program: &Vec<Instruction>) -> (usize, i32) {
    let mut pos: usize = 0;
    let mut acc: i32 = 0;
    let mut visited: std::collections::HashSet<usize> = std::collections::HashSet::new();
    while !visited.contains(&pos) && pos < program.len() {
        visited.insert(pos);
        step(&program, &mut pos, &mut acc);
    }
    return (pos, acc);
}

/*
Day 8, part 1: 1749
Day 8, part 2: 515
*/

pub fn day8() {
    let contents = std::fs::read_to_string("input8.txt").expect("Failed to read file");
    let mut program = parse_program(contents);
    let (_, acc) = run_program(&program);
    println!("Day 8, part 1: {:?}", acc);

    for i in 0..program.len() {
        if program[i].op == Operation::JMP {
            program[i].op = Operation::NOP;
        } else if program[i].op == Operation::NOP {
            program[i].op = Operation::JMP;
        }

        let (pos, acc) = run_program(&program);
        if pos >= program.len() {
            println!("Day 8, part 2: {}", acc);
            break;
        }

        if program[i].op == Operation::JMP {
            program[i].op = Operation::NOP;
        } else if program[i].op == Operation::NOP {
            program[i].op = Operation::JMP;
        }
    }
}
