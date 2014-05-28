extern crate rand;

use std::iter::FromIterator;
use std::fmt;
use rand::{Rng, random};


enum Action {
  Up,
  Down,
  Left,
  Right
}

impl Action {
  pub fn dir(&self) -> (int, int) {
    match *self {
      Down=> (0, 1),
      Up=> (0, -1),
      Right=> (-1, 0),
      Left=> (1, 0)
    }
  }

  pub fn neg_dir(&self) -> (int, int) {
    let (x,y) = self.dir();
    (-x, -y)
  }

  pub fn min_cord(&self, start : Cord) -> Cord {
    let Cord(x, y) = start;
    match *self {
      Up => Cord(x, 0),
      Down => Cord(x, 3),
      Left => Cord(0, y),
      Right => Cord(3, y)
    }
  }
}

#[deriving(Eq, Show)]
struct Cord(int, int);

impl Cord {
  pub fn is_valid(&self) -> bool {
    let Cord(x, y) = *self;
    if (x < 0) || (y < 0) {
      false
    } else if (x >= 4) || (y >= 4) {
      false
    } else {
      true
    }
  }

}

//TODO compute this once
fn get_traversal(action : Action) -> Vec<Cord> {
  match action {
    Up =>Vec::from_fn(16, |x| {
      Cord((x / 4) as int, (x % 4) as int)
    }),
    Down => {
      let mut ret = get_traversal(Up);
      ret.reverse();
      ret
    }
    Left => Vec::from_fn(16, |x| {
      Cord((x % 4) as int, (x / 4) as int)
    }),
    Right => {
      let mut ret = get_traversal(Left);
      ret.reverse();
      ret
    }
  }
}


fn get_first_free(start_cord: Cord, action : Action, board : &Board) -> Cord {
  let (dx, dy) = action.dir();
  let Cord(x,y) = start_cord;
  let mut on_cord = action.min_cord(start_cord);
  let mut ret = start_cord;
  while on_cord != start_cord {
    let Cord(x,y) = on_cord;
    if board.get(on_cord) == 0 {
      ret = on_cord;
      break
    }
    on_cord = Cord(x-dx, y-dy);
  }
  ret
}


#[deriving(Clone)]
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

  pub fn move(&self, action : Action) -> Board {
    let mut new = Board::empty();
    for &cord in get_traversal(action).iter() {
      match self.get(cord) {
        0 => (),
        value => {
          let best_free = get_first_free(cord, action, &new);
          *new.get_mut(best_free) = value;
        }
      }
    }
    new
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

  pub fn get(&self, c : Cord) -> int {
    let Cord(x, y) = c;
    *self.vec.get((x as uint) + (y*4) as uint)
  }

  pub fn get_mut<'a>(&'a mut self, c : Cord) -> &'a mut int {
    let Cord(x, y) = c;
    self.vec.get_mut((x as uint) + (y*4) as uint)
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
    for y in range(0, 4) {
      for x in range(0, 4) {
        let _ = write!(f, "{:5} ", self.get(Cord(x, y)));
      }
      let _ = write!(f, "\n");
    }
    write!(f, "\n")
  }
}

struct GameState {
  board : Board
}
#[cfg(test)]
mod test {
  use super::{Board, Action, Left, Right, Up, Down, Cord, get_first_free};
  #[test]
  fn test_Board_get_empty() {
    let mut board = Board::empty();
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

  #[test]
  fn test_first_free() {
    let mut board = Board::empty();
    *board.vec.get_mut(2) = 2;
    *board.vec.get_mut(5) = 2;
    assert_eq!(get_first_free(Cord(2,0), Left, &board), Cord(0,0));
    assert_eq!(get_first_free(Cord(1,1), Left, &board), Cord(0,1));
    assert_eq!(get_first_free(Cord(2,0), Up, &board), Cord(2,0));
    assert_eq!(get_first_free(Cord(1,1), Down, &board), Cord(1,3));
    *board.vec.get_mut(6) = 2;
    println!("{}", board);
    assert_eq!(get_first_free(Cord(2,1), Up, &board), Cord(2,1));
  }

  mod move {
    use super::super::{Board, Action, Left, Right, Up, Down};
    #[test]
    pub fn test_simple_move() {
      let mut board = Board::empty();
      *board.vec.get_mut(2) = 4;
      *board.vec.get_mut(5) = 2;
      println!("{}", board);
      let board_left = board.move(Left);
      println!("{}", board_left);
      let left = vec!(4, 0, 0, 0,
                      2, 0, 0, 0,
                      0, 0, 0, 0,
                      0, 0, 0, 0);
      assert_eq!(board_left.vec, left);
      let board_down = board.move(Down);
      println!("{}", board_down );

      let down = vec!(0, 0, 0, 0,
                      0, 0, 0, 0,
                      0, 0, 0, 0,
                      0, 2, 4, 0);
      assert_eq!(board_down.vec, down);
    }
  }
}

