extern crate rand;

use game::{Board, Left, Right, Up, Down};

pub mod game;

fn main() {
  println!("Enter wasd");
  let mut board = Board::new();
  println!("{}", board);
  for line in std::io::stdin().lines() {
    let string = match line {
      Ok(s) => s,
      _ => "nothing".to_owned()
    };
    let mut trial_board;
    if string == "a\n".to_owned() {
      trial_board = board.move(Left);
    } else if string == "d\n".to_owned() {
      trial_board = board.move(Right);
    } else if string == "w\n".to_owned() {
      trial_board = board.move(Up);
    } else if string == "s\n".to_owned() {
      trial_board = board.move(Down);
    } else {
      println!("Enter wasd");
      continue
    }
    if board == trial_board {
      println!("{}", board);
      continue
    }
    board = trial_board.add_random();
    println!("{}", board);
  }
}
