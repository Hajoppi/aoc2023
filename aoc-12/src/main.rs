mod spring;
use std::fs;

use spring::Spring;

fn parse_file(path: &str) -> Vec<Spring> {
    let file = fs::read_to_string(path).unwrap();
    file.split("\n").map(Spring::new).collect()
}

fn main() {
    let list = vec![1, 2, 3, 4, 5, 6];
    let slice = &list[0..2];
    println!("{slice:?}");
    let springs = parse_file("test.txt");
    springs.iter().for_each(|item| println!("{item}"));
}
