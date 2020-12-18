#[derive(Debug, Clone)]
pub enum GrammarItem {
    Product,
    Sum,
    Number(u64),
    Paren,
}

#[derive(Debug, Clone)]
pub struct ParseNode {
    pub children: Vec<ParseNode>,
    pub entry: GrammarItem,
}

impl ParseNode {
    pub fn new() -> ParseNode {
        ParseNode {
            children: Vec::new(),
            entry: GrammarItem::Paren,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LexItem {
    Paren(char),
    Op(char),
    Num(u64),
}

fn get_number(c: char) -> u64 {
    c.to_string().parse().unwrap()
}

fn lex(input: &str) -> Result<Vec<LexItem>, String> {
    let mut result = Vec::new();

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                it.next();
                let n = get_number(c);
                result.push(LexItem::Num(n));
            }
            '+' | '*' => {
                result.push(LexItem::Op(c));
                it.next();
            }
            '(' | ')' => {
                result.push(LexItem::Paren(c));
                it.next();
            }
            ' ' => {
                it.next();
            }
            _ => {
                return Err(format!("unexpected character {}", c));
            }
        }
    }
    Ok(result)
}

fn matching(c: char) -> char {
    match c {
        ')' => '(',
        '(' => ')',
        _ => panic!("should have been a parenthesis!"),
    }
}

fn parse_term(
    tokens: &[LexItem],
    pos: usize,
    prio_add: bool,
) -> Result<(ParseNode, usize), String> {
    let c: &LexItem = tokens.get(pos).ok_or(String::from(
        "Unexpected end of input, expected paren or number",
    ))?;
    match c {
        LexItem::Num(n) => {
            let mut node = ParseNode::new();
            node.entry = GrammarItem::Number(*n);
            Ok((node, pos + 1))
        }
        LexItem::Paren(c) => {
            match c {
                '(' | ')' => {
                    parse_expr(tokens, pos + 1, prio_add).and_then(|(node, next_pos)| {
                        if let Some(&LexItem::Paren(c2)) = tokens.get(next_pos) {
                            if c2 == matching(*c) {
                                // okay!
                                let mut paren = ParseNode::new();
                                paren.children.push(node);
                                Ok((paren, next_pos + 1))
                            } else {
                                Err(format!(
                                    "Expected {} but found {} at {}",
                                    matching(*c),
                                    c2,
                                    next_pos
                                ))
                            }
                        } else {
                            Err(format!(
                                "Expected closing paren at {} but found {:?}",
                                next_pos,
                                tokens.get(next_pos)
                            ))
                        }
                    })
                }
                _ => Err(format!("Expected paren at {} but found {:?}", pos, c)),
            }
        }
        _ => Err(format!(
            "Unexpected token {:?}, expected paren or number",
            { c }
        )),
    }
}

fn parse_summand(tokens: &[LexItem], pos: usize) -> Result<(ParseNode, usize), String> {
    let (node_term, next_pos) = parse_term(tokens, pos, true)?;
    let c = tokens.get(next_pos);
    match c {
        Some(&LexItem::Op('+')) => {
            // recurse on the summand
            let mut sum = ParseNode::new();
            sum.entry = GrammarItem::Sum;
            sum.children.push(node_term);
            let (rhs, i) = parse_summand(tokens, next_pos + 1)?;
            sum.children.push(rhs);
            Ok((sum, i))
        }
        _ => {
            // we have just the term production, nothing more.
            Ok((node_term, next_pos))
        }
    }
}

fn parse_expr(
    tokens: &[LexItem],
    pos: usize,
    prio_add: bool,
) -> Result<(ParseNode, usize), String> {
    let (node_summand, next_pos) = match prio_add {
        true => parse_summand(tokens, pos)?,
        false => parse_term(tokens, pos, prio_add)?,
    };
    let c = tokens.get(next_pos);
    match c {
        Some(&LexItem::Op('*')) => {
            // recurse on the expr
            let mut product = ParseNode::new();
            product.entry = GrammarItem::Product;
            product.children.push(node_summand);
            let (rhs, i) = parse_expr(tokens, next_pos + 1, prio_add)?;
            product.children.push(rhs);
            Ok((product, i))
        }
        Some(&LexItem::Op('+')) => {
            // recurse on the summand
            let mut sum = ParseNode::new();
            sum.entry = GrammarItem::Sum;
            sum.children.push(node_summand);
            let (rhs, i) = parse_expr(tokens, next_pos + 1, prio_add)?;
            sum.children.push(rhs);
            Ok((sum, i))
        }
        _ => {
            // we have just the summand production, nothing more.
            Ok((node_summand, next_pos))
        }
    }
}

pub fn parse(input: &str, prio_add: bool) -> Result<ParseNode, String> {
    let tokens = lex(input)?;
    parse_expr(&tokens, 0, prio_add).and_then(|(n, i)| {
        if i == tokens.len() {
            Ok(n)
        } else {
            Err(format!(
                "Expected end of input, found {:?} at {}",
                tokens[i], i
            ))
        }
    })
}

fn eval(n: &ParseNode) -> u64 {
    match &n.entry {
        GrammarItem::Product => {
            let lhs = eval(&n.children[0]);
            let rhs = eval(&n.children[1]);
            //            println!("{} * {}", lhs, rhs);
            lhs * rhs
        }
        GrammarItem::Sum => {
            let lhs = eval(&n.children[0]);
            let rhs = eval(&n.children[1]);
            //            println!("{} + {}", lhs, rhs);
            lhs + rhs
        }
        GrammarItem::Number(nr) => {
            //            println!("{}", *nr);
            *nr
        }
        GrammarItem::Paren => eval(&n.children[0]),
    }
}

fn calc_expr(l: &str, prio_add: bool) -> u64 {
    eval(&parse(&l.chars().rev().collect::<String>(), prio_add).unwrap())
}

/*
Day 18, part 1: 5019432542701
Day 18, part 2: 70518821989947
*/

pub fn day18() {
    let contents = std::fs::read_to_string("input18.txt").expect("Failed to read file");
    println!(
        "Day 18, part 1: {}",
        contents.lines().map(|l| calc_expr(l, false)).sum::<u64>()
    );
    println!(
        "Day 18, part 2: {}",
        contents.lines().map(|l| calc_expr(l, true)).sum::<u64>()
    );
}
