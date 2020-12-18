use regex::Regex;

/*
Exp : (Exp)
Exp : Exp Op Exp
Exp : [0-9]+
Op  : * | +
*/

fn calc_expr(l: &str) -> u64 {
    let ls = l.len();
    if l[ls - 1..ls].eq(")") {
        let mut bal = 1;
        let mut ix = ls - 1;
        while bal > 0 {
            ix -= 1;
            if l[ix..ix + 1].eq("(") {
                bal -= 1;
            } else if l[ix..ix + 1].eq(")") {
                bal += 1;
            }
        }
        let rhs = calc_expr(&l[ix + 1..ls - 1]);
        if ix == 0 {
            return rhs;
        }
        let lhs = calc_expr(&l[0..ix - 3]);
        if l[ix - 2..ix - 1].eq("+") {
            return lhs + rhs;
        } else {
            return lhs * rhs;
        }
    }
    let re_literal: regex::Regex = Regex::new(r"(.*)(\d+)").unwrap();
    if let Some(m) = re_literal.captures(l) {
        let rhs: u64 = m[2].parse().unwrap();
        if m[1].is_empty() {
            return rhs;
        } else {
            // The end
            let s = m[1].len();
            let lhs = calc_expr(&m[1][..s - 3]);
            if m[1][s - 2..s - 1].eq("+") {
                return lhs + rhs;
            } else {
                return lhs * rhs;
            }
        }
    }
    0
}

/*
Day 18, part 1: 5019432542701
Day 18, part 2:
*/

pub fn day18() {
    let contents = std::fs::read_to_string("input18.txt").expect("Failed to read file");
    println!(
        "Day 18, part 1: {}",
        contents.lines().map(|l| calc_expr(l)).sum::<u64>()
    );
    println!("Day 18, part 2: {}", 0);
}
