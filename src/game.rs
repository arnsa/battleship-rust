use crate::board::{attacking_board::AttackingBoard, my_board::MyBoard, Board};

pub struct Game {
  player_a_board: MyBoard,
  player_a_attacking_board: AttackingBoard,
  player_b_board: MyBoard,
  player_b_attacking_board: AttackingBoard,
}

impl Game {
  pub fn new() -> Self {
    let mut player_a_board = MyBoard::new();
    let mut player_b_board = MyBoard::new();

    player_a_board.initiate_board_with_ships_from_input();
    player_b_board.initiate_board_with_ships_at_random();

    return Game {
      player_a_board,
      player_b_board,
      player_a_attacking_board: AttackingBoard::new(),
      player_b_attacking_board: AttackingBoard::new(),
    }
  }

  pub fn shoot(&mut self, player: char, row: char, col: i8) -> Result<(), &'static str> {
    let (board, attacking_board) = match player {
      'A' => Ok((&mut self.player_b_board, &mut self.player_a_attacking_board)),
      'B' => Ok((&mut self.player_a_board, &mut self.player_b_attacking_board)),
      _ => Err("Invalid player"),
    }?;

    match board.did_hit_ship(row, col) {
        Ok(result) => attacking_board.shoot(row, col, result).unwrap(),
        Err(err) => println!("Error: {}", err),
    }
    self.player_a_attacking_board.draw_board();

    return Ok(());
  }
}