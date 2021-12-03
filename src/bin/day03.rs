use std::error::Error as Error;
use std::fs;

use advent2021::exercise::{Exercise, Solution};
use advent2021::read;

struct MySolution { }

impl Solution for MySolution {
  fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
    let diagnostic = read::read_lines(fs::File::open(filename)?)?;
    
    let mut zeroes: Vec<isize> = vec![];
    let mut ones: Vec<isize> = vec![];
    for d in 0..diagnostic[0].chars().count() {
      zeroes.push(0);
      ones.push(0);
      for n in 0..diagnostic.iter().count() {
        if diagnostic[n].as_bytes()[d] as char == '1' {
          ones[d] += 1;
        } else {
          zeroes[d] += 1;
        }
      }
    }
    
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..zeroes.iter().count() {
      gamma = gamma << 1;
      epsilon = epsilon << 1;
      if ones[i] > zeroes[i] {
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
    let diagnostic = read::read_lines(fs::File::open(filename)?)?;
    
    let mut oxygen: Vec<String> = diagnostic.to_vec();
    for d in 0..oxygen[0].chars().count() {
      let size = oxygen.iter().count();
      if size == 1 { break };
      let mut zeroes = 0;
      let mut ones = 0;
      for n in 0..size {
        if oxygen[n].as_bytes()[d] as char == '1' {
          ones += 1;
        } else {
          zeroes += 1;
        }
      }
      if ones >= zeroes {
        oxygen.retain(|x| x.as_bytes()[d] as char == '1');
      } else {
        oxygen.retain(|x| x.as_bytes()[d] as char == '0');
      }
    }
    
    let mut co2: Vec<String> = diagnostic;
    for d in 0..co2[0].chars().count() {
      let size = co2.iter().count();
      if size == 1 { break };
      let mut zeroes = 0;
      let mut ones = 0;
      for n in 0..size {
        if co2[n].as_bytes()[d] as char == '1' {
          ones += 1;
        } else {
          zeroes += 1;
        }
      }
      if zeroes <= ones {
        co2.retain(|x| x.as_bytes()[d] as char == '0');
      } else {
        co2.retain(|x| x.as_bytes()[d] as char == '1');
      }
    }
    
    // println!("{:?}", oxygen);
    // println!("{:?}", co2);
    
    let mut co2_num = 0;
    let mut oxygen_num = 0;
    for d in 0..co2[0].chars().count() {
      co2_num = co2_num << 1;
      oxygen_num = oxygen_num << 1;
      if co2[0].as_bytes()[d] as char == '1' { co2_num += 1; }
      if oxygen[0].as_bytes()[d] as char == '1' { oxygen_num += 1; }
    }
    Ok(co2_num * oxygen_num)
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
