use regex::Regex;

#[derive(Debug)]
struct Instruction {
    op: char,
    arg: i32,
}

fn parse_instructions(lines: std::str::Lines) -> Vec<Instruction> {
    let mut result = Vec::new();
    let re: regex::Regex = Regex::new(r"(.)(\d*)").unwrap();
    for line in lines {
        let m = re.captures(line).unwrap();
        result.push(Instruction {
            op: m[1].chars().next().unwrap(),
            arg: m[2].parse().unwrap(),
        });
    }
    return result;
}

fn forward(dir: i32, length: i32, x: &mut i32, y: &mut i32) {
    match dir {
        0 => *x += length,
        90 => *y -= length,
        180 => *x -= length,
        270 => *y += length,
        _ => panic!("Unknown direction"),
    }
}

fn step1(instruction: &Instruction, x: &mut i32, y: &mut i32, dir: &mut i32) {
    match instruction.op {
        'N' => *y -= instruction.arg,
        'S' => *y += instruction.arg,
        'E' => *x += instruction.arg,
        'W' => *x -= instruction.arg,
        'L' => {
            *dir += instruction.arg;
            *dir %= 360;
        }
        'R' => {
            *dir += 360 - instruction.arg;
            *dir %= 360;
        }
        'F' => forward(*dir, instruction.arg, x, y),
        _ => panic!("Unknown operation"),
    }
}

fn part1(instructions: &Vec<Instruction>) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dir: i32 = 0;
    for instruction in instructions {
        step1(&instruction, &mut x, &mut y, &mut dir);
    }
    return x.abs() + y.abs();
}

fn rotate(angle: i32, x: &mut i32, y: &mut i32) {
    let d = (angle + 360) % 360;
    let xx = *x;
    match d {
        0 => (),
        90 => {
            *x = *y;
            *y = -xx;
        }
        180 => {
            *x = -(*x);
            *y = -(*y);
        }
        270 => {
            *x = -(*y);
            *y = xx;
        }
        _ => panic!("Unknown direction"),
    }
}

fn step2(
    instruction: &Instruction,
    pos_x: &mut i32,
    pos_y: &mut i32,
    way_x: &mut i32,
    way_y: &mut i32,
) {
    match instruction.op {
        'N' => *way_y -= instruction.arg,
        'S' => *way_y += instruction.arg,
        'E' => *way_x += instruction.arg,
        'W' => *way_x -= instruction.arg,
        'L' => rotate(instruction.arg, way_x, way_y),
        'R' => rotate(-instruction.arg, way_x, way_y),
        'F' => {
            *pos_x += instruction.arg * (*way_x);
            *pos_y += instruction.arg * (*way_y);
        }
        _ => panic!("Unknown operation"),
    }
}

fn part2(instructions: &Vec<Instruction>) -> i32 {
    let mut pos_x: i32 = 0;
    let mut pos_y: i32 = 0;
    let mut way_x: i32 = 10;
    let mut way_y: i32 = -1;
    for instruction in instructions {
        step2(&instruction, &mut pos_x, &mut pos_y, &mut way_x, &mut way_y);
    }
    return pos_x.abs() + pos_y.abs();
}

/*
Day 12, part 1: 2879
Day 12, part 2: 178986
*/

pub fn day12() {
    let contents = std::fs::read_to_string("input12.txt").expect("Failed to read file");
    let instructions = parse_instructions(contents.lines());
    println!("Day 12, part 1: {}", part1(&instructions));
    println!("Day 12, part 2: {}", part2(&instructions));
}
