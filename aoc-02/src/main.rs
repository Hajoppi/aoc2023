use regex::Regex;
use std::fs;

fn get_game_id(line: &str) -> u32 {
    let re = Regex::new(r"Game (?<id>[0-9]+)").unwrap();
    let caps = re.captures(line).unwrap();
    let result: u32 = caps["id"].parse().unwrap();
    return result;
}

fn parse_color_regex(line: &str, re: Regex) -> u32 {
    re.captures_iter(line)
        .map(|caps| caps["color"].parse::<u32>().unwrap())
        .max()
        .unwrap()
}

fn parse_values(line: &str) -> (u32, u32, u32) {
    let re_red = Regex::new(r"(?<color>[0-9]+) red").unwrap();
    let re_green = Regex::new(r"(?<color>[0-9]+) green").unwrap();
    let re_blue = Regex::new(r"(?<color>[0-9]+) blue").unwrap();
    let red: u32 = parse_color_regex(line, re_red);
    let green: u32 = parse_color_regex(line, re_green);
    let blue: u32 = parse_color_regex(line, re_blue);
    return (red, green, blue);
}

#[derive(Debug)]
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn parse_games(path: &str) -> Vec<Game> {
    let file = fs::read_to_string(path).unwrap();
    let lines = file.split("\n");
    let mut games: Vec<Game> = vec![];
    for line in lines {
        let id = get_game_id(line);
        let (red, green, blue) = parse_values(line);
        let game = Game {
            id,
            red,
            green,
            blue,
        };
        games.push(game);
    }
    return games;
}

fn check_games(games: &Vec<Game>) -> u32 {
    let mut total = 0;
    games.iter().for_each(|game| {
        if game.blue <= MAX_BLUE && game.red <= MAX_RED && game.green <= MAX_GREEN {
            total += game.id;
        }
    });
    total
}

fn power_games(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .fold(0, |acc, game| acc + game.red * game.green * game.blue)
}
fn main() {
    let games = parse_games("input.txt");
    let result = check_games(&games);
    let power = power_games(&games);
    println!("{result}");
    println!("{power}")
}
