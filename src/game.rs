use crate::board::{attacking_board::AttackingBoard, my_board::MyBoard, Board};

pub struct Game {
  pub player_a_board: MyBoard,
  pub player_a_attacking_board: AttackingBoard,
  pub player_b_board: MyBoard,
  pub player_b_attacking_board: AttackingBoard,
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
}