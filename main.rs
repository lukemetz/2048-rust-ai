extern crate rand;

use game::{Board, Action, Left, Right, Up, Down};
use ai::{AIPlayer, Player, RandomPlayer, ExpectiMax};

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
  //let random = RandomPlayer;
  //let mut player = Player::new(random);
  let expecti = ExpectiMax::new(5);
  let mut player = Player::new(expecti);

  //Player::play_interactive(expecti);
  player.play(14);
  let max = player.reports.iter().max_by(|report| report.summary.best_tile);
  println!("max: {}", max);

}
