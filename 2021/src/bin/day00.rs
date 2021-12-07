use std::error::Error as Error;
use std::fs;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;

struct Solution { }

impl SolutionT for Solution {
  fn day(&self) -> &str { "1" }

  fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let _test = read::read_lines(fs::File::open(filename)?)?;

    println!("Hello 1!");
    Ok(1)
  }

  fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let _test = read::read_lines(fs::File::open(filename)?)?;

    println!("Hello 2!");
    Ok(2)
  }
}

pub fn main() {
  let d = Solution {};

  let args1: Vec<String> = vec!["1".to_string(),
                                format!("inputs/input{}.txt", d.day()).to_string()];
  let _r1 = Exercise::run(args1, &d);

  let args2: Vec<String> = vec!["2".to_string(),
                                format!("inputs/input{}.txt", d.day()).to_string()];
  let _r2 = Exercise::run(args2, &d);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let d = Solution {};
    let args: Vec<String> = vec!["1".to_string(),
                                 format!("inputs/input{}.test.txt", d.day()).to_string()];
    let res = Exercise::run(args, &d);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 1);
  }

  #[test]
  fn test_2() {
    let d = Solution {};
    let args: Vec<String> = vec!["2".to_string(),
                                 format!("inputs/input{}.test.txt", d.day()).to_string()];
    let res = Exercise::run(args, &d);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 2);
  }
}
