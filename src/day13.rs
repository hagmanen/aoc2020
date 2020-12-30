fn parse_busses(line: &str) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    for bus in line.split(',') {
        if bus != "x" {
            result.push(bus.parse().unwrap());
        } else {
            result.push(0);
        }
    }
    result
}

fn part1(time: u64, busses: &[u64]) -> u64 {
    let mut first_dep = u64::MAX;
    let mut first_bus = 0;
    for bus in busses {
        if *bus > 0 {
            let dep = bus - (time % bus);
            if dep < first_dep {
                first_dep = dep;
                first_bus = *bus;
            }
        }
    }
    first_bus * first_dep
}

fn part2(busses: &[u64]) -> u64 {
    let mut timestamp: u64 = 0;
    let mut inc: u64 = 1;
    for (offset, bus) in busses.iter().enumerate() {
        if *bus > 0 {
            loop {
                timestamp += inc;
                if (timestamp + (offset as u64)) % bus == 0 {
                    break;
                }
            }
            inc *= bus
        }
    }
    timestamp
}

/*
Day 13, part 1: 2238
Day 13, part 2: 560214575859998
*/

pub fn day13() {
    let contents = std::fs::read_to_string("input13.txt").expect("Failed to read file");
    let mut lines = contents.lines();
    let time: u64 = lines.next().unwrap().parse().unwrap();
    let busses: Vec<u64> = parse_busses(lines.next().unwrap());
    println!("Day 13, part 1: {}", part1(time, &busses));
    println!("Day 13, part 2: {}", part2(&busses));
}
