use crate::ship::ShipPoint;
use super::Board;

#[derive(Debug, Copy, Clone)]
enum Cell {
    Empty,
    Miss,
    Hit,
}

#[derive(Debug)]
pub struct AttackingBoard {
    cells: [[Cell; 10]; 10],
}

impl AttackingBoard {
    pub fn shoot(&mut self, row: char, col: i8, hit_the_target: bool) -> Result<(), &str> {
        let row = AttackingBoard::letter_to_number(row);
        let is_point_on_board = AttackingBoard::check_if_point_on_board(&ShipPoint { row, col });

        if !is_point_on_board {
            return Err("Point not on board");
        }

        let row = self.cells.get_mut(row as usize).ok_or("Invalid row")?;
        let cell = row.get_mut(col as usize).ok_or("Invalid column")?;

        match cell {
            Cell::Empty => *cell = if hit_the_target { Cell::Hit } else { Cell::Miss },
            Cell::Hit | Cell::Miss => return Err("Already shot here"),
        }

        return Ok(());
    }

    pub fn draw_board(&self) {
        println!("   A B C D E F G H I J");
        for (index, row) in self.cells.iter().enumerate() {
            let index = index + 1;
            if index < 10 {
                print!("{}  ", index);
            } else {
                print!("{} ", index);
            }

            for cell in row {
                match cell {
                    Cell::Empty => print!(". "),
                    Cell::Miss => print!("o "),
                    Cell::Hit => print!("x "),
                }
            }

            println!();
        }
    }
}

impl Board for AttackingBoard {
    fn new() -> Self {
        return Self {
            cells: [[Cell::Empty; 10]; 10]
        };
    }
}