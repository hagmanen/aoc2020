use regex::Regex;

struct Line {
    min: u32,
    max: u32,
    ch: String,
    password: String,
}

fn parse_lines(lines: std::str::Lines) -> Vec<Line> {
    let mut result: Vec<Line> = Vec::new();
    // 3-4 j: tjjj
    let rg = Regex::new(r"(\d+)-(\d+) ([a-z]): (.*)").unwrap();
    for line in lines {
        let m = rg.captures(line).unwrap();
        result.push(Line {
            min: m[1].parse().unwrap(),
            max: m[2].parse().unwrap(),
            ch: m[3].to_string(),
            password: m[4].to_string(),
        });
    }
    result
}

fn valid_password(line: &Line) -> bool {
    let re = Regex::new(&line.ch).unwrap();
    let count = re.find_iter(&line.password).count();
    line.min as usize <= count && count <= line.max as usize
}

fn valid_password2(line: &Line) -> bool {
    (line.password.chars().nth((line.min - 1) as usize) == line.ch.chars().next())
        ^ (line.password.chars().nth((line.max - 1) as usize) == line.ch.chars().next())
}

/*
Day 2, part 1: 483
Day 2, part 2: 482
*/

pub fn day2() {
    let contents: String = std::fs::read_to_string("input2.txt").expect("Failed to read file");
    let lines_raw: std::str::Lines = contents.lines();
    let lines: Vec<Line> = parse_lines(lines_raw);

    let mut valid = 0;
    for line in &lines {
        if valid_password(line) {
            valid += 1;
        }
    }
    println!("Day 2, part 1: {}", valid);

    valid = 0;
    for line in &lines {
        if valid_password2(line) {
            valid += 1;
        }
    }
    println!("Day 2, part 2: {}", valid);
}
