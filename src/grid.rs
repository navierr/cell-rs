use std::fs;
use std::ops::{ Index, IndexMut };
use crate::cell::Cell;

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<Cell>>,
    width: usize, 
    height: usize,
}

impl Grid {
    pub fn new((width, height): (usize, usize)) -> Self {
        Self { grid: vec![vec![Cell::Empty; width]; height], width, height }
    }

    pub fn from_gridfile(path: &str) -> Self {
        let text = fs::read_to_string(path).unwrap().trim().to_string();
        let lines = text.split('\n').map(|s| s.to_string()).collect::<Vec<_>>();

        let size = lines[0].split_ascii_whitespace().into_iter().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let (width, height) = (size[0], size[1]);

        let grid = lines.iter()
            .skip(1)
            .take(height)
            .map(|line| line.chars().take(width).map(|c| match c {
                '0' => Cell::Empty,
                '1' => Cell::Life,
                _ => Cell::Empty,
            }).collect::<Vec<_>>()).collect::<Vec<_>>();

        Self { grid, width, height }
    }

    pub fn randomize(&mut self, p: f64) {
        self.grid
            .iter_mut()
            .flatten()
            .for_each(|c| *c = Cell::random(p));
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn count_life(&self) -> usize {
        self.grid
            .iter()
            .flatten()
            .fold(0, |count, c| count + c.is_life() as usize)
    }

    pub fn neighbors(&self, (x, y): (usize, usize)) -> u8 {
        let (width, height) = (self.width as isize, self.height as isize);
        let (x, y) = (x as isize, y as isize);

        [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1), 
         (x - 1, y    ),             (x + 1, y    ), 
         (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)]
         .iter()
         .filter(|(nx, ny)| (0..width).contains(&nx) && (0..height).contains(&ny))
         .fold(0, |count, (nx, ny)| {
            count + self[(*nx as usize, *ny as usize)].is_life() as u8
         })
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Cell;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.grid[y][x]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.grid[y][x]
    }
}