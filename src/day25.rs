fn transform(subject: u64, loop_size: u64) -> u64 {
    let mut value: u64 = 1;
    for _ in 0..loop_size {
        value *= subject;
        value %= 20201227;
    }
    value
}

fn calc_loop_size(public_key: u64) -> u64 {
    let mut value: u64 = 1;
    let subject: u64 = 7;
    let mut loop_size: u64 = 0;
    while value != public_key {
        value *= subject;
        value %= 20201227;
        loop_size += 1;
    }
    loop_size
}

fn part1(key1: u64, key2: u64) -> u64 {
    let key2_loop_size = calc_loop_size(key2);
    transform(key1, key2_loop_size)
}

/*
Day 25, part 1: 354320
Day 25, part 2: Work hard
*/

pub fn day25() {
    let contents = std::fs::read_to_string("input25.txt").expect("Failed to read file");
    let mut keys = contents.lines().map(|x| x.parse::<u64>().unwrap());

    println!(
        "Day 25, part 1: {}",
        part1(keys.next().unwrap(), keys.next().unwrap())
    );
    println!("Day 25, part 2: Work hard");
}
