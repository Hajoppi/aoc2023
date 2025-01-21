use std::fs;
#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

fn get_distance(max_time: usize, time: usize) -> usize {
    time * (max_time - time)
}

impl Race {
    fn _from_string(lines: &str) -> Vec<Self> {
        let mut lines = lines.split("\n");
        let t_string = lines.nth(0).unwrap().split(":").nth(1).unwrap().trim();
        let times: Vec<usize> = t_string
            .split(" ")
            .filter(|item| item.len() > 0)
            .map(|item| item.parse().unwrap())
            .collect();
        let d_string = lines.nth(0).unwrap();
        let distances: Vec<usize> = d_string
            .split(":")
            .nth(1)
            .unwrap()
            .split(" ")
            .filter(|item| item.len() > 0)
            .map(|item| item.parse().unwrap())
            .collect();
        let range = 0..distances.len();
        range
            .map(|index| Self {
                time: times[index],
                distance: distances[index],
            })
            .collect()
    }
    fn from_string2(lines: &str) -> Vec<Self> {
        let mut lines = lines.split("\n");
        let time: usize = lines
            .nth(0)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .trim()
            .replace(" ", "")
            .parse()
            .unwrap();
        let distance: usize = lines
            .nth(0)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .trim()
            .replace(" ", "")
            .parse()
            .unwrap();
        return vec![Self { time, distance }];
    }
}

fn parse_file(path: &str) {
    let file = fs::read_to_string(path).unwrap();
    let races = Race::from_string2(&file);
    let mut wins: Vec<usize> = vec![0; races.len()];
    for (index, race) in races.iter().enumerate() {
        for t in 0..race.time + 1 {
            let distance = get_distance(race.time, t);
            if distance > race.distance {
                wins[index] += 1;
            }
        }
    }
    let result = wins.iter().fold(1, |acc, item| acc * item);
    println!("{result:?}");
}

fn main() {
    parse_file("input.txt");
}
