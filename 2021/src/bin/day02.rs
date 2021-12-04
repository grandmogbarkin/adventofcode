use std::error::Error as Error;
use std::fs;

use advent2021::exercise::{Exercise, Solution};
use advent2021::read;

struct MySolution { }

impl Solution for MySolution {
  fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let directions = read::read_lines(fs::File::open(filename)?)?;
  
    let mut horizontal: i64 = 0;
    let mut depth: i64 = 0;
  
    for direction in directions.iter() {
      let split: Vec<&str> = direction.split(' ').collect();
      match &split[..] {
        [direction, n] => {
          let distance: i64 = n.parse().unwrap();
          match direction.clone() {
            "forward" => {
              horizontal += distance;
            },
            "down" => {
              depth += distance;
            },
            "up" => {
              depth -= distance;
            },
            _ => simple_error::bail!(
                      format!("Unexpected direction: {}", direction)),
          }
        },
        _ => simple_error::bail!(format!("Unable to process line {}", direction)),
      }
    }
  
    println!("Depth: {}, horizontal: {}, product: {}",
              depth, horizontal, depth * horizontal);
    Ok(depth * horizontal)
  }


  fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let directions = read::read_lines(fs::File::open(filename)?)?;
  
    let mut horizontal: i64 = 0;
    let mut depth: i64 = 0;
    let mut aim: i64 = 0;
  
    for direction in directions.iter() {
      let split: Vec<&str> = direction.split(' ').collect();
      match &split[..] {
        [direction, n] => {
          let distance: i64 = n.parse().unwrap();
          match direction.clone() {
            "forward" => {
              horizontal += distance;
              depth += distance * aim;
            },
            "down" => {
              aim += distance;
            },
            "up" => {
              aim -= distance;
            },
            _ => simple_error::bail!(
                      format!("Unexpected direction: {}", direction)),
          }
        },
        _ => simple_error::bail!(format!("Unable to process line {}", direction)),
      }
    }
  
    println!("Depth: {}, horizontal: {}, aim: {}, product: {}",
              depth, horizontal, aim, depth * horizontal);
    Ok(depth*horizontal)
  }
}

pub fn main() {
  let d = MySolution {};

  let args1: Vec<String> = vec!["1".to_string(),
                                "inputs/input2.txt".to_string()];
  let _r1 = Exercise::run(args1, &d);

  let args2: Vec<String> = vec!["2".to_string(),
                                "inputs/input2.txt".to_string()];
  let _r2 = Exercise::run(args2, &d);
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_1() {
    let args: Vec<String> = vec!["1".to_string(),
                                 "inputs/input2.test.txt".to_string()];
    let d = super::MySolution {};
    let res = super::Exercise::run(args, &d);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 150);
  }

  #[test]
  fn test_2() {
    let args: Vec<String> = vec!["2".to_string(),
                                 "inputs/input2.test.txt".to_string()];
    let d = super::MySolution {};
    let res = super::Exercise::run(args, &d);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 900);
  }
}
