use itertools::Itertools;
use std::fs;
mod universe;
use universe::Universe;

fn get_grid(path: &str) -> Universe {
    let file = fs::read_to_string(path).unwrap();
    Universe::new(&file)
}

fn calculate_distances(universe: &Universe) -> usize {
    let mut total_distance = 0;
    for pair in universe.galaxies.iter().combinations(2) {
        let first = pair[0];
        let second = pair[1];
        total_distance += universe.get_distance(first, second, 1_000_000);
    }
    total_distance
}
fn main() {
    let universe = get_grid("input.txt");
    let result = calculate_distances(&universe);
    println!("{result}");
}
// 82000210
// 70816186
// 9543156
// 625243292686
