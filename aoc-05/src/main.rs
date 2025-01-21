use std::fs;

#[derive(Debug, Clone)]
struct Range {
    source: usize,
    target: usize,
    size: usize,
}

#[derive(Debug)]
struct Mapping {
    ranges: Vec<Range>,
}

fn get_next(value: usize, ranges: &Vec<Range>) -> usize {
    for range in ranges {
        if value >= range.source && value < range.source + range.size {
            let diff = value - range.source;
            return range.target + diff;
        }
    }
    return value;
}

fn parse_map(lines: &str) -> Mapping {
    let mut iterator = lines.split("\n").into_iter();
    iterator.next();
    let mut mapping = Mapping { ranges: vec![] };
    for line in iterator {
        let nums: Vec<usize> = line.split(" ").map(|item| item.parse().unwrap()).collect();
        let range = Range {
            source: nums[1],
            target: nums[0],
            size: nums[2],
        };
        mapping.ranges.push(range);
    }
    return mapping;
}

fn parse_seed_ranges(line: &str) -> Vec<usize> {
    let nums: Vec<usize> = line
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|item| item.parse().unwrap())
        .collect();
    println!("New loop");
    let mut total_capacity = 0;
    for chunks in nums.chunks(2) {
        if let [_start, len] = *chunks {
            total_capacity += len;
        }
    }
    let mut result = Vec::with_capacity(total_capacity);

    for chunks in nums.chunks(2) {
        if let [start, len] = *chunks {
            println!("Looping {len} elements");
            result.extend(start..start + len);
        }
    }
    return result;
}

fn parse_seeds(line: &str) -> Vec<usize> {
    line.split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|item| item.parse().unwrap())
        .collect()
}
enum Task {
    First,
    Second,
}

fn parse_lines(path: &str, task: Task) {
    let file = fs::read_to_string(path).unwrap();

    let mut sections = file.split("\n\n").into_iter();
    let mut maps: Vec<Mapping> = vec![];
    let seed_str = sections.next().unwrap();
    let mut seeds = match task {
        Task::First => parse_seeds(seed_str),
        Task::Second => parse_seed_ranges(seed_str),
    };

    for section in sections {
        let map = parse_map(section);
        maps.push(map);
    }
    for map in maps {
        seeds = seeds
            .into_iter()
            .map(|seed| get_next(seed, &map.ranges))
            .collect();
    }
    let min = seeds.iter().min().unwrap();

    println!("{min:?}");
}

fn main() {
    parse_lines("input.txt", Task::First);
    parse_lines("input.txt", Task::Second);
}
