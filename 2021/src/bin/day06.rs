use std::error::Error as Error;
use std::fs;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;


struct Solution { }

impl SolutionT for Solution {
  fn day(&self) -> &str { "6" }

  fn test1_result(&self) -> i64 { 5934 }
  fn test2_result(&self) -> i64 { 26984457539 }
  
  fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let input = read::read_lines(fs::File::open(filename)?)?;
    let mut ages: Vec<usize> = input[0].split(",").map(|s| s.parse().unwrap()).collect();

    for _i in 0..80 {
      let mut new_fish = 0;
      for fish in &mut ages {
        if *fish == 0 {
          new_fish += 1;
          *fish = 7;
        }
        *fish -= 1;
      }
      ages.resize(ages.len()+new_fish, 8);
    }

    println!("{:?}", ages.len());
    Ok(ages.len().try_into().unwrap())
  }

  fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let input = read::read_lines(fs::File::open(filename)?)?;
    let ages: Vec<usize> = input[0].split(",").map(|s| s.parse().unwrap()).collect();

    let mut fish_by_day: [usize; 9] = [0; 9];
    for age in ages {
      fish_by_day[age] += 1;
    }
    for _day in 0..256 {
      let new_births = fish_by_day[0];
      for i in 0..8 {
        fish_by_day[i] = fish_by_day[i+1];
      }
      fish_by_day[6] += new_births;
      fish_by_day[8] = new_births;
    }
    println!("{:?}",fish_by_day);
    Ok(fish_by_day.iter().sum::<usize>().try_into().unwrap())
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
