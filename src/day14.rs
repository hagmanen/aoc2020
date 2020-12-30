use regex::Regex;

fn parse_mask(mask: &str) -> (u64, u64) {
    let mut zero_mask: u64 = 0;
    let mut one_mask: u64 = 0;
    for c in mask.chars() {
        match c {
            '0' => {
                zero_mask <<= 1;
                one_mask <<= 1;
            }
            '1' => {
                zero_mask <<= 1;
                zero_mask |= 1;
                one_mask <<= 1;
                one_mask |= 1;
            }
            'X' => {
                zero_mask <<= 1;
                zero_mask |= 1;
                one_mask <<= 1;
            }
            _ => panic!("Invalid bitmask"),
        }
    }
    (zero_mask, one_mask)
}

fn calc_value(nr: u64, mask: &(u64, u64)) -> u64 {
    let mut result = nr;
    result &= mask.0;
    result |= mask.1;
    result
}

fn part1(instructions: &str) -> u64 {
    let mut mem: std::collections::HashMap<u64, u64> = std::collections::HashMap::new();
    let re_mask: regex::Regex = Regex::new(r"mask = (.+)").unwrap();
    let re_write: regex::Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    let mut mask: (u64, u64) = (0, 0);
    for instruction in instructions.lines() {
        if let Some(m) = re_mask.captures(instruction) {
            mask = parse_mask(&m[1]);
            continue;
        }
        if let Some(m) = re_write.captures(instruction) {
            mem.insert(
                m[1].parse().unwrap(),
                calc_value(m[2].parse().unwrap(), &mask),
            );
            continue;
        }
        panic!("Invalid instruction {}", instruction);
    }
    let mut sum = 0;
    for n in mem {
        sum += n.1;
    }
    sum
}

fn update(
    mem: &mut std::collections::HashMap<u64, u64>,
    addr: u64,
    value: u64,
    permutations: &[u64],
) {
    let float_indexes: u64 = permutations.len() as u64;
    let nr_of_permutations: u64 = 1 << float_indexes;
    let size_mask: u64 = (1 << 36) - 1;
    for i in 0..nr_of_permutations {
        let mut cur_addr = addr;
        for j in 0..float_indexes {
            let bit: u64 = 1 << permutations[j as usize];
            if i & (1 << j) != 0 {
                cur_addr |= bit;
            } else {
                cur_addr &= size_mask & !bit;
            }
        }
        mem.insert(cur_addr, value);
    }
}

fn parse_mask2(mask: &str) -> (u64, Vec<u64>) {
    let mut one_mask: u64 = 0;
    let mut floating: Vec<u64> = Vec::new();
    for (i, c) in mask.chars().enumerate() {
        one_mask <<= 1;
        match c {
            '0' => (),
            '1' => one_mask |= 1,
            'X' => {
                floating.push(35 - i as u64);
            }
            _ => panic!("Invalid bitmask"),
        }
    }
    (one_mask, floating)
}

fn calc_addr(one_mask: u64, addr: u64) -> u64 {
    one_mask | addr
}

fn part2(instructions: &str) -> u64 {
    let mut mem: std::collections::HashMap<u64, u64> = std::collections::HashMap::new();
    let re_mask: regex::Regex = Regex::new(r"mask = (.+)").unwrap();
    let re_write: regex::Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    let mut mask: (u64, Vec<u64>) = (0, Vec::new());
    for instruction in instructions.lines() {
        if let Some(m) = re_mask.captures(instruction) {
            mask = parse_mask2(&m[1]);
            continue;
        }
        if let Some(m) = re_write.captures(instruction) {
            update(
                &mut mem,
                calc_addr(mask.0, m[1].parse().unwrap()),
                m[2].parse().unwrap(),
                &mask.1,
            );
            continue;
        }
        panic!("Invalid instruction {}", instruction);
    }
    let mut sum = 0;
    for n in mem {
        sum += n.1;
    }
    sum
}

/*
Day 14, part 1: 9615006043476
Day 14, part 2: 4275496544925
*/

pub fn day14() {
    let contents = std::fs::read_to_string("input14.txt").expect("Failed to read file");
    println!("Day 14, part 1: {}", part1(&contents));
    println!("Day 14, part 2: {}", part2(&contents));
}
