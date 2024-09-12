
use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

use super::random::random_range;

type Position = (usize, usize);

pub enum OpenResult {
    Mine,
    NoMine(u8),
}

#[derive(Debug)]
struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>,
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let _position = (x, y);

                if !self.open_fields.contains(&_position) {
                    if self.flagged_fields.contains(&_position) {
                        f.write_str("🚩")?;
                    } else {
                        f.write_str("⬛")?;
                    }
                } else if self.mines.contains(&_position) {
                    f.write_str("💣")?;
                } else {
                    write!(f, "{} ", self.mines_nearby(_position))?;
                }
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            open_fields: HashSet::new(),
            mines: {
                let mut mines = HashSet::new();

                while mines.len() < mine_count {
                    mines.insert((random_range(0, width), random_range(0, height)));
                }

                mines
            },
            flagged_fields: HashSet::new(),
        }
    }

    pub fn neighbors(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;

        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |_position| _position != &(x, y))
    }

    pub fn mines_nearby(&self, position: Position) -> u8 {
        self.neighbors(position)
            .filter(|_position| self.mines.contains(_position))
            .count() as u8
    }

    pub fn open(&mut self, position: Position) -> OpenResult {
        if self.flagged_fields.contains(&position) {
            return OpenResult::NoMine(self.mines_nearby(position));
        }

        self.open_fields.insert(position);

        let is_mine = self.mines.contains(&position);

        if is_mine {
            OpenResult::Mine
        } else {
            OpenResult::NoMine(self.mines_nearby(position))
        }
    }

    pub fn toggle_flag(&mut self, position: Position) {
        if self.open_fields.contains(&position) {
            return;
        }

        if self.flagged_fields.contains(&position) {
            self.flagged_fields.remove(&position);
        } else {
            self.flagged_fields.insert(position);
        }
    }
}

#[cfg(test)]
mod test_game {

    use crate::minesweeper::Minesweeper;

    #[test]
    fn test() {
        let mut ms = Minesweeper::new(10, 10, 10);

        ms.open((5, 5));
        ms.toggle_flag((6, 6));
        ms.open((6, 6));

        println!("Minesweeper Board: \n{}", ms);
    }
}
