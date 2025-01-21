use std::fmt;
#[derive(Clone, Copy, PartialEq, Eq)]

pub enum Item {
    Galaxy,
    Empty,
}
impl From<char> for Item {
    fn from(value: char) -> Self {
        match value {
            '#' => Item::Galaxy,
            '.' => Item::Empty,
            _ => panic!("Invalid character"),
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self {
            Item::Empty => '-',
            Item::Galaxy => '#',
        };
        write!(f, "{result}")
    }
}
type Grid = Vec<Vec<Item>>;

pub type Position = [usize; 2];

pub struct Universe {
    grid: Grid,
    pub galaxies: Vec<Position>,
    pub row_expansions: Vec<usize>,
    pub col_expansions: Vec<usize>,
}

impl Universe {
    pub fn new(file: &str) -> Self {
        let lines = file.split("\n");
        let mut galaxies = vec![];
        let grid: Grid = lines
            .map(|line| line.chars().map(Item::from).collect())
            .collect();
        for (y, row) in grid.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                if *item == Item::Galaxy {
                    galaxies.push([y, x]);
                }
            }
        }
        let (row_expansions, col_expansions) = Self::get_expansions(&grid);
        Self {
            grid,
            galaxies,
            row_expansions,
            col_expansions,
        }
    }
    pub fn get_distance(&self, first: &Position, second: &Position, multiplier: usize) -> usize {
        let multiplier = multiplier - 1;
        let x1 = first[0].min(second[0]);
        let x2 = first[0].max(second[0]);
        let y1 = first[1].min(second[1]);
        let y2 = first[1].max(second[1]);
        let row_expansion_count = self
            .row_expansions
            .iter()
            .filter(|item| **item > x1 && **item < x2)
            .count();
        let col_expansion_count = self
            .col_expansions
            .iter()
            .filter(|item| **item > y1 && **item < y2)
            .count();
        let dx = (first[1] as isize - second[1] as isize).abs() as usize
            + col_expansion_count * multiplier;
        let dy = (first[0] as isize - second[0] as isize).abs() as usize
            + row_expansion_count * multiplier;
        dx + dy
    }

    fn get_expansions(grid: &Grid) -> (Vec<usize>, Vec<usize>) {
        let width = grid[0].len();
        let mut row_expansions: Vec<usize> = vec![];
        let mut col_expansions: Vec<usize> = vec![];
        grid.iter().enumerate().for_each(|(index, row)| {
            let is_empty = row.iter().all(|item| *item == Item::Empty);
            if is_empty {
                row_expansions.push(index);
            }
        });
        for col_index in (0..width).rev() {
            let is_empty = grid.iter().all(|row| row[col_index] == Item::Empty);
            if is_empty {
                col_expansions.push(col_index);
            }
        }
        (row_expansions, col_expansions)
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.grid.iter() {
            for item in row {
                write!(f, "{item}")?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
