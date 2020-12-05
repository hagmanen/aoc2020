
fn ticket_seat_id(line : &str, occupied : &mut std::collections::BTreeSet<u32>) -> u32 {
    let mut id : u32 = 0;
    for c in line.chars() {
        id = id << 1;
        if c == 'B' || c == 'R' {
            id += 1;
        }
    }
    occupied.insert(id);
    return id;
}

/*
Day 5, part 1: 835
Day 5, part 2: 649
*/

pub fn day5() {
    let contents = std::fs::read_to_string("input5.txt").expect("Failed to read file");
    let mut occupied = std::collections::BTreeSet::new();
    let max_seat_id = contents.lines().map(|l| ticket_seat_id(l, &mut occupied)).max().unwrap();
    println!("Day 5, part 1: {}", max_seat_id);

    let mut it = occupied.iter();
    let mut last = it.next().unwrap();
    for seat in it {
        if *seat > last + 1 {
            println!("Day 5, part 2: {}", last + 1);
        }
        last = seat;
    }
}
