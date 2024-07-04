use std::{fmt, fmt::Write};
use crate::ship::ShipPoint;

pub mod attacking_board;
pub mod my_board;

const BOARD_LENGTH: usize = 26;

pub trait Board {
    fn new() -> Self;

    fn check_if_point_on_board(point: &ShipPoint) -> bool {
        if point.row < 0 || point.row > 9 || point.col < 0 || point.col > 9 {
            return false;
        }

        return true;
    }

    fn get_board<T>(
        &self,
        buf: &mut String,
        name: &str,
        cells: [[T; 10]; 10],
        get_cell_display_value: fn(&T) -> &'static str,
        indent: usize
    ) -> fmt::Result {
        if indent == 0 {
            // Clears terminal screen
            write!(buf, "{}[2J", 27 as char)?;
        } else {
            // \x1B[1A moves the cursor up one line
            // \x1B[1C moves the cursor right by 1 character
            write!(buf, "\x1B[14A\x1B[{}C", indent * BOARD_LENGTH)?;
        }

        writeln!(buf, "{}\n", name)?;
        writeln!(buf, "\x1B[{}C   A B C D E F G H I J", indent * BOARD_LENGTH)?;
        for (index, row) in cells.iter().enumerate() {
            if indent != 0 {
                write!(buf, "\x1B[{}C", indent * BOARD_LENGTH)?;
            }

            let index = index + 1;
            if index < 10 {
                write!(buf, "{}  ", index)?;
            } else {
                write!(buf, "{} ", index)?;
            }

            for cell in row {
                write!(buf, "{}", get_cell_display_value(cell))?;
            }

            writeln!(buf)?;
        }

        return writeln!(buf);
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