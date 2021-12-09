use std::error::Error as Error;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;

struct Solution { }

impl SolutionT for Solution {
  fn day(&self) -> &str { "03" }

  fn test1_result(&self) -> i64 { 198 }
  fn test2_result(&self) -> i64 { 230 }
  
  fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let diagnostic = read::read_binary(filename)?;
    let size = i64::BITS - diagnostic.iter().max().unwrap().leading_zeros();
    
    println!("Size: {}", size);
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in (0..size).rev() {
      let mask = 1 << i;
      let ones = diagnostic.iter().filter(|&n| *n & mask != 0).count();
      let zeroes = diagnostic.iter().count() - ones;
      
      gamma = gamma << 1;
      epsilon = epsilon << 1;
      if ones > zeroes {
        gamma += 1;
      } else {
        epsilon += 1;
      }
    }
    
    println!("Gamma: {:?}", gamma);
    println!("Epsilon: {:?}", epsilon);
    Ok(gamma * epsilon)
  }

  // Breaking case: There's more than 1 value left, and none of them have a 0 for co2 or a 1 for oxygen.
  fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let diagnostic = read::read_binary(filename)?;
    let size = i64::BITS - diagnostic.iter().max().unwrap().leading_zeros();
    
    let mut oxygen = diagnostic.to_vec(); // make a copy
    for d in (0..size).rev() {
      if oxygen.iter().count() == 1 { break; }
      let mask = 1 << d;
      let ones = oxygen.iter().filter(|&n| *n & mask != 0).count();
      let zeroes = oxygen.iter().count() - ones;
      
      if ones >= zeroes {
        oxygen.retain(|x| x & mask != 0);
      } else {
        oxygen.retain(|x| x & mask == 0);
      }
    }
    
    let mut co2 = diagnostic;
    for d in (0..size).rev() {
      if co2.iter().count() == 1 { break; }
      let mask = 1 << d;
      let ones = co2.iter().filter(|&n| *n & mask != 0).count();
      let zeroes = co2.iter().count() - ones;

      // println!("Zeroes: {}, Ones: {}\nDict: {:?}",zeroes,ones,co2);
      if zeroes <= ones {
        co2.retain(|x| x & mask == 0);
      } else {
        co2.retain(|x| x & mask != 0);
      }
    }
    
    // println!("{:?}", oxygen);
    // println!("{:?}", co2);
    
    Ok(co2[0] * oxygen[0])
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
