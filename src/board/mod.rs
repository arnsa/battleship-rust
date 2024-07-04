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
}
