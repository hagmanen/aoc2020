#[derive(Debug, Clone)]
pub enum Rule {
    OrSeq(OrSeq),
    Seq(Seq),
    Char(char),
}

type OrSeq = (Seq, Seq);
type Seq = Vec<RuleId>;
type RuleId = u32;
type Rules = std::collections::HashMap<RuleId, Rule>;
type Indexes = std::collections::HashSet<usize>;
type Messages = Vec<String>;

fn parse_seq(input: &str) -> Seq {
    let mut seq = Seq::new();
    for r in input.split(' ') {
        seq.push(r.parse().unwrap());
    }
    seq
}

fn parse_rule(input: &str) -> Rule {
    let mut cit = input.chars();
    let first = cit.next().unwrap();
    if first == '"' {
        return Rule::Char(cit.next().unwrap());
    }
    let mut split_it = input.split(" | ");
    let lhs = split_it.next().unwrap();
    match split_it.next() {
        Some(rhs) => Rule::OrSeq((parse_seq(lhs), parse_seq(rhs))),
        _ => Rule::Seq(parse_seq(lhs)),
    }
}

fn parse_rules(input: std::str::Lines) -> Rules {
    let mut rules = Rules::new();
    for l in input {
        let mut p = l.split(": ");
        rules.insert(
            p.next().unwrap().parse().unwrap(),
            parse_rule(p.next().unwrap()),
        );
    }
    rules
}

fn parse_messages(input: std::str::Lines) -> Messages {
    let mut messages = Messages::new();
    for l in input {
        messages.push(l.to_string());
    }
    messages
}

fn parse(input: &mut std::str::Split<'_, &str>) -> (Rules, Messages) {
    (
        parse_rules(input.next().unwrap().lines()),
        parse_messages(input.next().unwrap().lines()),
    )
}

fn valid_or(or: &OrSeq, rules: &Rules, message: &str, ix: usize) -> Indexes {
    valid_seq(&or.0, rules, message, ix)
        .union(&valid_seq(&or.1, rules, message, ix))
        .copied()
        .collect()
}

fn valid_seq(seq: &[RuleId], rules: &Rules, message: &str, ix: usize) -> Indexes {
    let mut last_indexes: Indexes = vec![ix].into_iter().collect();
    for r in seq {
        let mut current_indexes = Indexes::new();
        for ii in last_indexes {
            current_indexes = current_indexes
                .union(&valid_message(*r, rules, message, ii))
                .copied()
                .collect();
        }
        if current_indexes.is_empty() {
            return current_indexes;
        }
        last_indexes = current_indexes;
    }
    last_indexes
}

fn valid_char(c: char, message: &str, ix: usize) -> Indexes {
    if message.len() > ix && message.get(ix..ix + 1).unwrap() == c.to_string() {
        vec![ix + 1].into_iter().collect()
    } else {
        Indexes::new()
    }
}

fn valid_message(rule: u32, rules: &Rules, message: &str, ix: usize) -> Indexes {
    match &rules[&rule] {
        Rule::OrSeq(o) => valid_or(o, rules, message, ix),
        Rule::Seq(s) => valid_seq(s, rules, message, ix),
        Rule::Char(c) => valid_char(*c, message, ix),
    }
}

fn valid_message_and_length(rules: &Rules, message: &str) -> bool {
    valid_message(0, rules, &message, 0).contains(&message.len())
}

fn valid_messages(rules: &Rules, messages: &[String]) -> usize {
    messages
        .iter()
        .filter(|m| valid_message_and_length(rules, &m))
        .count()
}

/*
Day 19, part 1: 208
Day 19, part 2: 316
*/

pub fn day19() {
    let contents = std::fs::read_to_string("input19.txt").expect("Failed to read file");
    let (mut rules, messages) = parse(&mut contents.split("\n\n"));
    println!("Day 19, part 1: {}", valid_messages(&rules, &messages));
    rules.insert(8, parse_rule("42 | 42 8"));
    rules.insert(11, parse_rule("42 31 | 42 11 31"));
    println!("Day 19, part 2: {}", valid_messages(&rules, &messages));
}
