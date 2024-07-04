use super::Board;
use crate::ship::{Ship, ShipDirection, ShipPoint};

#[derive(Debug, Copy, Clone)]
enum Cell {
    Empty,
    Ship,
}

#[derive(Debug)]
pub struct MyBoard {
    cells: [[Cell; 10]; 10],
}

impl MyBoard {
    pub fn place_ship(&mut self, row: char, col: i8, size: u8, direction: ShipDirection) -> Result<(), &str> {
        let row = MyBoard::letter_to_number(row);
        let start_point = ShipPoint { row, col };
        let end_point = Ship::ship_end_point(&start_point, size, &direction);
        let all_ship_points = Ship::all_points(&start_point, size, &direction);

        MyBoard::check_if_ship_on_board(&start_point, &end_point)?;

        let ship_collides = self.check_if_ship_collides(&all_ship_points)?;

        if ship_collides {
            return Err("There's already a ship at this position");
        }

        for point in all_ship_points.iter() {
            let row = self.cells.get_mut(point.row as usize).ok_or("Invalid row")?;
            let cell = row.get_mut(point.col as usize).ok_or("Invalid column")?;

            *cell = Cell::Ship;
        }

        return Ok(());
    }

    pub fn did_hit_ship(&mut self, row: char, col: i8) -> Result<bool, &str> {
        let row = MyBoard::letter_to_number(row);
        let is_point_on_board = MyBoard::check_if_point_on_board(&ShipPoint { row, col });

        if !is_point_on_board {
            return Err("Point not on board");
        }

        let row = self.cells.get_mut(row as usize).ok_or("Invalid row")?;
        let cell = row.get_mut(col as usize).ok_or("Invalid column")?;

        match cell {
            Cell::Empty => {
                return Ok(false)
            },
            Cell::Ship => {
                return Ok(true)
            },
        }
    }

    fn check_if_ship_on_board(start_point: &ShipPoint, end_point: &ShipPoint) -> Result<bool, &'static str> {
        let ship_is_on_board =
            MyBoard::check_if_point_on_board(start_point) &&
            MyBoard::check_if_point_on_board(end_point);

        if ship_is_on_board {
            return Ok(true);
        }

        return Err("Ship not on board");
    }

    fn check_if_ship_collides(&self, points: &Vec<ShipPoint>) -> Result<bool, &'static str> {
        for point in points {
            let row = self.cells.get(point.row as usize).ok_or("Invalid row")?;
            let cell = row.get(point.col as usize).ok_or("Invalid column")?;

            match cell {
                Cell::Ship => return Ok(true),
                _ => (),
            }
        }

        return Ok(false);
    }
}

impl Board for MyBoard {
    fn new() -> Self {
        return Self {
            cells: [[Cell::Empty; 10]; 10]
        };
    }
}