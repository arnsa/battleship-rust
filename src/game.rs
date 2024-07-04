use rand::Rng;
use std::{env, io, io::Write};
use crate::{board::{attacking_board::AttackingBoard, my_board::MyBoard, Board}, ship::ShipPoint};

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

    match env::var("AUTO_INITIATE") {
      Ok(_) => player_a_board.initiate_board_with_ships_at_random(),
      Err(_) => player_a_board.initiate_board_with_ships_from_input(),
      };
    player_b_board.initiate_board_with_ships_at_random();

    return Game {
      player_a_board,
      player_b_board,
      player_a_attacking_board: AttackingBoard::new(),
      player_b_attacking_board: AttackingBoard::new(),
    }
  }

  pub fn start(&mut self) {
    const WRONG_FORMAT_ERROR_MESSAGE: &str = "Wrong input format. Input example: A5";

    self.player_a_board.draw_board();
    self.player_a_attacking_board.draw_board();

    loop {
      let mut input = String::new();

      print!("Shoot: ");
      io::stdout().flush().expect("Failed to flush stdout");
      io::stdin().read_line(&mut input).expect("Failed to read input");

      let (col, row) = input.split_at(1);
      let col = match col.trim().parse::<char>() {
        Ok(r) => r,
        Err(_) => {
          println!("{}", WRONG_FORMAT_ERROR_MESSAGE);
          continue;
        },
      };
      let row = match row.trim().parse::<i8>() {
        Ok(c) => c - 1,
        Err(_) => {
          println!("{}", WRONG_FORMAT_ERROR_MESSAGE);
          continue;
        },
      };

      match self.shoot('A', &ShipPoint::new(col, row)) {
        Ok(_) => self.player_a_attacking_board.draw_board(),
        Err(err) => {
          println!("Error: {}", err);
          continue;
        }
      }
      self.computer_shoot();
    }
  }

  fn shoot<'a>(&'a mut self, player: char, point: &ShipPoint) -> Result<(), &'a str> {
    let (board, attacking_board) = match player {
        'A' => Ok((&mut self.player_b_board, &mut self.player_a_attacking_board)),
        'B' => Ok((&mut self.player_a_board, &mut self.player_b_attacking_board)),
        _ => Err("Invalid player"),
    }?;

    let result = board.did_hit_ship(&point)?;
    attacking_board.shoot(&point, result)?;

    return Ok(());
  }

  fn computer_shoot(&mut self) {
    let mut rng = rand::thread_rng();
    let col = rng.gen_range(b'A'..=b'J') as char;
    let row = rng.gen_range(1..=10);

    match self.shoot('B', &ShipPoint::new(col, row)) {
      Ok(_) => println!("Computer shot at: {}{}", row, col),
      Err(_) => self.computer_shoot(),
    };
  }
}