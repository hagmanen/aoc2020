fn is_valid(nr: u64, preamble: &std::collections::VecDeque<&u64>) -> bool {
    for i in 0..preamble.len() {
        for j in i..preamble.len() {
            if **preamble.get(i).unwrap() + **preamble.get(j).unwrap() == nr {
                return true;
            }
        }
    }
    false
}

fn validate_numbers(numbers: &[u64]) -> usize {
    let mut i: usize = 0;
    let mut preamble = std::collections::VecDeque::new();
    while i < 25 {
        preamble.push_back(numbers.get(i).unwrap());
        i += 1;
    }
    while i < numbers.len() {
        let current = numbers.get(i).unwrap();
        if !is_valid(*current, &preamble) {
            return i;
        }
        i += 1;
        preamble.push_back(current);
        preamble.pop_front();
    }
    i
}

/*
Day 9, part 1: 133015568
Day 9, part 2: 16107959
*/

pub fn day9() {
    let contents = std::fs::read_to_string("input9.txt").expect("Failed to read file");
    let numbers: Vec<u64> = contents
        .lines()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    let invalid_number = *numbers.get(validate_numbers(&numbers)).unwrap();
    println!("Day 9, part 1: {}", invalid_number);

    let mut low: usize = 0;
    let mut high: usize = 0;
    let mut sum: u64 = numbers[low];
    while sum != invalid_number {
        if sum < invalid_number {
            high += 1;
            sum += numbers[high];
        } else {
            sum -= numbers[low];
            low += 1;
        }
    }
    let min = numbers[low..high].iter().min().unwrap();
    let max = numbers[low..high].iter().max().unwrap();
    println!("Day 9, part 2: {}", min + max);
}
