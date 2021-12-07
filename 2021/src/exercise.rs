extern crate simple_error;

use std::error::Error as Error;

pub struct Config {
    pub task: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
      if args.len() < 2 {
        return Err("not enough arguments");
      }
      let task = args[0].clone();
      let filename = args[1].clone();

      Ok(Config { task, filename })
    }
}

pub trait SolutionT {
  fn day(&self) -> &str;
  fn test1_result(&self) -> i64;
  fn test2_result(&self) -> i64;
  fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>>;
  fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>>;
}

pub struct Exercise { }

impl Exercise {
  pub fn run(solution: &dyn SolutionT, test: bool) {
    let filename = if test {
      format!("inputs/input{}.test.txt", solution.day()).to_string()
    } else {
      format!("inputs/input{}.txt", solution.day()).to_string()
    };
    let r1 = solution.task_1(filename.to_string());
    println!("Result for task 1: {:?}", r1);
    if test {
      assert!(r1.is_ok());
      assert_eq!(r1.unwrap(), solution.test1_result());
    }
    let r2 = solution.task_2(filename.to_string());
    println!("Result for task 2: {:?}", r2);
    if test {
      assert!(r2.is_ok());
      assert_eq!(r2.unwrap(), solution.test2_result());
    }
  }
}
