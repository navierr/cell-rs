use crate::cell::Cell;
use crate::grid::Grid;
use crate::rule::Rule;
use std::io::Write;
use termion::{color, cursor, raw};

pub struct Game {
    pub generation: u32,
    grid: Grid,
    rule: Rule,
}

impl Game {
    pub fn new(grid: Grid, rule: Rule) -> Self {
        Self { grid, rule, generation: 0 }
    }

    pub fn draw<Output: Write>(&self, output: &mut raw::RawTerminal<Output>) {
        let (width, height) = self.grid.size();

        write!(output, "{}", cursor::Goto(1, 1)).unwrap();

        for y in 0..height {
            for x in 0..width {
                if let Cell::Life = self.grid[(x, y)] {
                    write!(output, "{}", color::Bg(color::LightRed)).unwrap();
                } 
                write!(output, "  {}", color::Bg(color::Reset)).unwrap();
            }
            write!(output, "\n\r").unwrap();       
        }
    }

    pub fn print_state<Output: Write>(&mut self, output: &mut raw::RawTerminal<Output>) {
        writeln!(output, "life count : {:5}\n\rgeneration : {:5}", self.grid.count_life(), self.generation).unwrap();
    }

    pub fn next(&mut self) {
        let (width, height) = self.grid.size();
        let mut next_grid = Grid::new((width, height));

        for x in 0..width {
            for y in 0..height {
                let cell = self.grid[(x, y)];
                let neighbors = self.grid.neighbors((x, y));

                next_grid[(x, y)] = self.rule.next_state(&cell, neighbors);
            }
        }

        self.grid = next_grid;
        self.generation += 1;
    }
}