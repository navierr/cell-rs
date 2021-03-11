use crate::cell::Cell;

pub struct Rule {
    alive_cond: Vec<u8>,
    birth_cond: Vec<u8>,
}

pub enum RuleType {
    GoL,
}

impl Rule {
    pub fn new(alive_cond: Vec<u8>, birth_cond: Vec<u8>,) -> Self {
        Self { alive_cond, birth_cond }
    }

    pub fn from_string(text: String) -> Self {
        let v = text.split('/').map(|s| s.as_bytes()).collect::<Vec<_>>();

        Self::new(v[0].iter().map(|&b| b - b'0').collect(), v[1].iter().map(|&b| b - b'0').collect())
    }

    pub fn from_type(rule_type: RuleType) -> Self {
        match rule_type {
            RuleType::GoL => Self::new(vec![2, 3], vec![3]),
        }
    }

    pub fn next_state(&self, cell: &Cell, neighbors: u8) -> Cell {
        match *cell {
            Cell::Empty => self.birth_cond.iter().find(|&&i| neighbors == i).map_or(Cell::Empty, |_| Cell::Life),
            Cell::Life => self.alive_cond.iter().find(|&&i| neighbors == i).map_or(Cell::Empty, |_| Cell::Life)
        }
    }
}