use std::fs;

type List = Vec<Vec<i32>>;
fn parse_sequence(line: &str) -> Vec<i32> {
    line.split(" ").map(|item| item.parse().unwrap()).collect()
}

fn create_differences(mut acc: List) -> List {
    let latest = acc.last().unwrap();
    if latest.iter().all(|item| *item == 0) {
        return acc;
    }
    let mut iterator = latest.iter();
    let mut previous = iterator.next().unwrap().to_owned();
    let mut new: Vec<i32> = vec![];
    for current in iterator {
        new.push(current - previous);
        previous = *current;
    }
    acc.push(new);
    create_differences(acc)
}

fn extrapolate(mut acc: List) -> i32 {
    let lowest = acc.last().unwrap().clone();
    let length = acc.len();
    if length == 1 {
        return lowest.last().unwrap().to_owned();
    }
    if let Some(current) = acc.get_mut(length - 2) {
        let val = current.last().unwrap() - lowest.last().unwrap();
        current.push(val);
        acc.pop();
        return extrapolate(acc);
    } else {
        return lowest.last().unwrap().to_owned();
    }
}

fn parse_file(path: &str) {
    let file = fs::read_to_string(path).unwrap();
    let sequences: List = file.split("\n").map(|line| parse_sequence(line)).collect();
    let mut nums: Vec<i32> = vec![];
    for sequence in sequences {
        let mut list = create_differences(vec![sequence]);
        list.iter_mut().for_each(|seq| seq.reverse());
        let new_num = extrapolate(list);
        nums.push(new_num);
    }
    let result = nums.into_iter().reduce(|acc, item| acc + item).unwrap();
    println!("{result}");
}
fn main() {
    parse_file("input.txt");
}
