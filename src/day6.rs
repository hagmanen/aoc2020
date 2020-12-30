/*
Day 6, part 1: 6457
Day 6, part 2: 3260
*/

pub fn day6() {
    let contents = std::fs::read_to_string("input6.txt").expect("Failed to read file");

    let mut answers = std::collections::BTreeSet::new();
    let mut sum: usize = 0;
    for line in contents.lines() {
        if line.is_empty() {
            sum += answers.len();
            answers.clear();
            continue;
        }
        for c in line.chars() {
            answers.insert(c);
        }
    }
    sum += answers.len();
    println!("Day 6, part 1: {}", sum);

    sum = 0;
    let mut first = true;
    answers.clear();
    for line in contents.lines() {
        if line.is_empty() {
            first = true;
            sum += answers.len();
            answers.clear();
            continue;
        }
        if first {
            first = false;
            for c in line.chars() {
                answers.insert(c);
            }
        } else {
            let mut new_answers = std::collections::BTreeSet::new();
            for c in line.chars().filter(|c| answers.contains(c)) {
                new_answers.insert(c);
            }
            answers = new_answers;
        }
    }
    sum += answers.len();
    println!("Day 6, part 2: {}", sum);
}
