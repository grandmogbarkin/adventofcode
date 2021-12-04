use std::error::Error as Error;
use std::fs;

use advent2021::exercise::{Exercise, Solution};
use advent2021::read;

struct Game {
  seen: u128,
}

impl Game {
  fn add_number(&mut self, n: usize) {
    self.seen |= 1 << n;
  }
}

struct Board {
  rows: [u128; 5],
  columns: [u128; 5],
}

impl Board {
  fn new(b: [[u64; 5]; 5]) -> Self {
    let mut rows: [u128; 5] = [0; 5];
    let mut cols: [u128; 5] = [0; 5];
    for r in 0..5 {
      for c in 0..5 {
        rows[r] |= 1 << b[r][c];
        cols[c] |= 1 << b[r][c];
      }
    }
    Board{rows: rows, columns: cols}
  }
  
  fn check(&self, g: &Game) -> bool {
    for r in self.rows {
      // println!("Checking row {:b}", r);
      if r & g.seen == r { return true; }
    }
    for c in self.columns {
      // println!("Checking column {:b}", c);
      if c & g.seen == c { return true; }
    }
    return false;
  }

  fn get_sum(&self, g: &Game) -> usize {
    let mut sum: usize = 0;
    for r in self.columns {
      sum += get_unmarked_numbers_sum(r, g.seen);
    }
    sum
  }
}


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

struct Day4 { }

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

impl Solution for Day4 {
  fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let input = read::read_lines(fs::File::open(filename)?)?;
  
    let (game, boards) = get_state(input);
    
    let mut live_game = Game { seen: 0 };
    
    for i in game {
      live_game.add_number(i);
      // println!("Testing {}", i);
      for b in &boards {
        // println!("Checking board");
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
    
    let mut live_game = Game { seen: 0 };
    
    for i in game {
      live_game.add_number(i);
      // println!("Testing {}", i);
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
      if boards.iter().count() == 1 && boards[0].check(&live_game) {
        println!("Found a winner! {}", i);
        return Ok((boards[0].get_sum(&live_game) * i).try_into().unwrap());
      }
    }

    Ok(0)
  }
}

pub fn main() {
  let d = Day4 {};

  let args1: Vec<String> = vec!["1".to_string(),
                                "inputs/input4.txt".to_string()];
  let _r1 = Exercise::run(args1, &d);

  let args2: Vec<String> = vec!["2".to_string(),
                                "inputs/input4.txt".to_string()];
  let _r2 = Exercise::run(args2, &d);
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_1() {
    let args: Vec<String> = vec!["1".to_string(),
                                 "inputs/input4.test.txt".to_string()];
    let d = super::Day4 {};
    let res = super::Exercise::run(args, &d);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 4512);
  }

  #[test]
  fn test_2() {
    let args: Vec<String> = vec!["2".to_string(),
                                 "inputs/input4.test.txt".to_string()];
    let d = super::Day4 {};
    let res = super::Exercise::run(args, &d);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 1924);
  }
}
