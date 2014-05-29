extern crate rand;

use ai::{Player, ExpectiMax};

pub mod game;
pub mod ai;

//Do a batch run and get statistics back to test AI
pub fn main() {
  let expecti = ExpectiMax::new(6, 6);
  let mut player = Player::new(expecti);
  player.play(100);
  player.print_reports();
}
