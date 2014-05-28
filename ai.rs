extern crate rand;
extern crate time;

use game::{Action, Board, Summary};
use rand::{Rng, random};

pub trait AIPlayer {
  fn next_action(&self, board : &Board) -> Action;
}

#[deriving(Show)]
pub struct RandomPlayer;
impl AIPlayer for RandomPlayer {
  fn next_action(&self, board : &Board) -> Action {
    let actions = board.get_actions();
    let action = rand::task_rng().choose(actions.as_slice());
    *action.unwrap()
  }
}

#[deriving(Show)]
pub struct Player<T> {
  pub player : T,
  pub summaries : Vec<Summary>
}

impl<T : AIPlayer> Player<T> {
  pub fn new(player : T) -> Player<T> {
    Player { player : player , summaries : vec!()}
  }
  pub fn play(&mut self, n : uint) {
    let start = time::get_time();
    for _ in range(0, n) {
      let mut board = Board::new();
      while board.get_actions().len() > 0 {
        let action = self.player.next_action(&board);
        board = board.move(action).add_random();
      }
      self.summaries.push(board.summary());
    }
    let end = time::get_time();
    let mut delta_s = (end.sec-start.sec) as f32;
    delta_s += (end.nsec-start.nsec) as f32 / 1e9;
    let gps = n as f32 / delta_s;
    println!("{} Games per second", gps);
  }
}
