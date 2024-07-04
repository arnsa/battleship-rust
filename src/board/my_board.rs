use core::fmt;
use std::{io, io::Write};
use rand::Rng;
use crate::ship::{Ship, ShipDirection, ShipPoint};
use super::Board;

#[derive(Debug, Copy, Clone)]
enum Cell {
    Empty,
    Ship,
    Hit,
}

#[derive(Debug)]
pub struct MyBoard {
    cells: [[Cell; 10]; 10],
}

const BOARD_NAME: &str = "My Board";

const SHIPS: [u8; 5] = [5, 4, 3, 3, 2];

impl MyBoard {
    pub fn did_hit_ship(&mut self, point: &ShipPoint) -> Result<bool, &str> {
        let is_point_on_board = MyBoard::check_if_point_on_board(point);

        if !is_point_on_board {
            return Err("Point not on board");
        }

        let row = self.cells.get_mut(point.row as usize).ok_or("Invalid row")?;
        let cell = row.get_mut(point.col as usize).ok_or("Invalid column")?;

        match cell {
            Cell::Empty => return Ok(false),
            Cell::Ship => {
                *cell = Cell::Hit;
                return Ok(true);
            },
            Cell::Hit => return Err("Already shot here"),
        }
    }

    pub fn initiate_board_with_ships_from_input(&mut self) {
        for ship_size in SHIPS {
            loop {
                let mut input = String::new();

                print!("Place ship (size: {}): ", ship_size);
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin().read_line(&mut input).expect("Failed to read input");

                match MyBoard::parse_user_input(&input) {
                    Ok((col, row, direction)) => {
                        match self.place_ship(&ShipPoint::new((col as u8 - 1) as char, row), ship_size, &direction) {
                            Ok(_) => break,
                            Err(err) => println!("ERR: {}", err),
                        }
                    },
                    Err(err) => println!("{}", err),
                };
            }
        }
    }

    pub fn initiate_board_with_ships_at_random(&mut self) {
        let mut rng = rand::thread_rng();

        for ship_size in SHIPS {
            loop {
                let col = rng.gen_range(b'A'..=b'J') as char;
                let row = rng.gen_range(1..=10);
                let direction_num = rng.gen_range(0..4);
                let direction = match direction_num {
                    0 => ShipDirection::Up,
                    1 => ShipDirection::Right,
                    2 => ShipDirection::Down,
                    3 => ShipDirection::Left,
                    _ => unreachable!(),
                };

                match self.place_ship(&ShipPoint::new(col, row), ship_size, &direction) {
                    Ok(_) => {
                        println!("Added ship (size: {}): {}{} {:?}", ship_size, row, col, direction);
                        break;
                    },
                    Err(_) => (),
                }
            }
        }
    }

    pub fn write_board(&self, f: &mut String, indent: usize) -> fmt::Result {
        return self.get_board(f, BOARD_NAME, self.cells, get_cell_display_value, indent);
    }

    fn parse_user_input(input: &str) -> Result<(char, i8, ShipDirection), &'static str> {
        const WRONG_FORMAT_ERROR_MESSAGE: &str = "Wrong input format. Input example: A5 D";
        let mut chars = input.chars();
        let row = chars.next().ok_or(WRONG_FORMAT_ERROR_MESSAGE)?;
        let col = chars
            .by_ref()
            .take_while(|num| num.is_digit(10))
            .collect::<String>()
            .parse::<i8>()
            .map_err(|_| WRONG_FORMAT_ERROR_MESSAGE)?;
        let direction = match chars.next().ok_or(WRONG_FORMAT_ERROR_MESSAGE) {
            Ok('U') => ShipDirection::Up,
            Ok('R') => ShipDirection::Right,
            Ok('D') => ShipDirection::Down,
            Ok('L') => ShipDirection::Left,
            Ok(_) => return Err(WRONG_FORMAT_ERROR_MESSAGE),
            Err(_) => return Err(WRONG_FORMAT_ERROR_MESSAGE),
        };

        if row < 'A' || row > 'J' || col < 1 || col > 10 {
            return Err(WRONG_FORMAT_ERROR_MESSAGE);
        };

        return Ok((row, col, direction));
    }

    fn place_ship(&mut self, start_point: &ShipPoint, size: u8, direction: &ShipDirection) -> Result<(), &str> {
        let end_point = Ship::ship_end_point(start_point, size, &direction);
        let all_ship_points = Ship::all_points(start_point, size, &direction);

        MyBoard::check_if_ship_on_board(start_point, &end_point)?;

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

fn get_cell_display_value(cell: &Cell) -> &'static str {
    match cell {
        Cell::Empty => return ". ",
        Cell::Ship => return "o ",
        Cell::Hit => return "x ",
    };
}

impl fmt::Display for MyBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();

        self.get_board(&mut buf, BOARD_NAME, self.cells, get_cell_display_value, 0)?;
        return write!(f, "{}", buf);
    }
}