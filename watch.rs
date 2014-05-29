extern crate rand;

use ai::{Player, ExpectiMax};

pub mod game;
pub mod ai;

pub fn main() {
  let expecti = ExpectiMax::new(7, 16);
  Player::play_interactive(expecti);
}
