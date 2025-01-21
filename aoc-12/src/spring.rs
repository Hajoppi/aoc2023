use std::fmt;

#[derive(PartialEq, Eq)]
pub enum Condition {
    Damaged,
    Unknown,
    Operational,
}

impl From<char> for Condition {
    fn from(value: char) -> Self {
        match value {
            '.' => Condition::Operational,
            '#' => Condition::Damaged,
            '?' => Condition::Unknown,
            _ => panic!("Invalid character"),
        }
    }
}
impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self {
            Condition::Damaged => '#',
            Condition::Operational => '.',
            Condition::Unknown => '?',
        };
        write!(f, "{result}")
    }
}

pub struct Spring {
    pub record: Vec<Condition>,
    pub sections: Vec<usize>,
}

impl fmt::Display for Spring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in self.record.iter() {
            write!(f, "{item}")?;
        }
        write!(f, " ")?;
        let strings: Vec<String> = self.sections.iter().map(|item| item.to_string()).collect();
        let result = strings.join(",");
        write!(f, "{result}")?;
        Ok(())
    }
}

impl Spring {
    pub fn new(line: &str) -> Self {
        let mut parts = line.split(" ");
        let record = parts.next().unwrap().chars().map(Condition::from).collect();
        let sections = parts
            .next()
            .unwrap()
            .split(",")
            .map(|item| item.parse().unwrap())
            .collect();
        Self { record, sections }
    }
}
