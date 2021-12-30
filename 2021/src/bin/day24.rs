use std::collections::VecDeque;
use std::error::Error;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;

struct Alu {
    reg: [i64; 4],
    input: VecDeque<i64>,
}

impl Alu {
    pub fn new() -> Self {
        Alu {
            reg: [0; 4],
            input: VecDeque::new(),
        }
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.reg = [0; 4];
    }

    pub fn get(&self, var: char) -> i64 {
        match var {
            'w'..='z' => {
                let idx = (var as u8 - b'w') as usize;
                self.reg[idx]
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub fn set(&mut self, var: char, val: i64) {
        self.reg[(var as u8 - b'w') as usize] = val;
    }

    pub fn run(&mut self, input: &[String], data: Option<&VecDeque<i64>>) {
        if data != None {
            self.input = data.unwrap().to_owned();
        }
        for line in input {
            self.parse(line);
        }
    }

    fn parse(&mut self, input: &str) {
        let cmd = input.split(' ').collect::<Vec<_>>();
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
        let first = input.bytes().next().unwrap();
        match first {
            b'w'..=b'z' => self.reg[(first - b'w') as usize],
            _ => get_num(input),
        }
    }
}

struct Solution {}

impl SolutionT for Solution {
    fn day(&self) -> &str {
        "24"
    }

    fn test1_result(&self) -> i64 {
        0
    }
    fn test2_result(&self) -> i64 {
        0
    }

    fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let input = read::read_lines(filename)?;

        let mut alu = Alu::new();
        let data: VecDeque<i64> = VecDeque::from(run_task(&input, "max"));
        alu.run(&input, Some(&data));
        println!("Input: {:?}, Result: {}", data, alu.get('z'));
        Ok(alu.get('z'))
    }

    fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let input = read::read_lines(filename)?;

        let mut alu = Alu::new();
        let data: VecDeque<i64> = VecDeque::from(run_task(&input, "min"));
        alu.run(&input, Some(&data));
        println!("Input: {:?}, Result: {}", data, alu.get('z'));
        Ok(alu.get('z'))
    }
}

fn get_num(input: &str) -> i64 {
    let mut bytes = input.bytes();
    let first = bytes.next().unwrap();
    match first {
        b'-' | b'0'..=b'9' => {
            let mut multiplier: i64 = 1;
            let mut num: i64 = first as i64 - b'0' as i64;
            if first == b'-' {
                multiplier = -1;
                num = (bytes.next().unwrap() - b'0') as i64;
            }
            for num_byte in bytes {
                num *= 10;
                num += (num_byte - b'0') as i64;
            }
            num * multiplier
        }
        _ => unreachable!(),
    }
}

fn run_task(input: &[String], func: &str) -> [i64; 14] {
    let mut stack: Vec<(usize, i64)> = vec![];
    let mut res_max = [0; 14];
    let mut res_min = [0; 14];
    // Consider each block after an inp separately
    // Read the operand of command 4, if it's a 1, push current idx and operand of command 15
    // If it's not (it's a 26), pop, then solve min/max for:
    //   cur_idx = pop_idx + pop_v + op5
    for idx in 0..=13 {
        let op4 = input[4 + (idx * 18)].split(' ').collect::<Vec<_>>();
        if get_num(op4[2]) == 1 {
            let op15 = input[15 + (idx * 18)].split(' ').collect::<Vec<_>>();
            stack.push((idx, get_num(op15[2])))
        } else {
            let op5 = input[5 + (idx * 18)].split(' ').collect::<Vec<_>>();
            let (pop_idx, pop_v) = stack.pop().unwrap();
            let v = pop_v + get_num(op5[2]);
            // println!("w_{} = w_{} + {}", idx, pop_idx, v);
            if v > 0 {
                res_max[idx] = 9;
                res_max[pop_idx] = 9 - v;
                res_min[idx] = 1 + v;
                res_min[pop_idx] = 1;
                // println!("w_{} = {}, w_{} = {}", idx, 9, pop_idx, 9 - v);
            } else {
                res_max[idx] = 9 + v;
                res_max[pop_idx] = 9;
                res_min[idx] = 1;
                res_min[pop_idx] = 1 - v;
                // println!("w_{} = {}, w_{} = {}", idx, 9 + v, pop_idx, 9);
            }
        }
    }
    let res = if func == "max" { res_max } else { res_min };
    for v in res {
        print!("{}", v);
    }
    println!();
    res
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
        let input_str = ["inp z", "inp x", "mul z 3", "eql z x"];
        let input = input_str.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let mut alu = Alu::new();

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
        let input_str = [
            "inp w", "add z w", "mod z 2", "div w 2", "add y w", "mod y 2", "div w 2", "add x w",
            "mod x 2", "div w 2", "mod w 2",
        ];
        let input = input_str.iter().map(|s| s.to_string()).collect::<Vec<String>>();

        let mut alu = Alu::new();
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
