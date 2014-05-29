extern crate rand;
extern crate time;

use std::num::Float;

use game::{Action, Board, Summary, Up, Cord};
use rand::{Rng, random, task_rng};
use std::iter::FromIterator;
use std::iter::AdditiveIterator;

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

  pub fn from_board(board : Board) -> State {
    State {action: Start, depth:0, board:board}
  }
}

#[deriving(Show)]
pub struct Score {
  empty_count : f32,
  near_game_over : f32,
  squared_log : f32,
  best_not_in_center : f32,
  smooth_rating : f32
}

impl Score {
  pub fn as_f32(&self) -> f32{
    self.empty_count +
      self.near_game_over +
      self.squared_log +
      self.best_not_in_center +
      self.smooth_rating
  }
}

#[deriving(Clone)]
pub struct ExpectiMax {
  max_depth : uint,
  num_expecti : uint
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

  pub fn new(max_depth : uint, num_expecti : uint) -> ExpectiMax {
    ExpectiMax { max_depth : max_depth , num_expecti : num_expecti}
  }

  pub fn max_layer(&self, s : &State) -> (State, f32) {
    let actions_vec = s.board.get_actions();
    if s.depth == self.max_depth || actions_vec.len() == 0 {
      let score = ExpectiMax::herustic(s);
      (s.clone(), score.as_f32())
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
      let score = ExpectiMax::herustic(s);
      score.as_f32()
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
      let empty = s.board.vec.iter().enumerate().filter_map( |x| {
        match x {
          (_, _) => None
        }
      });

      //TODO make this sampling actually correct
      let actions = if num_empty <= (self.num_expecti-1) as f32 {
        action_2.chain(empty.map(|a| a))
      } else {
        action_2.chain(a.map(|(indx, val, prob)| (indx, 4, 0.1 / num_empty)))
      };

      let num_actions = (num_empty * 2.) as uint;
      let num_samps = self.num_expecti;

      let sampled_actions = if num_actions > num_samps {
        task_rng().sample(actions, num_samps)
      } else {
        task_rng().sample(actions, num_actions)
      };

      let mut states : Vec<State> = FromIterator::from_iter(
        sampled_actions.iter().map(|&action| {
          State::new(Space(action), s.depth + 1, s.board.add_space(action))
        }));

      let mut results = states.iter().map(|next_state| {
        self.max_layer(next_state)
      });


      let mut cum_score = 0.;
      let mut cum_prob = 0.;
      for (idx, (_, score)) in results.enumerate() {
        let state = states.get(idx);
        let (_, _, prob) = state.action.space();
        cum_score += score * prob;
        cum_prob += prob;
      }
      cum_score / cum_prob
    }
  }

  pub fn herustic( s : &State) -> Score{
    //Want empty spaces
    let empty_count = s.board.count_empty() as f32;

    //near game end
    let near_game_over = if empty_count == 0. {
      -100.
    } else if empty_count < 3. {
      -7.*(3.-empty_count)
    } else {
      0.
    };

    //strive for large numbers
    let squared_sum = s.board.vec.iter().map(|&x| x*x).sum();
    let squared_log = (squared_sum as f32).log2();

    //large numbers not in the center
    let mut best_not_in_center = 0.;
    let best = s.board.get_best_tile();
    let coords = vec!(Cord(1,1), Cord(1, 2), Cord(2, 1), Cord(2, 2));
    for &cord in coords.iter() {
      if best == s.board.get(cord) {
        best_not_in_center -= 6.;
      }
      if best/2 == s.board.get(cord) {
        best_not_in_center -= 4.;
      }
    }

    //How smooth board is
    let mut smooth = 0.;
    for x in range(0, 3) {
      for y in range(0, 4) {
        let b = match s.board.get(Cord(x,y)) as f32 { 0. => 1., x => x };
        let d = match s.board.get(Cord(x+1,y)) as f32 {0. => 1., x => x };
        if b != 0. && d != 0. {
          let factor = (b.log2() - d.log2()).abs();
          smooth -= factor;
        }
      }
    }
    for y in range(0, 3) {
      for x in range(0, 4) {
        let b = match s.board.get(Cord(x,y)) as f32 { 0. => 1., x => x };
        let d = match s.board.get(Cord(x,y+1)) as f32 {0. => 1., x => x };
        if b != 0. && d != 0. {
          let factor = (b.log2() - d.log2()).abs();
          smooth -= factor;
        }
      }
    }
    let smooth_rating = -(smooth * smooth) / 200.0;

    let score = Score{
      empty_count : empty_count*2.,
      near_game_over : near_game_over,
      squared_log : squared_log,
      best_not_in_center : best_not_in_center * 3.,
      smooth_rating : smooth_rating
    };
    score
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
      let state = State::from_board(board.clone());
      println!("{}", ExpectiMax::herustic(&state));
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

  pub fn launch(&self, tx : &Sender<Report>) {
      let player = self.player.clone();
      let tx = (*tx).clone();
      spawn(proc() {
        let report = Player::play_one(player.clone());
        tx.send(report);
      });
  }

  pub fn play(&mut self, n : uint) {
    let start = time::get_time();
    let (tx, rx) : (Sender<_>, Receiver<_>) = channel();
    let mut n_left = n-8;
    for _ in range(0, 8) {
      self.launch(&tx);
    }
    for _ in range(0, n) {
      self.reports.push(rx.recv());
      if n_left != 0 {
        self.launch(&tx);
        n_left-=1;
      }
      if self.reports.len() % 5 == 0 {
        self.print_reports();
      }
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

  pub fn print_reports(&self) {
    let mut p_1024 : int = 0;
    let mut p_2048 : int = 0;
    let mut p_4096 : int = 0;
    let mut p_8192 : int = 0;

    for report in self.reports.iter() {
      if report.summary.best_tile >= 1024 {
        p_1024 += 1;
      }
      if report.summary.best_tile >= 2048 {
        p_2048 += 1;
      }
      if report.summary.best_tile >= 4096 {
        p_4096 += 1;
      }
      if report.summary.best_tile >= 8192 {
        p_8192 += 1;
      }
    }
    println!("Scores from {} Samples\n==========", self.reports.len());
    let l = self.reports.len() as f32;
    println!("1024: {}", p_1024 as f32 / l);
    println!("2048: {}", p_2048 as f32 / l);
    println!("4096: {}", p_4096 as f32 / l);
    println!("8192: {}", p_8192 as f32 / l);
  }
}

