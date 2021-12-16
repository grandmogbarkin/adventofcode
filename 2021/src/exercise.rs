extern crate simple_error;

use std::error::Error;

pub trait SolutionT {
    fn day(&self) -> &str;
    fn test1_result(&self) -> i64;
    fn test2_result(&self) -> i64;
    fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>>;
    fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>>;
}

pub struct Exercise {}

impl Exercise {
    pub fn run(solution: &dyn SolutionT, test: bool) {
        let test_str = if test { ".test" } else { "" };
        let filename = format!("inputs/day{}/input{}.txt", solution.day(), test_str);
        let r1 = solution.task_1(filename.to_string());
        println!("Result for task 1: {:?}\n\n", r1);
        if test {
            assert!(r1.is_ok());
            assert_eq!(r1.unwrap(), solution.test1_result());
        }
        let r2 = solution.task_2(filename);
        println!("Result for task 2: {:?}", r2);
        if test {
            assert!(r2.is_ok());
            assert_eq!(r2.unwrap(), solution.test2_result());
        }
    }

    pub fn custom_test(
        solution: &dyn SolutionT,
        test_file_suffix: &str,
        test1_result: i64,
        test2_result: i64,
    ) {
        let filename =
            format!("inputs/day{}/input{}.txt", solution.day(), test_file_suffix);
        let r1 = solution.task_1(filename.to_string());
        println!("Result for task 1: {:?}\n\n", r1);
        assert!(r1.is_ok());
        assert_eq!(r1.unwrap(), test1_result);
        let r2 = solution.task_2(filename);
        println!("Result for task 2: {:?}", r2);
        assert!(r2.is_ok());
        assert_eq!(r2.unwrap(), test2_result);
    }
}
