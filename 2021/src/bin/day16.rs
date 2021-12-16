use std::error::Error;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;

struct Solution {}

impl SolutionT for Solution {
    fn day(&self) -> &str {
        "16"
    }

    fn test1_result(&self) -> i64 {
        31
    }
    fn test2_result(&self) -> i64 {
        2
    }

    fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let _test = read::read_lines(filename)?;

        println!("Hello 1!");
        Ok(1)
    }

    fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let _test = read::read_lines(filename)?;

        println!("Hello 2!");
        Ok(2)
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
    
    #[test]
    fn test1() {
        let d = Solution {};
        Exercise::custom_test(&d, ".test.1", 16, 2)
    }
    
    #[test]
    fn test2() {
        let d = Solution {};
        Exercise::custom_test(&d, ".test.2", 12, 2)
    }
    
    #[test]
    fn test3() {
        let d = Solution {};
        Exercise::custom_test(&d, ".test.3", 23, 2)
    }
}
