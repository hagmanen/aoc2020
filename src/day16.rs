use regex::Regex;

type Range = (u32, u32);
type Ranges = (Range, Range);
type Fields = std::collections::HashMap<String, Ranges>;
type FieldNames = std::collections::HashSet<String>;
type Ticket = Vec<u32>;
type Tickets = Vec<Ticket>;

fn parse_fields(text: String) -> Fields {
    let re_field: regex::Regex = Regex::new(r"(.*): (\d*)-(\d*) or (\d*)-(\d*)").unwrap();
    let mut fields: Fields = std::collections::HashMap::new();
    for line in text.lines() {
        let m = re_field.captures(line).unwrap();
        let r1: Range = (m[2].parse().unwrap(), m[3].parse().unwrap());
        let r2: Range = (m[4].parse().unwrap(), m[5].parse().unwrap());
        fields.insert(m[1].to_string(), (r1, r2));
    }
    fields
}

fn parse_my_ticket(text: String) -> Ticket {
    let mut ticket: Ticket = Vec::new();
    let mut it = text.lines();
    it.next();
    for val in it.next().unwrap().split(',') {
        ticket.push(val.parse().unwrap());
    }
    ticket
}

fn parse_nearby_tickets(text: String) -> Tickets {
    let mut tickets: Tickets = Vec::new();
    let mut it = text.lines();
    it.next();
    for line in it {
        let mut ticket: Ticket = Vec::new();
        for val in line.split(',') {
            ticket.push(val.parse().unwrap());
        }
        tickets.push(ticket);
    }
    tickets
}

fn in_any_range(val: u32, fields: &Fields) -> bool {
    for ranges in fields.values() {
        if (val >= ranges.0 .0 && val <= ranges.0 .1) || (val >= ranges.1 .0 && val <= ranges.1 .1)
        {
            return true;
        }
    }
    false
}

fn part1(fields: &Fields, tickets: &[Ticket]) -> u32 {
    let mut res: u32 = 0;
    for ticket in tickets {
        for val in ticket {
            if !in_any_range(*val, fields) {
                res += val;
            }
        }
    }
    res
}

fn valid_ticket(fields: &Fields, ticket: &[u32]) -> bool {
    for val in ticket {
        if !in_any_range(*val, fields) {
            return false;
        }
    }
    true
}

fn in_range(val: u32, ranges: &Ranges) -> bool {
    if (val >= ranges.0 .0 && val <= ranges.0 .1) || (val >= ranges.1 .0 && val <= ranges.1 .1) {
        return true;
    }
    false
}

fn find_single(matching_fields: &[FieldNames]) -> (usize, String) {
    let mut i: usize = 0 as usize;
    for mf in matching_fields {
        if mf.len() == 1 {
            return (i, mf.iter().next().unwrap().to_string());
        }
        i += 1;
    }
    (i, "".to_string())
}

fn part2(fields: &Fields, my_ticket: &[u32], tickets: &[Ticket]) -> u64 {
    let mut valid_tickets: Tickets = Vec::new();
    for ticket in tickets.iter().filter(|x| valid_ticket(fields, &x)) {
        valid_tickets.push(ticket.clone());
    }
    let mut fields_of_interest: FieldNames = std::collections::HashSet::new();
    for field in fields.keys().filter(|x| x.starts_with("departure")) {
        fields_of_interest.insert(field.clone());
    }
    let mut matching_fields: Vec<FieldNames> = Vec::new();
    for _ in fields {
        matching_fields.push(fields_of_interest.clone());
    }

    for ticket in valid_tickets {
        let mut i: usize = 0 as usize;
        for value in ticket {
            matching_fields[i] = matching_fields[i]
                .clone()
                .into_iter()
                .filter(|x| in_range(value, &fields[&x.to_owned()]))
                .collect();
            i += 1;
        }
    }

    let mut indexes: std::collections::HashMap<usize, String> = std::collections::HashMap::new();
    loop {
        let t = find_single(&matching_fields);
        if t.1.is_empty() {
            break;
        }
        indexes.insert(t.0, t.1.clone());
        for m in &mut matching_fields {
            m.remove(&t.1);
        }
    }
    let mut res: u64 = 1;
    for i in indexes.keys() {
        res *= my_ticket[*i] as u64;
    }
    res
}

/*
Day 16, part 1: 24980
Day 16, part 2: 809376774329
*/

pub fn day16() {
    let contents = std::fs::read_to_string("input16.txt").expect("Failed to read file");
    let mut it = contents.split("\n\n");
    let (fields, my_ticket, nearby_tickets): (Fields, Ticket, Tickets) = (
        parse_fields(it.next().unwrap().to_string()),
        parse_my_ticket(it.next().unwrap().to_string()),
        parse_nearby_tickets(it.next().unwrap().to_string()),
    );

    println!("Day 16, part 1: {}", part1(&fields, &nearby_tickets));
    println!(
        "Day 16, part 2: {}",
        part2(&fields, &my_ticket, &nearby_tickets)
    );
}
