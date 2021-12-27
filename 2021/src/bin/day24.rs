use std::collections::VecDeque;
use std::error::Error;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;

struct ALU {
    reg: [i64; 4],
    input: VecDeque<i64>,
}

impl ALU {
    pub fn new() -> Self {
        ALU {
            reg: [0; 4],
            input: VecDeque::new(),
        }
    }

    pub fn reset(&mut self) {
        self.reg = [0; 4];
    }

    pub fn get(&self, var: char) -> i64 {
        match var {
            'w'..='z' => {
                let idx = (var as u8 - b'w') as usize;
                return self.reg[idx];
            }
            _ => unreachable!(),
        }
    }

    pub fn set(&mut self, var: char, val: i64) {
        self.reg[(var as u8 - b'w') as usize] = val;
    }

    pub fn run(&mut self, input: &Vec<String>, data: Option<&VecDeque<i64>>) {
        if data != None {
            self.input = data.unwrap().to_owned();
        }
        for line in input {
            self.parse(&line);
        }
    }

    fn parse(&mut self, input: &String) {
        let cmd = input.split(" ").collect::<Vec<_>>();
        let source_reg_i = (cmd[1].bytes().next().unwrap() - b'w') as usize;
        let operand = self.get_operand(if cmd.len() > 2 { cmd[2] } else { "0" });
        match cmd[0] {
            "inp" => {
                self.reg[source_reg_i] = self.input.pop_front().unwrap();
            }
            "add" => {
                self.reg[source_reg_i] += operand;
            }
            "mul" => {
                self.reg[source_reg_i] *= operand;
            }
            "div" => {
                self.reg[source_reg_i] /= operand;
            }
            "mod" => {
                self.reg[source_reg_i] %= operand;
            }
            "eql" => {
                self.reg[source_reg_i] = if self.reg[source_reg_i] == operand {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        }
    }

    fn get_operand(&self, input: &str) -> i64 {
        let mut bytes = input.bytes();
        let first = bytes.next().unwrap();
        match first {
            b'w'..=b'z' => {
                return self.reg[(first - b'w') as usize];
            }
            b'-' | b'0'..=b'9' => {
                let mut multiplier: i64 = 1;
                let mut num: i64 = first as i64 - b'0' as i64;
                if first == b'-' {
                    multiplier = -1;
                    num = (bytes.next().unwrap() - b'0') as i64;
                }
                while let Some(num_byte) = bytes.next() {
                    num *= 10;
                    num += (num_byte - b'0') as i64;
                }
                num *= multiplier;
                return num;
            }
            _ => unreachable!(),
        }
    }
}

struct Solution {}

impl SolutionT for Solution {
    fn day(&self) -> &str {
        "24"
    }

    fn test1_result(&self) -> i64 {
        1
    }
    fn test2_result(&self) -> i64 {
        2
    }

    fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let input = read::read_lines(filename)?;
        let mut alu = ALU::new();
        let data: VecDeque<i64> = VecDeque::from([9, 7, 4, 1, 9, 9, 9, 3, 2, 9, 9, 9, 9, 5]);
        alu.run(&input, Some(&data));
        println!("Input: {:?}, Result: {}", data, alu.get('z'));
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

    // #[test]
    // fn it_is_working() {
    //     let d = Solution {};
    //     Exercise::run(&d, true)
    // }

    #[test]
    fn test_mul_3() {
        let input = ["inp z", "inp x", "mul z 3", "eql z x"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let mut alu = ALU::new();

        let mut data = VecDeque::from([3, 9]);
        alu.run(&input, Some(&data));
        assert_eq!(1, alu.get('z'));
        alu.reset();

        data = VecDeque::from([3, 6]);
        alu.run(&input, Some(&data));
        assert_eq!(0, alu.get('z'));
        alu.reset();

        data = VecDeque::from([3, 8]);
        alu.run(&input, Some(&data));
        assert_eq!(0, alu.get('z'));
        alu.reset();

        data = VecDeque::from([3, 27]);
        alu.run(&input, Some(&data));
        assert_eq!(0, alu.get('z'));
        alu.reset();
    }

    #[test]
    fn test_bin() {
        let input = [
            "inp w", "add z w", "mod z 2", "div w 2", "add y w", "mod y 2", "div w 2", "add x w",
            "mod x 2", "div w 2", "mod w 2",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let mut alu = ALU::new();
        let mut data = VecDeque::from([15]);
        alu.run(&input, Some(&data));
        assert_eq!(1, alu.get('w'));
        assert_eq!(1, alu.get('x'));
        assert_eq!(1, alu.get('y'));
        assert_eq!(1, alu.get('z'));
        alu.reset();

        data = VecDeque::from([14]);
        alu.run(&input, Some(&data));
        assert_eq!(1, alu.get('w'));
        assert_eq!(1, alu.get('x'));
        assert_eq!(1, alu.get('y'));
        assert_eq!(0, alu.get('z'));
        alu.reset();

        data = VecDeque::from([9]);
        alu.run(&input, Some(&data));
        assert_eq!(1, alu.get('w'));
        assert_eq!(0, alu.get('x'));
        assert_eq!(0, alu.get('y'));
        assert_eq!(1, alu.get('z'));
        alu.reset();

        data = VecDeque::from([10]);
        alu.run(&input, Some(&data));
        assert_eq!(1, alu.get('w'));
        assert_eq!(0, alu.get('x'));
        assert_eq!(1, alu.get('y'));
        assert_eq!(0, alu.get('z'));
        alu.reset();
    }
}
