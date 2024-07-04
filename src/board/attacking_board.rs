use std::fmt;

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
    pub fn shoot(&mut self, point: &ShipPoint, hit_the_target: bool) -> Result<(), &str> {
        let is_point_on_board = AttackingBoard::check_if_point_on_board(point);

        if !is_point_on_board {
            return Err("Point not on board");
        }

        let row = self.cells.get_mut(point.row as usize).ok_or("Invalid row")?;
        let cell = row.get_mut(point.col as usize).ok_or("Invalid column")?;

        match cell {
            Cell::Empty => *cell = if hit_the_target { Cell::Hit } else { Cell::Miss },
            Cell::Hit | Cell::Miss => return Err("Already shot here"),
        }

        return Ok(());
    }
}

impl Board for AttackingBoard {
    fn new() -> Self {
        return Self {
            cells: [[Cell::Empty; 10]; 10]
        };
    }
}

fn get_cell_display_value(cell: &Cell) -> &'static str {
    match cell {
        Cell::Empty => return ". ",
        Cell::Miss => return "o ",
        Cell::Hit => return "x ",
    };
}

impl fmt::Display for AttackingBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return self.draw_board(f, self.cells, get_cell_display_value);
    }
}