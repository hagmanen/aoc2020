fn part(input: &str, turns: i64) -> i64 {
    let mut mem: std::collections::HashMap<i64, i64> = std::collections::HashMap::new();
    let mut last_nr: i64 = 0;
    let mut start: i64 = 0;

    for line in input.lines().next().unwrap().split(',') {
        last_nr = line.parse().unwrap();
        start += 1;
        mem.insert(last_nr, start);
    }

    mem.remove(&last_nr);
    let mut last_last: i64;
    for turn in start..turns {
        last_last = last_nr;
        last_nr = turn - mem.get(&last_nr).or(Option::Some(&turn)).unwrap();
        *mem.entry(last_last).or_insert(turn) = turn;
    }
    last_nr
}
/*
Day 15, part 1: 610
Day 15, part 2: 1407
*/

pub fn day15() {
    let contents = std::fs::read_to_string("input15.txt").expect("Failed to read file");

    println!("Day 15, part 1: {}", part(&contents, 2020));
    println!("Day 15, part 2: {}", part(&contents, 30000000));
}
