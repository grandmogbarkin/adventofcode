use std::error::Error as Error;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;

struct Solution { }

impl SolutionT for Solution {
  fn day(&self) -> &str { "02" }

  fn test1_result(&self) -> i64 { 150 }
  fn test2_result(&self) -> i64 { 900 }
  
  fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let directions = read::read_lines(filename)?;
  
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
    let directions = read::read_lines(filename)?;
  
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
  let d = Solution {};
  Exercise::run(&d, false)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_is_working() {
    let d = Solution {};
    Exercise::run(&d, true)
  }
}
