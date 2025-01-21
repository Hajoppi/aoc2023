use std::fs;

use regex::Regex;

#[derive(Debug)]
struct Card {
    id: usize,
    count: usize,
    winning: Vec<usize>,
    current: Vec<usize>,
}

fn get_card_id(line: &str) -> usize {
    let re = Regex::new(r"(?<id>[0-9]+)").unwrap();
    let caps = re.captures(line).unwrap();
    let result: usize = caps["id"].parse().unwrap();
    return result;
}
fn get_card_numbers(line: &str, id: usize) -> Card {
    let parts: Vec<_> = line.split("|").collect();
    let winning: Vec<usize> = parts[0]
        .split(" ")
        .filter(|item| item.len() > 0)
        .map(|value| value.parse().unwrap())
        .collect();
    let current: Vec<usize> = parts[1]
        .split(" ")
        .filter(|item| item.len() > 0)
        .map(|value| value.parse().unwrap())
        .collect();
    Card {
        id,
        winning,
        count: 1,
        current,
    }
}

fn get_points(card: &Card) -> usize {
    let mut total_matches: u32 = 0;
    let winning = &card.winning;
    let current = &card.current;
    for w in winning {
        let matches = current.iter().filter(|c| w == *c).count();
        total_matches += matches as u32;
    }
    if total_matches == 0 {
        return 0;
    }
    let base: usize = 2;
    return base.pow(total_matches - 1);
}

fn get_matches(card: &Card) -> usize {
    let mut total_matches: usize = 0;
    let winning = &card.winning;
    let current = &card.current;
    for w in winning {
        let matches = current.iter().filter(|c| w == *c).count();
        total_matches += matches;
    }
    return total_matches;
}

fn parse_cards(path: &str) {
    let file = fs::read_to_string(path).unwrap();
    let lines = file.split("\n");
    let mut total = 0;
    for line in lines {
        let parts: Vec<_> = line.split(":").collect();
        let card_id = get_card_id(parts[0]);
        let number_part = parts[1];
        let card = get_card_numbers(number_part, card_id);
        let points = get_points(&card);
        total += points;
    }
    println!("{total}");
}

fn parse_copies(path: &str) {
    let file = fs::read_to_string(path).unwrap();
    let lines = file.split("\n");
    let mut cards: Vec<Card> = vec![];
    for line in lines {
        let parts: Vec<_> = line.split(":").collect();
        let card_id = get_card_id(parts[0]);
        let number_part = parts[1];
        let card = get_card_numbers(number_part, card_id);
        cards.push(card);
    }
    let mut index = 0;
    while index < cards.len() {
        let card = &cards[index];
        let count = card.count;
        let matches = get_matches(card);
        for id in card.id..card.id + matches {
            if id < cards.len() {
                cards[id].count += count
            }
        }
        index += 1;
    }
    let result = cards.iter().fold(0, |acc, card| acc + card.count);
    println!("{result}");
}

fn main() {
    parse_cards("input.txt");
    parse_copies("input.txt");
}
