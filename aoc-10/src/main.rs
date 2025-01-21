use std::fs;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
    X,
    A,
}

impl Direction {
    fn reverse(direction: Direction) -> Direction {
        match direction {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
            _ => direction,
        }
    }

    fn next(position: Coordinate, direction: Direction, limits: [usize; 2]) -> Option<Coordinate> {
        match direction {
            Direction::N if position[0] > 0 => Some([position[0] - 1, position[1]]),
            Direction::S if position[0] < limits[0] - 1 => Some([position[0] + 1, position[1]]),
            Direction::E if position[1] < limits[1] - 1 => Some([position[0], position[1] + 1]),
            Direction::W if position[1] > 0 => Some([position[0], position[1] - 1]),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Pipe {
    directions: [Direction; 2],
}

impl Pipe {
    fn new(input: char) -> Self {
        let directions = match input {
            '|' => [Direction::N, Direction::S],
            'L' => [Direction::N, Direction::E],
            'J' => [Direction::N, Direction::W],
            '-' => [Direction::E, Direction::W],
            'F' => [Direction::S, Direction::E],
            '7' => [Direction::W, Direction::S],
            '.' => [Direction::X, Direction::X],
            'S' => [Direction::A, Direction::A],
            _ => panic!("Invalid character: {}", input),
        };
        Self { directions }
    }
}

#[derive(Debug)]
struct Grid(Vec<Vec<Pipe>>);

impl Grid {
    fn get(&self, position: Coordinate) -> Option<&Pipe> {
        self.0.get(position[0]).and_then(|row| row.get(position[1]))
    }

    fn new(input: &str) -> Self {
        let grid = input
            .split('\n')
            .map(|line| line.chars().map(Pipe::new).collect())
            .collect();
        Self(grid)
    }

    fn get_allowed_moves(&self, position: &Coordinate) -> Vec<Coordinate> {
        let dimensions = [self.height(), self.width()];
        self.get(*position)
            .map(|pipe| {
                pipe.directions
                    .iter()
                    .filter_map(|&d| Direction::next(*position, d, dimensions))
                    .collect()
            })
            .unwrap_or_else(|| panic!("No allowed moves from position {:?}", position))
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }
}

type Coordinate = [usize; 2];

fn parse_file(path: &str) -> Grid {
    let file_content = fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("Failed to read file {}: {}", path, err));
    Grid::new(&file_content)
}

fn transform_starting_position(position: Coordinate, grid: &mut Grid) {
    let directions = [Direction::N, Direction::E, Direction::S, Direction::W];
    let limits = [grid.height(), grid.width()];
    let result = directions
        .iter()
        .filter_map(|&d| {
            let next_pos = Direction::next(position, d, limits)?;
            let pipe = grid.get(next_pos)?;
            if pipe
                .directions
                .iter()
                .any(|&dir| Direction::reverse(dir) == d)
            {
                Some(d)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Ensuring we have exactly 2 directions
    if result.len() != 2 {
        panic!("Invalid number of directions at starting position");
    }

    grid.0[position[0]][position[1]].directions = [result[0], result[1]];
}

fn find_starting_position(grid: &Grid) -> Coordinate {
    grid.0
        .iter()
        .enumerate()
        .find_map(|(row_index, row)| {
            row.iter().enumerate().find_map(|(col_index, &pipe)| {
                if pipe.directions == [Direction::A, Direction::A] {
                    Some([row_index, col_index])
                } else {
                    None
                }
            })
        })
        .expect("No starting coordinates found")
}

fn walk(starting_position: Coordinate, grid: &Grid) -> Vec<Coordinate> {
    let mut previous_position = starting_position;
    let mut current_position = starting_position;
    let mut path: Vec<Coordinate> = vec![];
    loop {
        let allowed_moves = grid.get_allowed_moves(&current_position);
        if let Some(next_position) = allowed_moves
            .into_iter()
            .find(|&pos| pos != previous_position)
        {
            path.push(current_position);
            if next_position == starting_position {
                break;
            }
            previous_position = current_position;
            current_position = next_position;
        } else {
            panic!("No allowed moves from position {:?}", current_position);
        }
    }
    path
}

fn count_enclosing(path: &Vec<Coordinate>, grid: &Grid) {
    let width = grid.width();
    let height = grid.height();
    let mut enclosing_tiles: Vec<Coordinate> = vec![];
    for y in 0..height {
        let mut inside = false;
        for x in 0..width {
            let coordinate: Coordinate = [y, x];
            let intersects = path.iter().any(|item| *item == coordinate);
            if intersects {
                let pipe = grid.get(coordinate).unwrap();
                let is_north = pipe.directions.iter().any(|item| *item == Direction::N);
                if is_north {
                    println!("Coordinate: {coordinate:?} is north, flipping inside");
                    inside = !inside;
                }
            }
            if inside && !intersects {
                enclosing_tiles.push(coordinate);
            }
        }
    }
    println!("{enclosing_tiles:?}");
    let length = enclosing_tiles.len();
    println!("{length}");
}

fn main() {
    let mut grid = parse_file("input.txt");
    let starting_position = find_starting_position(&grid);
    transform_starting_position(starting_position, &mut grid);
    let path: Vec<[usize; 2]> = walk(starting_position, &grid);
    let path_length = path.len();
    let mid_point = (path_length as f32 / 2.0).floor() as usize;
    println!("Midpoint: {}", mid_point);
    println!("{path:?}");
    count_enclosing(&path, &grid);
}
