use std::fs;

use regex::Regex;

#[derive(Debug)]
struct Number {
    value: usize,
    x1: usize,
    x2: usize,
    y: usize,
}

#[derive(Debug)]
struct Symbol {
    x: usize,
    y: usize,
}

fn get_numbers(line: &str, index: usize) -> Vec<Number> {
    let re: Regex = Regex::new(r"(?<number>[0-9]+)").unwrap();
    re.find_iter(line)
        .map(|caps| Number {
            value: caps.as_str().parse().unwrap(),
            x1: caps.start() + 1,
            x2: caps.end(),
            y: index + 1,
        })
        .collect()
}

fn get_symbols(line: &str, index: usize) -> Vec<Symbol> {
    let re: Regex = Regex::new(r"(?<number>[!#/*@\=\-$%&/+]{1})").unwrap();
    re.find_iter(line)
        .map(|caps| Symbol {
            x: caps.start() + 1,
            y: index + 1,
        })
        .collect()
}

fn items_are_near(symbol: &Symbol, number: &Number) -> bool {
    let first = number.x1 - 1 <= symbol.x;
    let second = number.x2 + 1 >= symbol.x;
    let third = number.y.abs_diff(symbol.y) <= 1;
    return first && second && third;
}

fn get_gear_symbols(line: &str, index: usize) -> Vec<Symbol> {
    let re: Regex = Regex::new(r"(?<number>[*]{1})").unwrap();
    re.find_iter(line)
        .map(|caps| Symbol {
            x: caps.start() + 1,
            y: index + 1,
        })
        .collect()
}
fn get_parts(numbers: Vec<Number>, symbols: Vec<Symbol>) -> Vec<Number> {
    numbers
        .into_iter()
        .filter(|number| symbols.iter().any(|symbol| items_are_near(symbol, number)))
        .collect()
}
fn get_gears(numbers: Vec<Number>, symbols: Vec<Symbol>) -> usize {
    let mut sum = 0;
    for symbol in symbols {
        let matches: Vec<usize> = numbers
            .iter()
            .filter(|number| items_are_near(&symbol, number))
            .map(|number| number.value)
            .collect();
        if matches.len() != 2 {
            continue;
        }
        sum += matches[0] * matches[1]
    }
    return sum;
}

fn parse_values(path: &str) {
    let file = fs::read_to_string(path).unwrap();
    let lines = file.split("\n");
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];
    for (index, line) in lines.enumerate() {
        let mut n = get_numbers(line, index);
        let mut s = get_symbols(line, index);
        numbers.append(&mut n);
        symbols.append(&mut s);
    }
    let part_numbers = get_parts(numbers, symbols);
    let result = part_numbers.iter().fold(0, |acc, part| acc + part.value);
    println!("{result}");
}

fn parse_gears(path: &str) {
    let file = fs::read_to_string(path).unwrap();
    let lines = file.split("\n");
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];
    for (index, line) in lines.enumerate() {
        let mut n = get_numbers(line, index);
        let mut s = get_gear_symbols(line, index);
        numbers.append(&mut n);
        symbols.append(&mut s);
    }
    let result = get_gears(numbers, symbols);
    println!("{result}");
}

fn main() {
    parse_values("input.txt");
    parse_gears("input.txt");
}
