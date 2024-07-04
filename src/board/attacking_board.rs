use crate::ship::ShipPoint;
use super::{Board, Point};

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
    pub fn shoot(&mut self, point: &Point, hit_the_target: bool) -> Result<(), &str> {
        let row = AttackingBoard::letter_to_number(point.row);
        let is_point_on_board = AttackingBoard::check_if_point_on_board(&ShipPoint { row, col: point.col });

        if !is_point_on_board {
            return Err("Point not on board");
        }

        let row = self.cells.get_mut(row as usize).ok_or("Invalid row")?;
        let cell = row.get_mut(point.col as usize).ok_or("Invalid column")?;

        match cell {
            Cell::Empty => *cell = if hit_the_target { Cell::Hit } else { Cell::Miss },
            Cell::Hit | Cell::Miss => return Err("Already shot here"),
        }

        return Ok(());
    }

    fn draw_cell(cell: &Cell) {
        match cell {
            Cell::Empty => print!(". "),
            Cell::Miss => print!("o "),
            Cell::Hit => print!("x "),
        }
    }

    pub fn draw_board(&self) {
        Board::draw_board(self, self.cells, AttackingBoard::draw_cell);
    }
}

impl Board for AttackingBoard {
    fn new() -> Self {
        return Self {
            cells: [[Cell::Empty; 10]; 10]
        };
    }
}