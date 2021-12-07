use std::error::Error as Error;
use std::fs;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;


struct Solution { }

impl SolutionT for Solution {
  fn day(&self) -> &str { "6" }

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
    assert_eq!(res.unwrap(), 5934);
  }

  #[test]
  fn test_2() {
    let d = Solution {};
    let args: Vec<String> = vec!["2".to_string(),
                                 format!("inputs/input{}.test.txt", d.day()).to_string()];
    let res = Exercise::run(args, &d);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 26984457539);
  }
}
