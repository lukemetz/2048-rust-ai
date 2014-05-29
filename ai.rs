extern crate rand;
extern crate time;

use std::num::Float;

use game::{Action, Board, Summary, Up};
use rand::{Rng, random};
use std::iter::FromIterator;

pub trait AIPlayer {
  fn next_action(&self, board : &Board) -> Action;
}

#[deriving(Show, Clone, Send)]
pub struct RandomPlayer;
impl AIPlayer for RandomPlayer {
  fn next_action(&self, board : &Board) -> Action {
    let actions = board.get_actions();
    let action = rand::task_rng().choose(actions.as_slice());
    *action.unwrap()
  }
}

#[deriving(Clone, Show)]
pub enum Move {
  Direction(Action),
  Space((uint, int, f32)),
  Start,
  End
}

impl Move {
  pub fn space(&self) -> (uint, int, f32) {
    match *self {
      Space(u) => u,
      _ => fail!("Not a space")
    }
  }

  pub fn dir(&self) -> Action {
    match *self {
      Direction(a) => a,
      _ => fail!("Not a direction")
    }
  }
}

#[deriving(Clone, Show)]
pub struct State {
  action : Move, //Action to get to this board
  depth : uint, //The current depth
  board : Board
}


impl State {
  pub fn new(action : Move, depth : uint, board : Board) -> State {
    State {action: action, depth:depth, board:board}
  }
}

#[deriving(Clone)]
pub struct ExpectiMax {
  max_depth : uint
}

impl AIPlayer for ExpectiMax {
  fn next_action(&self, board : &Board) -> Action {
    //println!("STARTED DOING THE TREE");
    let cur_state = State::new(Start, 0, board.clone());
    let (s, score) = self.max_layer(&cur_state);
    self.trace(&cur_state, score);
    //println!("DONE! got state {}", s);
    s.action.dir()
  }
}

impl ExpectiMax {

  fn trace(&self, state : &State, score : f32) {
    /*
    for k in range(0, state.depth) {
      print!("    ");
    }
    println!("Score {}, State {}", score, state);
    */
  }

  pub fn new(max_depth : uint) -> ExpectiMax {
    ExpectiMax { max_depth : max_depth }
  }

  pub fn max_layer(&self, s : &State) -> (State, f32) {
    let actions_vec = s.board.get_actions();
    if s.depth == self.max_depth || actions_vec.len() == 0 {
      let score = self.herustic(s);
      //print!("Leaf");
      self.trace(s, score);
      //TODO not quite correct... should be returning a Direction
      (s.clone(), score)
    } else {
      let actions = actions_vec.iter();

      let mut states : Vec<State> = FromIterator::from_iter(
        actions.map(|&action| {
          State::new(Direction(action), s.depth + 1, s.board.move(action))
        }));

      let mut results = states.iter().map(|next_state| {
        self.expecti_layer(next_state)
      });

      let inf : f32 = Float::infinity();
      let mut max_score : f32 = -inf;
      let mut max_idx : Option<uint> = None;

      for (idx, score) in results.enumerate() {
        self.trace(states.get(idx), score);
        if score > max_score {
          max_score = score;
          max_idx = Some(idx);
        }
      }

      (states.get(max_idx.unwrap()).clone(), max_score)
    }
  }

  pub fn expecti_layer(&self, s : &State) -> f32 {
    let actions_vec = s.board.get_actions();
    if s.depth == self.max_depth || actions_vec.len() == 0 {
      let score = self.herustic(s);
      //println!("<<<Leaf>>>");
      self.trace(s, score);
      score
    } else {

      let num_empty = s.board.count_empty() as f32;

      let action_2 = s.board.vec.iter().enumerate().filter_map( |x| {
        match x {
          (index, &0) => Some((index, 2, 0.9 / num_empty)),
          (_, _) => None
        }
      });
      //TODO remove code duplciation..
      let a = s.board.vec.iter().enumerate().filter_map( |x| {
        match x {
          (index, &0) => Some((index, 2, 0.9 / num_empty)),
          (_, _) => None
        }
      });
      let actions = action_2.chain(a.map(|(indx, val, prob)| (indx, 4, 0.1 / num_empty)));

      let mut states : Vec<State> = FromIterator::from_iter(
        actions.map(|action| {
          State::new(Space(action), s.depth + 1, s.board.add_space(action))
        }));

      let mut results = states.iter().map(|next_state| {
        self.max_layer(next_state)
      });


      let mut cum_score = 0.;
      for (idx, (_, score)) in results.enumerate() {
        let state = states.get(idx);
        let (_, _, prob) = state.action.space();
        cum_score += score * prob;
      }
      cum_score
    }
  }

  pub fn herustic(&self, s : &State) -> f32 {
    s.board.count_empty() as f32
  }
}

#[test]
pub fn test_ExpectiMax_simple() {
  let mut board = Board::empty();
  board.vec  = vec!(0, 2, 4, 2,
                    0, 2, 0, 2,
                    0, 4, 4, 2,
                    0, 4, 2, 2);
  let e = ExpectiMax::new(3);
  let action = e.next_action(&board);

  println!("{}", action);
  assert_eq!(0, 1);
}


#[deriving(Show)]
pub struct Report {
  pub moves : uint,
  pub summary : Summary
}

impl Report {
  pub fn new(moves : uint, summary : Summary) -> Report {
    Report {moves : moves, summary : summary}
  }
}

#[deriving(Show)]
pub struct Player<T> {
  pub player : T,
  pub reports : Vec<Report>
}

impl<T : AIPlayer + Clone + Send> Player<T> {
  pub fn new(player : T) -> Player<T> {
    Player { player : player , reports: vec!()}
  }

  pub fn play_interactive(player : T) -> Report {
    println!("starting");
    let mut board = Board::new();
    let mut moves = 0;
    while board.get_actions().len() > 0 {
      let action = player.next_action(&board);
      board = board.move(action).add_random();
      println!("{}", board);
      moves += 1;
    }
    let sum = board.summary();
    Report::new(moves, sum)
  }

  pub fn play_one(player : T) -> Report {
    println!("starting");
    let mut board = Board::new();
    let mut moves = 0;
    while board.get_actions().len() > 0 {
      let action = player.next_action(&board);
      board = board.move(action).add_random();
      moves += 1;
    }
    println!("{}", board);
    let sum = board.summary();
    Report::new(moves, sum)
  }

  pub fn play(&mut self, n : uint) {
    let start = time::get_time();
    let (tx, rx) : (Sender<_>, Receiver<_>) = channel();
    for _ in range(0, n) {
      let player = self.player.clone();
      let tx = tx.clone();
      spawn(proc() {
        let report = Player::play_one(player.clone());
        tx.send(report);
      });
    }
    for _ in range(0, n) {
      self.reports.push(rx.recv());
    }
    let end = time::get_time();
    let mut delta_s = (end.sec-start.sec) as f32;
    delta_s += (end.nsec-start.nsec) as f32 / 1e9;
    let gps = n as f32 / delta_s;

    let mut moves : i64 = 0;
    for report in self.reports.iter() {
      moves += report.moves as i64;
    }
    println!("{} Games per second", gps);
    println!("{} Moves per second", moves as f32 / delta_s );
  }
}

