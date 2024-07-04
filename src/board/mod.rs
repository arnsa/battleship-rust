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

    fn letter_to_number(letter: char) -> i8 {
        return (letter.to_ascii_lowercase()) as i8 - 'a' as i8;
    }

    fn draw_board<T>(&self, cells: [[T; 10]; 10], draw_cell: fn(&T)) {
        // Clears terminal screen
        print!("{}[2J", 27 as char);
        println!("   A B C D E F G H I J");
        for (index, row) in cells.iter().enumerate() {
            let index = index + 1;
            if index < 10 {
                print!("{}  ", index);
            } else {
                print!("{} ", index);
            }

            for cell in row {
                draw_cell(cell);
            }

            println!();
        }
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

    #[test]
    fn letter_to_number() {
        assert_eq!(0, TestBoard::letter_to_number('a'));
        assert_eq!(2, TestBoard::letter_to_number('c'));
    }
}