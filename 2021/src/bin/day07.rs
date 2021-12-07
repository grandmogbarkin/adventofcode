use std::error::Error as Error;
use std::fs;

use advent2021::exercise::{Exercise, Solution};
use advent2021::read;

fn mean(numbers: &Vec<i64>) -> i64 {
    let sum = numbers.iter().sum::<i64>();
    (sum as f64 / numbers.len() as f64).round() as i64
  }

struct Day7 { }

impl Solution for Day7 {
  fn day(&self) -> &str { "7" }

  fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let input = read::read_lines(fs::File::open(filename)?)?;
    let mut crabs: Vec<i64> = input[0].split(",").map(|s| s.parse().unwrap()).collect();

    crabs.sort();
    let mid = crabs.len() / 2;
    let median = if (crabs.len() % 2) == 0 {
                     mean(&vec![crabs[mid - 1], crabs[mid]])
                  } else {
                    crabs[mid]
                  };
    
    let fuel = crabs.iter().map(|c| (c - median).abs()).sum::<i64>();
    
    Ok(fuel)
  }

  fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let input = read::read_lines(fs::File::open(filename)?)?;
    let crabs: Vec<i64> = input[0].split(",").map(|s| s.parse().unwrap()).collect();

    let avg: i64 = mean(&crabs);
    
    let fuel = crabs.iter().map(|c| {
              let n = (c - avg).abs();
              n * (n + 1) / 2
            }).sum::<i64>();
    
    Ok(fuel)
  }
}

pub fn main() {
  let d = Day7 {};

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
    let d = Day7 {};
    let args: Vec<String> = vec!["1".to_string(),
                                 format!("inputs/input{}.test.txt", d.day()).to_string()];
    let res = Exercise::run(args, &d);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 37);
  }

  #[test]
  fn test_2() {
    let d = Day7 {};
    let args: Vec<String> = vec!["2".to_string(),
                                 format!("inputs/input{}.test.txt", d.day()).to_string()];
    let res = Exercise::run(args, &d);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 168);
  }
}
