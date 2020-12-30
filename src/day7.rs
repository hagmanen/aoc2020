use regex::Regex;

#[derive(Hash, Eq, PartialEq, Debug)]
struct BagQuantity {
    name: String,
    nr: u32,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Bag {
    name: String,
    bags: Vec<BagQuantity>,
}

fn parse_bag_quantity(line: &str, re_bag_quantity: &regex::Regex) -> Vec<BagQuantity> {
    let mut result: Vec<BagQuantity> = Vec::new();
    if line == "no other bags." {
        return result;
    }
    for l in line.split(',') {
        let m = re_bag_quantity.captures(l).unwrap();
        result.push(BagQuantity {
            name: m[2].to_string(),
            nr: m[1].parse().unwrap(),
        });
    }
    result
}

fn parse_bag(line: &str, re_bag: &regex::Regex, re_bag_quantity: &regex::Regex) -> Bag {
    let m = re_bag.captures(line).unwrap();
    Bag {
        name: m[1].to_string(),
        bags: parse_bag_quantity(&m[2], re_bag_quantity),
    }
}

fn contains_gold(bag: &Bag, bags: &std::collections::HashMap<String, Bag>) -> bool {
    for b in &bag.bags {
        if b.name == "shiny gold" {
            return true;
        }
        if contains_gold(&bags[&b.name], &bags) {
            return true;
        }
    }
    false
}

fn contains_bags(bag: &Bag, bags: &std::collections::HashMap<String, Bag>) -> u32 {
    let mut count: u32 = 1;
    for b in &bag.bags {
        count += b.nr * contains_bags(&bags[&b.name], &bags);
    }
    count
}

/*
Day 7, part 1: 155
Day 7, part 2: 54803
*/

pub fn day7() {
    let contents = std::fs::read_to_string("input7.txt").expect("Failed to read file");
    let mut bags = std::collections::HashMap::new();
    let re_bag: regex::Regex = Regex::new(r"(.*) bags contain[s]* (.*)").unwrap();
    let re_bag_quantity: regex::Regex = Regex::new(r"[ ]*(\d+) (.*) bag.*").unwrap();
    for line in contents.lines() {
        let bag = parse_bag(line, &re_bag, &re_bag_quantity);
        bags.insert(bag.name.clone(), bag);
    }

    let mut count = 0;
    for bag in &bags {
        if contains_gold(bag.1, &bags) {
            count += 1;
        }
    }

    println!("Day 7, part 1: {:?}", count);
    println!(
        "Day 7, part 2: {}",
        contains_bags(&bags["shiny gold"], &bags) - 1
    );
}
