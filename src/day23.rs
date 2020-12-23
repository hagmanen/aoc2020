type Cups = std::collections::VecDeque<usize>;

fn parse_cups(input: &str) -> Cups {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn result1(mut cups: Cups) -> usize {
    let mut top = cups.pop_front().unwrap();
    while top != 1 {
        cups.push_back(top);
        top = cups.pop_front().unwrap();
    }
    let mut result: usize = 0;
    for cup in cups {
        result = 10 * result + cup;
    }
    result
}

fn move_cups(cups: &mut Cups) {
    let current = cups.pop_front().unwrap();
    let mut destination = if current == 1 { 9 } else { current - 1 };
    let mut picked_cups: Cups = vec![
        cups.pop_front().unwrap(),
        cups.pop_front().unwrap(),
        cups.pop_front().unwrap(),
    ]
    .into_iter()
    .collect();
    while picked_cups.contains(&destination) {
        destination = if destination == 1 { 9 } else { destination - 1 };
    }
    for ix in 0..cups.len() {
        if cups[ix] == destination {
            cups.insert(ix + 1, picked_cups.pop_back().unwrap());
            cups.insert(ix + 1, picked_cups.pop_back().unwrap());
            cups.insert(ix + 1, picked_cups.pop_back().unwrap());
            break;
        }
    }
    cups.push_back(current);
}

fn part1(mut cups: Cups) -> usize {
    let moves = 100;
    for _ in 0..moves {
        move_cups(&mut cups);
    }
    result1(cups)
}

fn part2(mut cups: Cups) -> usize {
    let moves = 10000000;
    let max_initial_cup = cups.len();
    let max_cup: usize = 1000000;
    for cup in max_initial_cup..max_cup + 1 {
        cups.push_back(cup + 1);
    }
    let mut neigbuors: Vec<(usize, usize)> = Vec::new();
    neigbuors.resize(max_cup + 1, (0, 0)); // Use cup id as index, so 0 is not used
    for ix in 0..max_cup {
        let cup = cups[ix];
        let prev = if ix == 0 {
            cups[max_cup - 1]
        } else {
            cups[ix - 1]
        };
        let next = if ix == max_cup - 1 {
            cups[0]
        } else {
            cups[ix + 1]
        };
        neigbuors[cup] = (prev, next);
    }

    let mut current_cup = cups[0];
    for _ in 0..moves {
        // Pick out cups
        let p1 = neigbuors[current_cup].1;
        let p2 = neigbuors[p1].1;
        let p3 = neigbuors[p2].1;
        let next_cup = neigbuors[p3].1;

        // Remove cups
        neigbuors[next_cup].0 = current_cup;
        neigbuors[current_cup].1 = next_cup;

        // Calc destination
        let mut dest = if current_cup == 1 {
            max_cup
        } else {
            current_cup - 1
        };
        while dest == p1 || dest == p2 || dest == p3 {
            dest = if dest == 1 { max_cup } else { dest - 1 };
        }

        // Insert after destination
        let dest_next = neigbuors[dest].1;
        neigbuors[dest].1 = p1;
        neigbuors[p1].0 = dest;
        neigbuors[p3].1 = dest_next;
        neigbuors[dest_next].0 = p1;

        // Calc next current cup
        current_cup = neigbuors[current_cup].1;
    }
    let after1 = neigbuors[1].1;
    let afterthat = neigbuors[after1].1;
    after1 * afterthat
}

/*
Day 23, part 1: 97624853
Day 23, part 2: 664642452305
*/

pub fn day23() {
    let input = "853192647";
    let cups = parse_cups(input);
    println!("Day 23, part 1: {}", part1(cups.clone()));
    println!("Day 23, part 2: {}", part2(cups));
}
