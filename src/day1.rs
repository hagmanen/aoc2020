
fn part1(numbers: &Vec<i32>) {
    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                println!("Day 1, part 1: {}", numbers[i] * numbers[j]);
                return;
            }
        }
    }
}

fn part2(numbers: &Vec<i32>) {
    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            for k in j..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    println!("Day 1, part 2: {}", numbers[i] * numbers[j] * numbers[k]);
                    return;
                }
            }
        }
    }
}

/*
Day 1, part 1: 806656
Day 1, part 2: 230608320
*/

pub fn day1() {
    let contents = std::fs::read_to_string("input1.txt")
        .expect("Failed to read file");
    let numbers : Vec<i32> = contents.lines().map(|s| s.parse().expect("parse error")).collect();
    part1(&numbers);
    part2(&numbers);
}