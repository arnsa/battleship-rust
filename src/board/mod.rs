use std::fmt;

use crate::ship::ShipPoint;

pub mod attacking_board;
pub mod my_board;

pub trait Board {
    fn new() -> Self;

    fn check_if_point_on_board(point: &ShipPoint) -> bool {
        if point.row < 0 || point.row > 9 || point.col < 0 || point.col > 9 {
            return false;
        }

        return true;
    }

    fn draw_board<T>(&self, f: &mut fmt::Formatter, cells: [[T; 10]; 10], get_cell_display_value: fn(&T) -> &'static str) -> fmt::Result {
        // Clears terminal screen
        write!(f, "{}[2J", 27 as char)?;
        write!(f, "   A B C D E F G H I J\n")?;
        for (index, row) in cells.iter().enumerate() {
            let index = index + 1;
            if index < 10 {
                write!(f, "{}  ", index)?;
            } else {
                write!(f, "{} ", index)?;
            }

            for cell in row {
                write!(f, "{}", get_cell_display_value(cell))?;
            }

            write!(f, "\n")?;
        }

        return write!(f, "\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestBoard;

    impl Board for TestBoard {
        fn new() -> Self {
            TestBoard
        }
    }

    #[test]
    fn check_if_point_on_board() {
        let point_on_board = ShipPoint { row: 5, col: 5 };
        let point_off_board = ShipPoint { row: 10, col: 1 };

        assert!(TestBoard::check_if_point_on_board(&point_on_board));
        assert!(!TestBoard::check_if_point_on_board(&point_off_board));
    }
}