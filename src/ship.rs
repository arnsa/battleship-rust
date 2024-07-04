#[derive(Debug)]
pub enum ShipDirection {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Debug)]
pub struct ShipPoint {
  pub row: i8,
  pub col: i8,
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
        ShipDirection::Down => ShipPoint { row: start_point.row + (size as i8), col: start_point.col },
        ShipDirection::Up => ShipPoint { row: start_point.row - (size as i8), col: start_point.col },
        ShipDirection::Left => ShipPoint { row: start_point.row, col: start_point.col - (size as i8) },
        ShipDirection::Right => ShipPoint { row: start_point.row, col: start_point.col + (size as i8) },
    };
  }

  pub fn all_points(start_point: &ShipPoint, size: u8, direction: &ShipDirection) -> Vec<ShipPoint> {
    let mut result = Vec::new();

    for i in 0..size {
      match direction {
          ShipDirection::Down => result.push(ShipPoint { row: start_point.row + (i as i8), col: start_point.col }),
          ShipDirection::Up => result.push(ShipPoint { row: start_point.row - (i as i8), col: start_point.col }),
          ShipDirection::Left => result.push(ShipPoint { row: start_point.row, col: start_point.col - (i as i8) }),
          ShipDirection::Right => result.push(ShipPoint { row: start_point.row, col: start_point.col + (i as i8) }),
      };
    }

    return result;
  }
}