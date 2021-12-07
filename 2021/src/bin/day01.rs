use std::error::Error as Error;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;

struct Solution { }

impl SolutionT for Solution {
  fn day(&self) -> &str { "1" }

  fn test1_result(&self) -> i64 { 7 }
  fn test2_result(&self) -> i64 { 5 }
  
  fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let depths = read::read_ints(filename)?;
  
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
    let depths = read::read_ints(filename)?;
  
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
}
