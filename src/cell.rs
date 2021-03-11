use rand;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Life,
}

impl Cell {
    pub fn random(p: f64) -> Self {
        if rand::random::<f64>() < p { Self::Life } else { Self::Empty }
    }

    pub fn is_life(&self) -> bool {
        *self == Self::Life
    }
}