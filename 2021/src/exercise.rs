extern crate simple_error;

use std::error::Error as Error;

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
    let test_str = if test { ".test" } else { "" };
    let filename = format!("inputs/input{}{}.txt", solution.day(), test_str).to_string();
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
