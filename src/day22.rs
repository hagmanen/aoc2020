type Hand = std::collections::VecDeque<usize>;
type Hands = (Hand, Hand);
type PlayedHands = std::collections::HashSet<Hands>;

fn parse_hand(input: &str) -> Hand {
    let mut it = input.lines();
    it.next();
    let mut hand = Hand::new();
    for x in it {
        hand.push_back(x.parse().unwrap());
    }
    hand
}

fn parse_hands(input: &str) -> Hands {
    let mut it = input.split("\n\n");
    (
        parse_hand(it.next().unwrap()),
        parse_hand(it.next().unwrap()),
    )
}

fn score(hand: &Hand) -> usize {
    hand.iter().rev().enumerate().map(|x| (x.0 + 1) * x.1).sum()
}

fn part1(hands: &Hands) -> usize {
    let (mut player1, mut player2) = hands.clone();
    while !player1.is_empty() && !player2.is_empty() {
        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();
        if card1 > card2 {
            player1.push_back(card1);
            player1.push_back(card2);
        } else {
            player2.push_back(card2);
            player2.push_back(card1);
        }
    }
    if player1.is_empty() {
        return score(&player2);
    }
    score(&player1)
}

fn part2(hands: &Hands) -> usize {
    score(&play2(hands.clone()).1)
}

fn new_hand(size: &usize, hand: &Hand) -> Hand {
    let mut nh = hand.clone();
    for _ in *size..hand.len() {
        nh.pop_back();
    }
    nh
}

fn play2(mut hands: Hands) -> (bool, Hand) {
    let mut played_hands = PlayedHands::new();
    while !hands.0.is_empty() && !hands.1.is_empty() {
        if !played_hands.insert(hands.clone()) {
            return (true, hands.0);
        }
        let card1 = hands.0.pop_front().unwrap();
        let card2 = hands.1.pop_front().unwrap();
        let p1_winner: bool;
        if hands.0.len() >= card1 && hands.1.len() >= card2 {
            p1_winner = play2((new_hand(&card1, &hands.0), new_hand(&card2, &hands.1))).0;
        } else {
            p1_winner = card1 > card2;
        }
        if p1_winner {
            hands.0.push_back(card1);
            hands.0.push_back(card2);
        } else {
            hands.1.push_back(card2);
            hands.1.push_back(card1);
        }
    }
    if hands.1.is_empty() {
        return (true, hands.0);
    }
    (false, hands.1)
}

/*
Day 22, part 1: 32472
Day 22, part 2: 36463
*/

pub fn day22() {
    let contents = std::fs::read_to_string("input22.txt").expect("Failed to read file");
    let decks = parse_hands(&contents);
    println!("Day 22, part 1: {}", part1(&decks));
    println!("Day 22, part 2: {}", part2(&decks));
}
