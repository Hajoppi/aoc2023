use std::fs;

fn string_to_number(line: &str) -> usize {
    let values = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut num_indices: Vec<_> = line
        .match_indices(char::is_numeric)
        .map(|item| (item.0, item.1.parse::<usize>().unwrap()))
        .collect();
    for (value, item) in values.iter().enumerate() {
        let mut indices: Vec<_> = line
            .match_indices(item)
            .map(|item| (item.0, value + 1))
            .collect();
        num_indices.append(&mut indices);
    }
    num_indices.sort_by(|a, b| a.0.cmp(&b.0));
    let first = num_indices.first().unwrap().1;
    let last = num_indices.last().unwrap().1;
    return first * 10 + last;
}

fn parse(path: &str) {
    let file = fs::read_to_string(path).unwrap();
    let lines = file.split("\n");
    let mut total = 0;
    for line in lines {
        total += string_to_number(line);
    }
    println!("{total}");
}
fn main() {
    parse("input.txt");
}
