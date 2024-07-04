#[derive(Debug)]
pub enum ShipDirection {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Debug, PartialEq)]
pub struct ShipPoint {
  pub row: i8,
  pub col: i8,
}

impl ShipPoint {
    pub fn new(row: char, col: i8) -> Self {
        let row = ShipPoint::letter_to_number(row);

        return ShipPoint { row, col };
    }

    fn letter_to_number(letter: char) -> i8 {
        return (letter.to_ascii_lowercase()) as i8 - 'a' as i8;
    }
}

pub struct Ship {
  pub start_point: ShipPoint,
  pub end_point: ShipPoint,
}

impl Ship {
  pub fn new(start_point: ShipPoint, size: u8, direction: &ShipDirection) -> Self {
    let end_point = Ship::ship_end_point(&start_point, size, direction);

    return Ship {
      start_point,
      end_point,
    };
  }

  pub fn ship_end_point(start_point: &ShipPoint, size: u8, direction: &ShipDirection) -> ShipPoint {
    return match direction {
        ShipDirection::Down => ShipPoint { row: start_point.row, col: start_point.col + (size as i8) },
        ShipDirection::Up => ShipPoint { row: start_point.row, col: start_point.col - (size as i8) },
        ShipDirection::Left => ShipPoint { row: start_point.row - (size as i8), col: start_point.col },
        ShipDirection::Right => ShipPoint { row: start_point.row + (size as i8), col: start_point.col },
    };
  }

  pub fn all_points(start_point: &ShipPoint, size: u8, direction: &ShipDirection) -> Vec<ShipPoint> {
    let mut result = Vec::new();

    for i in 0..size {
      match direction {
          ShipDirection::Down => result.push(ShipPoint { row: start_point.row, col: start_point.col + (i as i8) }),
          ShipDirection::Up => result.push(ShipPoint { row: start_point.row, col: start_point.col - (i as i8) }),
          ShipDirection::Left => result.push(ShipPoint { row: start_point.row - (i as i8), col: start_point.col }),
          ShipDirection::Right => result.push(ShipPoint { row: start_point.row + (i as i8), col: start_point.col }),
      };
    }

    return result;
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ship_end_point_down() {
        assert_eq!(
            Ship::ship_end_point(&ShipPoint { row: 0, col: 0 }, 2, &ShipDirection::Down),
            ShipPoint { row: 0, col: 2 }
        );
    }

    #[test]
    fn ship_end_point_up() {
        assert_eq!(
            Ship::ship_end_point(&ShipPoint { row: 10, col: 10 }, 3, &ShipDirection::Up),
            ShipPoint { row: 10, col: 7 }
        );
    }

    #[test]
    fn ship_end_point_left() {
        assert_eq!(
            Ship::ship_end_point(&ShipPoint { row: 10, col: 10 }, 4, &ShipDirection::Left),
            ShipPoint { row: 6, col: 10 }
        );
    }

    #[test]
    fn ship_end_point_right() {
        assert_eq!(
            Ship::ship_end_point(&ShipPoint { row: 10, col: 10 }, 5, &ShipDirection::Right),
            ShipPoint { row: 15, col: 10 }
        );
    }

    #[test]
    fn all_points_down() {
        assert_eq!(
            Ship::all_points(&ShipPoint { row: 10, col: 10 }, 2, &ShipDirection::Down),
            vec![
                ShipPoint { row: 10, col: 10 },
                ShipPoint { row: 10, col: 11 }
            ]
        );
    }

    #[test]
    fn all_points_up() {
        assert_eq!(
            Ship::all_points(&ShipPoint { row: 10, col: 10 }, 3, &ShipDirection::Up),
            vec![
                ShipPoint { row: 10, col: 10 },
                ShipPoint { row: 10, col: 9 },
                ShipPoint { row: 10, col: 8 },
            ]
        );
    }

    #[test]
    fn all_points_left() {
        assert_eq!(
            Ship::all_points(&ShipPoint { row: 10, col: 10 }, 4, &ShipDirection::Left),
            vec![
                ShipPoint { row: 10, col: 10 },
                ShipPoint { row: 9, col: 10 },
                ShipPoint { row: 8, col: 10 },
                ShipPoint { row: 7, col: 10 },
            ]
        );
    }

    #[test]
    fn all_points_right() {
        assert_eq!(
            Ship::all_points(&ShipPoint { row: 10, col: 10 }, 5, &ShipDirection::Right),
            vec![
                ShipPoint { row: 10, col: 10 },
                ShipPoint { row: 11, col: 10 },
                ShipPoint { row: 12, col: 10 },
                ShipPoint { row: 13, col: 10 },
                ShipPoint { row: 14, col: 10 },
            ]
        );
    }

    #[test]
    fn letter_to_number() {
        assert_eq!(0, ShipPoint::letter_to_number('a'));
        assert_eq!(2, ShipPoint::letter_to_number('c'));
    }
}