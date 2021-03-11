mod cell;
mod grid;
mod rule;
mod game;

use std::io::{Write, stdin, stdout};
use std::env;
use termion::{clear, cursor, event::Key, input::TermRead, raw::IntoRawMode};
use grid::*;
use rule::*;
use game::*;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Hide).unwrap();

    let mut args = env::args().skip(1);

    let grid =
        if let Some(path) = args.next() {
            Grid::from_gridfile(&path)
        } else {
            let mut grid = Grid::new((60, 20));
            grid.randomize(0.4);
            grid
        };
    let rule =
        if let Some(rule) = args.next() { 
            Rule::from_string(rule)
        } else {
            Rule::from_type(RuleType::GoL)
        };

    let mut game = Game::new(grid, rule);

    let dur = std::time::Duration::from_secs_f32(0.1);

    for c in stdin().lock().keys() { 
        match c.unwrap() {
            Key::Char('q') => break,
            _ => {}  
        }
        game.draw(&mut stdout);
        game.print_state(&mut stdout);
        game.next();
        stdout.flush().unwrap();
        std::thread::sleep(dur);
    }

    write!(stdout, "{}", cursor::Show).unwrap();
}
