use std::error::Error as Error;
use std::fs;

use advent2021::exercise::{Exercise, Solution};
use advent2021::read;

struct MySolution { }

impl Solution for MySolution {
  fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let depths = read::read_ints(fs::File::open(filename)?)?;
  
    println!("{} lines", depths.iter().count());
  
    let mut last: &i64 = &0;
    let mut count: i64 = -1;
  
    for i in depths.iter() {
      if i > last {
        count += 1;
      }
      last = i;
    }
  
    println!("{} times height has increased", count);
  
    Ok(count)
  }

  fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let depths = read::read_ints(fs::File::open(filename)?)?;
  
    println!("{} lines", depths.iter().count());
  
    let mut last: i64 = depths[0] + depths[1] + depths[2];
    let mut count: i64 = 0;
  
    for n in 3..depths.iter().count() {
      let cur = depths[n] + depths[n-1] + depths[n-2];
      if cur > last {
        count += 1;
      }
      last = cur;
    }
  
    println!("{} times height has increased", count);
  
    Ok(count)
  }
}

pub fn main() {
  let d = MySolution {};

  let args1: Vec<String> = vec!["1".to_string(),
                                "inputs/input1.txt".to_string()];
  let _r1 = Exercise::run(args1, &d);

  let args2: Vec<String> = vec!["2".to_string(),
                                "inputs/input1.txt".to_string()];
  let _r2 = Exercise::run(args2, &d);
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_1() {
    let args: Vec<String> = vec!["1".to_string(),
                                 "inputs/input1.test.txt".to_string()];
    let d = super::MySolution {};
    let res = super::Exercise::run(args, &d);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 7);
  }

  #[test]
  fn test_2() {
    let args: Vec<String> = vec!["2".to_string(),
                                 "inputs/input1.test.txt".to_string()];
    let d = super::MySolution {};
    let res = super::Exercise::run(args, &d);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 5);
  }
}
