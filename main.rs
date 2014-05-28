extern crate rand;

use game::{Board, Action, Left, Right, Up, Down};
use ai::{AIPlayer, Player, RandomPlayer};

pub mod game;
pub mod ai;

pub fn repl() {
  let mut board = Board::new();
  for line in std::io::stdin().lines() {
    let string = match line {
      Ok(s) => s,
      _ => "nothing".to_owned()
    };
    if string == "a\n".to_owned() {
      board = board.move(Left);
    } else if string == "d\n".to_owned() {
      board = board.move(Right);
    } else if string == "w\n".to_owned() {
      board = board.move(Up);
    } else if string == "s\n".to_owned() {
      board = board.move(Down);
    }
    board = board.add_random();
    println!("{}", board);
  }
}

pub fn main() {
  let random = RandomPlayer;
  let mut player = Player::new(random);
  player.play(10000);
  let max = player.summaries.iter().max_by(|summary| summary.best_tile);
  println!("max: {}", max);

}
