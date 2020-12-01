
fn run(mut program : Vec<i32>) -> i32 {
    let mut pos : usize = 0;

    loop {
        let p1 : usize = program[pos + 1] as usize;
        let p2 : usize = program[pos + 2] as usize;
        let p3 : usize = program[pos + 3] as usize;
        if program[pos] == 1 {
            program[p3] = program[p1] + program[p2];
            pos += 4;
        } else if program[pos] == 2 {
            program[p3] = program[p1] * program[p2];
            pos += 4;
        } else if program[pos] == 99 {
            return program[0];
        } else {
            panic!("Invalid instruction");
        }
    }
    return 0;
}

pub fn day2() {
    let mut contents = std::fs::read_to_string("input2.txt")
        .expect("Failed to read file");
    contents.pop();
    let mut program : Vec<i32> = contents.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    
    //replace position 1 with the value 12 and replace position 2 with the value 2.
    program[1] = 12;
    program[2] = 2;
    let result = run(program);
    println!("Day 2, part 1: Result: {}", result);

    for noun in 0..100 {
        for verb in 0..100 {
            program = contents.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
            program[1] = noun;
            program[2] = verb;
            let result = run(program);
            if result == 19690720 {
                println!("Day 2, part 2: Result: {}", 100 * noun + verb);
                return;
            }
        }
    }
    panic!("No solution found");
}

