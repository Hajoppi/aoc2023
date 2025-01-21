use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

#[derive(Hash, PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
enum Card {
    A = 14,
    K = 13,
    Q = 12,
    J = 1,
    T = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

impl Card {
    fn new(value: char) -> Card {
        match value {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => Card::A,
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    power: u32,
}

fn card_diff(left: &[Card], right: &[Card]) -> Ordering {
    for i in 0..left.len() {
        let self_card = &left[i];
        let other_card = &right[i];
        match self_card.cmp(other_card) {
            Ordering::Equal => continue,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
        }
    }
    return Ordering::Equal;
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.power == other.power
            && match card_diff(&self.cards, &other.cards) {
                Ordering::Equal => true,
                _ => false,
            }
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Hand {
    fn gt(&self, other: &Self) -> bool {
        if self.power == other.power {
            match card_diff(&self.cards, &other.cards) {
                Ordering::Greater => return true,
                _ => return false,
            };
        }
        return self.power > other.power;
    }
    fn lt(&self, other: &Self) -> bool {
        !self.gt(other) && self.ne(other)
    }
    fn ge(&self, other: &Self) -> bool {
        self.gt(other) || self.eq(other)
    }
    fn le(&self, other: &Self) -> bool {
        self.lt(other) || self.eq(other)
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self == other {
            true => Some(Ordering::Equal),
            false => match self > other {
                true => Some(Ordering::Greater),
                false => Some(Ordering::Less),
            },
        }
    }
}
impl Eq for Hand {}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(value) => value,
            None => Ordering::Equal,
        }
    }
}

fn rank(cards: &[Card]) -> u32 {
    let mut jokers = 0;
    let mut rank_count = HashMap::new();

    for card in cards {
        if *card == Card::J {
            jokers += 1;
        } else {
            *rank_count.entry(card).or_insert(0) += 1;
        }
    }
    let mut freqs: Vec<_> = rank_count.into_values().collect();
    freqs.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order
    println!("{freqs:?}");
    if freqs.len() == 0 {
        //Happens when all are jokers
        freqs.push(0);
    }
    freqs[0] += jokers;

    match freqs.as_slice() {
        [5] | [5, ..] => 6,
        [4, ..] => 5,
        [3, 2, ..] => 4,
        [3, 1, ..] => 3,
        [2, 2, 1, ..] => 2,
        [2, 1, ..] => 1,
        _ => 0,
    }
}

impl Hand {
    fn new(cards: Vec<Card>, bid: u32) -> Self {
        let power = rank(&cards);
        Self { cards, bid, power }
    }
}

fn parse(path: &str) {
    let file = fs::read_to_string(path).unwrap();
    let lines = file.split("\n");
    let mut hands: Vec<Hand> = vec![];
    for line in lines {
        let mut parts = line.split(" ");
        let cards: Vec<Card> = parts.nth(0).unwrap().chars().map(Card::new).collect();
        let bid: u32 = parts.nth(0).unwrap().parse().unwrap();
        let hand = Hand::new(cards, bid);
        hands.push(hand);
    }
    hands.sort();
    hands.iter().for_each(|hand| println!("{hand:?}"));
    let result = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, hand)| acc + hand.bid * (rank + 1) as u32);
    println!("{result}");
}

fn main() {
    parse("input.txt");
}
