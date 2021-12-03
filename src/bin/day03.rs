use std::error::Error as Error;
use std::fs;

use advent2021::exercise::{Exercise, Solution};
use advent2021::read;

struct MySolution { }

impl Solution for MySolution {
  fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let diagnostic = read::read_binary(fs::File::open(filename)?)?;
    let size = 0_i64.leading_zeros() - diagnostic.iter().max().unwrap().leading_zeros();
    
    println!("Size: {}", size);
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in (0..size).rev() {
      let mask = 1 << i;
      let mut zeroes = 0;
      let mut ones = 0;
      
      // Count ones and zeroes at index i
      for n in diagnostic.iter() {
        if n & mask != 0 {
          ones += 1;
        } else {
          zeroes += 1;
        }
      }
      
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

  fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let diagnostic = read::read_binary(fs::File::open(filename)?)?;
    let size = 0_i64.leading_zeros() - diagnostic.iter().max().unwrap().leading_zeros();
    
    let mut oxygen = diagnostic.to_vec();
    for d in (0..size).rev() {
      if oxygen.iter().count() == 1 { break; }
      let mask = 1 << d;
      let mut zeroes = 0;
      let mut ones = 0;
      for n in oxygen.iter() {
        if n & mask != 0 {
          ones += 1;
        } else {
          zeroes += 1;
        }
      }
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
      let mut zeroes = 0;
      let mut ones = 0;
      for n in co2.iter() {
        if n & mask != 0 {
          ones += 1;
        } else {
          zeroes += 1;
        }
      }
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
  let d = MySolution {};

  let args1: Vec<String> = vec!["1".to_string(),
                                "inputs/input3.final.txt".to_string()];
  let _r1 = Exercise::run(args1, &d);

  let args2: Vec<String> = vec!["2".to_string(),
                                "inputs/input3.final.txt".to_string()];
  let _r2 = Exercise::run(args2, &d);
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_1() {
    let args: Vec<String> = vec!["1".to_string(),
                                 "inputs/input3.test.txt".to_string()];
    let d = super::MySolution {};
    let res = super::Exercise::run(args, &d);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 198);
  }

  #[test]
  fn test_2() {
    let args: Vec<String> = vec!["2".to_string(),
                                 "inputs/input3.test.txt".to_string()];
    let d = super::MySolution {};
    let res = super::Exercise::run(args, &d);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 230);
  }
}
