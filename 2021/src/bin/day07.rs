use std::error::Error as Error;
use std::fs;

use advent2021::exercise::{Exercise, Solution};
use advent2021::read;

struct Day7 { }

impl Solution for Day7 {
  fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let input = read::read_lines(fs::File::open(filename)?)?;
    let mut crabs: Vec<i64> = input[0].split(",").map(|s| s.parse().unwrap()).collect();

    crabs.sort();
    let median = if (crabs.len() % 2)==0 {
                    let ind_left = (crabs.len() / 2) - 1;
                    let ind_right = crabs.len() / 2;
                    (crabs[ind_left] + crabs[ind_right]) / 2
                  } else {
                    let mid = crabs.len() / 2;
                    crabs[mid]
                  };
    
    let fuel = crabs.iter().map(|c| (c - median).abs()).sum::<i64>();
    
    Ok(fuel)
  }

  fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let input = read::read_lines(fs::File::open(filename)?)?;
    let crabs: Vec<i64> = input[0].split(",").map(|s| s.parse().unwrap()).collect();

    let avg: f64 = crabs.iter().sum::<i64>() as f64 / crabs.len() as f64;
    
    let fuel = crabs.iter().map(|c| {
              let step = (c - avg.round() as i64).abs();
              step * (step + 1) / 2
            }).sum::<i64>();
    
    Ok(fuel)
  }
}

pub fn main() {
  let d = Day7 {};

  let args1: Vec<String> = vec!["1".to_string(),
                                "inputs/input7.txt".to_string()];
  let _r1 = Exercise::run(args1, &d);

  let args2: Vec<String> = vec!["2".to_string(),
                                "inputs/input7.txt".to_string()];
  let _r2 = Exercise::run(args2, &d);
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_1() {
    let args: Vec<String> = vec!["1".to_string(),
                                 "inputs/input7.test.txt".to_string()];
    let d = super::Day7 {};
    let res = super::Exercise::run(args, &d);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 37);
  }

  #[test]
  fn test_2() {
    let args: Vec<String> = vec!["2".to_string(),
                                 "inputs/input7.test.txt".to_string()];
    let d = super::Day7 {};
    let res = super::Exercise::run(args, &d);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 168);
  }
}
