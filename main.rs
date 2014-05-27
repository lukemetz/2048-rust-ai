extern crate rand;

use std::iter::FromIterator;
use std::fmt;
use rand::{Rng, random};


enum Actions {
  Up,
  Down,
  Left,
  Right
}

struct Board {
  vec : Vec<int>
}

impl Board {
  pub fn empty() -> Board {
    let vec = Vec::from_elem(16, 0);
    Board { vec : vec }
  }

  pub fn new() -> Board {
    let empty = Board::empty();
    empty.add_random().add_random()
  }

  pub fn get_empty(&self) -> Vec<uint>{
    FromIterator::from_iter(
      self.vec.iter().enumerate().filter_map( |x| {
        match x {
          (index, &0) => Some(index),
          (_, _) => None
        }
      })
     )
  }

  pub fn add_random(&self) -> Board {
    let r : f32 = rand::random();
    let value = if r < 0.9f32 { 2 } else { 4 };
    let mut vec = self.vec.clone();
    let empty = self.get_empty();
    let indx = rand::task_rng().choose(empty.as_slice());
    let indx = *indx.unwrap();
    *vec.get_mut(indx) = value;
    Board { vec : vec }
  }

  pub fn get(&self, x : uint, y : uint) -> int {
    *self.vec.get(x + y*4)
  }

  pub fn count_empty(&self) -> int {
    let mut accum = 0;
    for &entry in self.vec.iter() {
      if entry == 0 {
        accum += 1;
      }
    }
    accum
  }
}

impl fmt::Show for Board {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for y in range(0u, 4u) {
      for x in range(0u, 4u) {
        let _ = write!(f, "{:5} ", self.get(x, y));
      }
      let _ = write!(f, "\n");
    }
    write!(f, "\n")
  }
}

struct GameState {
  board : Board
}

#[test]
fn test_Board_get_empty() {
  let mut board = Board::empty();
  board.vec = Vec::from_elem(16, 0);
  *board.vec.get_mut(2) = 16;
  *board.vec.get_mut(5) = 16;
  let empty = board.get_empty();
  assert_eq!(empty, vec!(0, 1, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
}

#[test]
fn test_Board_new() {
  let board = Board::new();
  assert_eq!(board.count_empty(), 14);
}

