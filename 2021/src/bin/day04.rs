use std::error::Error as Error;
use std::fs;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;

/// State of the game
struct Game {
  /// bitmap where the bit location of every number drawn is set
  drawn: u128,
}

impl Game {
  /// Initialize a new game given a vector of drawn numbers. Used for testing.
  ///
  /// # Arguments
  ///
  /// `g` - Drawn numbers
  #[allow(dead_code)]
  fn new(g: Vec<usize>) -> Self {
    let mut game: Game = Game{ drawn: 0 };
    for i in g {
      game.add_number(i);
    }
    game
  }
  
  /// Adds a number to the list of drawn numbers
  ///
  /// # Arguments
  ///
  /// * `n` - The number to add to the list of drawn numbers
  fn add_number(&mut self, n: usize) {
    self.drawn |= 1 << n;
  }
}

/// The state of a single board
struct Board {
  /// There is full duplication here, as each number on the board
  /// will be present in both a row and a column.
  rows: [u128; 5],
  columns: [u128; 5],
}

impl Board {
  /// Build a new board from a 2d array
  ///
  /// # Arguments
  ///
  /// * `b` - 2d array containing the raw numbers in the format of [row[column]]
  fn new(b: [[u64; 5]; 5]) -> Self {
    let mut board: Board = Board{rows: [0_u128; 5], columns: [0_u128; 5]};
    for r in 0..5 {
      for c in 0..5 {
        board.rows[r] |= 1 << b[r][c];
        board.columns[c] |= 1 << b[r][c];
      }
    }
    board
  }
  
  /// Given the current state of a game, checks if there are any winners
  ///
  /// # Arguments
  ///
  /// * `g` - Game state (drawn numbers)
  fn check(&self, g: &Game) -> bool {
    for r in self.rows {
      // println!("Checking row {:b}", r);
      if r & g.drawn == r { return true; }
    }
    for c in self.columns {
      // println!("Checking column {:b}", c);
      if c & g.drawn == c { return true; }
    }
    return false;
  }

  /// Add up all undrawn numbers from this board
  ///
  /// # Arguments
  ///
  /// * `g` - Game state (drawn numbers)
  fn get_sum(&self, g: &Game) -> usize {
    let mut sum: usize = 0;
    for r in self.rows {
      sum += get_unmarked_numbers_sum(r, g.drawn);
    }
    sum
  }
}

/// Quickly add up numbers that were not drawn
///
/// # Arguments
///
/// * `n` - Numbers on a row or column of the board
/// * `g` - Drawn numbers
fn get_unmarked_numbers_sum(n: u128, g: u128) -> usize {
  let marked = n & g;
  let mut unmarked = n ^ marked;
  // println!("{:b}", n);
  // println!("{:b}", marked);
  // println!("{:b}", unmarked);
  let mut sum = 0_usize;
  let mut i = 0;
  while unmarked != 0 {
    if unmarked & 1 == 1 {
      // println!("Found unmarked: {}", i);
      sum += i;
    }
    unmarked >>= 1;
    i += 1;
  }
  sum
}

struct Solution { }

/// Initialize all game boards to the undrawn state based on input
///
/// # Arguments
///
/// * `input` - Vector of strings where the first line is the draw order,
///             then a matrix of boards, each board separated by a new line.
///
/// # Returns
///
/// * `game` - Vector of numbers in the order drawn
/// * `boards` - Vector of game boards
fn get_state(input: Vec<String>) -> (Vec<usize>, Vec<Board>) {
  let game: Vec<usize> = input[0].split(",").map(|s| s.parse().unwrap()).collect::<Vec<usize>>();
  let mut boards: Vec<Board> = vec![];
  
  let mut cur_board: [[u64; 5]; 5] = [[0; 5]; 5];
  let mut row = 0;
  for i in 2..input.iter().count() {
    if input[i] == "" {
      boards.push(Board::new(cur_board));
      cur_board = [[0; 5]; 5];
      row = 0;
      continue;
    }
    let sp = input[i].split(" ");
    let mut col = 0;
    for s in sp {
      if s == "" { continue; }
      cur_board[row][col] = s.parse().unwrap();
      col += 1;
    }
    row += 1;
  }
  if row == 5 {
    boards.push(Board::new(cur_board));
  }
  
  (game, boards)
}

impl SolutionT for Solution {
  fn day(&self) -> &str { "4" }

  fn test1_result(&self) -> i64 { 4512 }
  fn test2_result(&self) -> i64 { 1924 }
  
  fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let input = read::read_lines(fs::File::open(filename)?)?;
  
    let (game, boards) = get_state(input);
    
    let mut live_game = Game { drawn: 0 };
    
    // Play the game
    for i in game {
      live_game.add_number(i);
      // println!("Testing {}", i);
      for b in &boards {
        // println!("Checking board");
        // If there's a win, end the game
        if b.check(&live_game) {
          // println!("Found a winner! {}", i);
          return Ok((b.get_sum(&live_game) * i).try_into().unwrap());
        }
      }
    }

    Ok(0)
  }

  fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let input = read::read_lines(fs::File::open(filename)?)?;
  
    let (game, mut boards) = get_state(input);
    
    let mut live_game = Game { drawn: 0 };
    
    // Play the game
    for i in game {
      live_game.add_number(i);
      // println!("Testing {}", i);
      // While there's more than 1 board left, remove any winning boards from the board set
      if boards.iter().count() > 1 {
          boards.retain(|board| {
          if board.check(&live_game) {
            // println!("Found a winner! {}", i);
            return false;
          }
          return true;
        });
      }
      // println!("Played: {}, boards left: {}", i, boards.iter().count());
      // Continue playing the last board until it wins
      if boards.iter().count() == 1 && boards[0].check(&live_game) {
        println!("Found a winner! {}", i);
        return Ok((boards[0].get_sum(&live_game) * i).try_into().unwrap());
      }
    }

    Ok(0)
  }
}

pub fn main() {
  let d = Solution {};
  let _r2 = Exercise::run(&d, false);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_is_working() {
    let d = Solution {};
    Exercise::run(&d, true);
  }
  
  #[test]
  fn unit_test() {
    let board = Board::new([
            [14, 21, 17, 24, 4],
            [10, 16, 15, 9, 19],
            [18, 8, 23, 26, 20],
            [22, 11, 13, 6, 5],
            [2, 0, 12, 3, 7]]);
    let g = vec![7,4,9,5,11,17,23,2,0,14,21,24];
    let game = Game::new(g);
    assert!(board.check(&game));
    assert_eq!(188, board.get_sum(&game));
  }
}
