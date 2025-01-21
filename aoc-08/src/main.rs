use std::fs;

enum Instruction {
    Right,
    Left,
}

struct Path {
    value: String,
    left: String,
    right: String,
}

impl Path {
    fn new(input: &str) -> Vec<Path> {
        let lines = input.split("\n");
        let mut paths: Vec<Path> = vec![];
        for line in lines {
            let mut parts = line.split(" = ");
            let source = parts.nth(0).unwrap().trim();
            let target_parts: Vec<&str> = parts
                .nth(0)
                .unwrap()
                .trim_matches(|p| p == '(' || p == ')')
                .split(',')
                .collect();

            let [left, right] = match target_parts[..] {
                [l, r] => [l.trim(), r.trim()],
                _ => panic!("Did not have two parts"),
            };
            paths.push(Path {
                value: source.to_string(),
                left: left.to_string(),
                right: right.to_string(),
            });
        }
        return paths;
    }
    fn next<'a>(&self, instruction: &Instruction, paths: &'a [Path]) -> &'a Path {
        let next_value = match instruction {
            Instruction::Left => &self.left,
            Instruction::Right => &self.right,
        };
        paths
            .iter()
            .find(|path| path.value == *next_value)
            .expect("No next item found")
    }
}

fn get_cycle(path: &Path, paths: &[Path], instructions: &[Instruction]) -> usize {
    let mut index = 0;
    let instruction_size = instructions.len();
    let mut current_item = path;
    loop {
        let finished = current_item.value.ends_with('Z');
        if finished {
            break;
        }
        let instruction = &instructions[index % instruction_size];
        //let next_index = current_item.next(instruction, &paths);
        current_item = current_item.next(instruction, paths);
        index += 1;
    }
    return index;
}

impl Instruction {
    fn get_list(line: &str) -> Vec<Instruction> {
        line.chars()
            .map(|char| match char {
                'R' => Instruction::Right,
                'L' => Instruction::Left,
                _ => panic!("Invalid instruction"),
            })
            .collect()
    }
}
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn lcm_list(numbers: &[usize]) -> Option<usize> {
    if numbers.is_empty() {
        return None;
    }

    let mut result = numbers[0];
    for &number in numbers.iter().skip(1) {
        result = lcm(result, number);
    }

    Some(result)
}

fn parse_file(path: &str) {
    let file = fs::read_to_string(path).unwrap();
    let mut parts = file.split("\n\n");
    let instruction_string = parts.next().unwrap();
    let path_string = parts.next().unwrap();
    let instructions = Instruction::get_list(instruction_string);
    let paths = Path::new(path_string);

    let starting_points: Vec<&Path> = paths
        .iter()
        .filter_map(|path| {
            if path.value.ends_with('A') {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    let lengths: Vec<usize> = starting_points
        .iter()
        .map(|item| get_cycle(item, &paths, &instructions))
        .collect();
    let result = lcm_list(&lengths);
    println!("{result:?}");
}

fn main() {
    parse_file("test3.txt");
}
