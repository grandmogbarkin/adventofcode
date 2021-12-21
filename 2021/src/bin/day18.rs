use std::error::Error;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;

struct Solution {}

impl SolutionT for Solution {
    fn day(&self) -> &str {
        "18"
    }

    fn test1_result(&self) -> i64 {
        4140
    }
    fn test2_result(&self) -> i64 {
        3993
    }

    fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let input = read::read_lines(filename)?;

        let mut curr_num = parse_snails(&input[0]);

        for line in input.iter().skip(1) {
            let next_num = reduce_full(&add_snails(&curr_num, &parse_snails(line)));
            curr_num = next_num;
        }
        println!("{:?}", curr_num);
        Ok(magnitude(&curr_num) as i64)
    }

    fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let input = read::read_lines(filename)?;
        let mut max = 0;
        for (i, line1) in input.iter().enumerate() {
            for (j, line2) in input.iter().enumerate() {
                if i == j {
                    continue;
                }
                // println!("Adding {} with {}", line1, line2);
                let s1 = parse_snails(line1);
                let s2 = parse_snails(line2);
                let mag1 = magnitude(&reduce_full(&add_snails(&s1, &s2)));
                let mag2 = magnitude(&reduce_full(&add_snails(&s2, &s1)));
                max = std::cmp::max(max, std::cmp::max(mag1, mag2));
            }
        }

        Ok(max as i64)
    }
}

fn reduce_full(input: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut sum = input.to_owned();
    let mut next_sum = vec![];
    while sum != next_sum {
        next_sum = sum.clone();
        sum = reduce(&next_sum);
    }
    sum
}

fn parse_snails(input: &str) -> Vec<(usize, usize)> {
    let chars = input.chars().collect::<Vec<char>>();
    let mut out: Vec<(usize, usize)> = vec![];
    let mut depth: usize = 0;
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        i += 1;
        match c {
            '[' => {
                depth += 1;
                continue;
            }
            ']' => {
                depth -= 1;
                continue;
            }
            ',' => {
                continue;
            }
            _ => {
                // Number. 1-2 digits
                let mut v: usize = c.to_digit(10).unwrap() as usize;
                let v_2 = chars[i].to_digit(10);
                if v_2 != None {
                    v = (v * 10) + v_2.unwrap() as usize;
                    i += 1;
                }
                out.push((v, depth));
            }
        }
    }
    out
}

fn add_snails(left: &[(usize, usize)], right: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut out: Vec<(usize, usize)> = vec![];
    for &(v, depth) in left {
        out.push((v, depth + 1));
    }
    for &(v, depth) in right {
        out.push((v, depth + 1));
    }
    out
}

fn reduce(input: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut out = input.to_owned();
    // Explode
    for i in 0..input.len() {
        let (v, depth) = input[i];

        if depth > 4 {
            if i > 0 {
                out[i - 1].0 += v;
            }
            // by definition, there will never be depth > 5, so the next element is the second pair
            if i + 2 < input.len() {
                out[i + 2].0 += out[i + 1].0;
            }
            out[i + 1].0 = 0;
            out[i + 1].1 = depth - 1;
            out.remove(i);
            // break early, need to be done with explosions before moving on to splits
            return out;
        }
    }

    // Split
    for i in 0..input.len() {
        let (v, depth) = input[i];
        if v >= 10 {
            let v_avg = v as f32 / 2.0;
            let v0 = v_avg.floor() as usize;
            let v1 = v_avg.ceil() as usize;
            out[i].0 = v0;
            out[i].1 = depth + 1;
            out.insert(i + 1, (v1, depth + 1));
            // break early, may need to explode again.
            return out;
        }
    }

    out
}

fn magnitude(input: &[(usize, usize)]) -> usize {
    let mut num = input.to_owned();
    let mut depth = 4;

    while num.len() > 1 && depth > 0 {
        let mut found = false;
        for i in 0..num.len() {
            if num[i].1 == depth {
                let left = num[i].0 * 3;
                let right = num[i + 1].0 * 2;
                num[i].0 = left + right;
                num[i].1 -= 1;
                num.remove(i + 1);
                found = true;
                break;
            }
        }
        if !found {
            depth -= 1;
        }
    }
    num[0].0
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
    fn unit_tests() {
        let s1 = parse_snails("[1,2]");
        let s1_parsed: Vec<(usize, usize)> = vec![(1, 1), (2, 1)];
        assert_eq!(s1_parsed, s1);

        let s2 = parse_snails("[[3,4],5]");
        let s2_parsed: Vec<(usize, usize)> = vec![(3, 2), (4, 2), (5, 1)];
        assert_eq!(s2_parsed, s2);

        let added = add_snails(&s1, &s2);
        let added_expect: Vec<(usize, usize)> = vec![(1, 2), (2, 2), (3, 3), (4, 3), (5, 2)];
        assert_eq!(added_expect, added);

        let i1 = "[[[[4,3],4],4],[7,[[8,4],9]]]";
        let i2 = "[1,1]";
        let s3 = parse_snails(&i1);
        let s4 = parse_snails(&i2);
        let mut i_parsed = add_snails(&s3, &s4);
        // explode
        i_parsed = reduce(&i_parsed);
        let mut reduced: Vec<(usize, usize)> = vec![
            (0, 4),
            (7, 4),
            (4, 3),
            (7, 3),
            (8, 5),
            (4, 5),
            (9, 4),
            (1, 2),
            (1, 2),
        ];
        assert_eq!(reduced, i_parsed);
        // explode
        i_parsed = reduce(&i_parsed);
        reduced = vec![
            (0, 4),
            (7, 4),
            (4, 3),
            (15, 3),
            (0, 4),
            (13, 4),
            (1, 2),
            (1, 2),
        ];
        assert_eq!(reduced, i_parsed);
        // split
        i_parsed = reduce(&i_parsed);
        reduced = vec![
            (0, 4),
            (7, 4),
            (4, 3),
            (7, 4),
            (8, 4),
            (0, 4),
            (13, 4),
            (1, 2),
            (1, 2),
        ];
        assert_eq!(reduced, i_parsed);
        // split
        i_parsed = reduce(&i_parsed);
        reduced = vec![
            (0, 4),
            (7, 4),
            (4, 3),
            (7, 4),
            (8, 4),
            (0, 4),
            (6, 5),
            (7, 5),
            (1, 2),
            (1, 2),
        ];
        assert_eq!(reduced, i_parsed);
        // explode
        i_parsed = reduce(&i_parsed);
        reduced = vec![
            (0, 4),
            (7, 4),
            (4, 3),
            (7, 4),
            (8, 4),
            (6, 4),
            (0, 4),
            (8, 2),
            (1, 2),
        ];
        assert_eq!(reduced, i_parsed);
        // noop
        i_parsed = reduce(&i_parsed);
        assert_eq!(reduced, i_parsed);
    }

    #[test]
    fn magnitude_is_working() {
        let mut input = "[[1,2],[[3,4],5]]";
        assert_eq!(143, magnitude(&parse_snails(&input)));
        input = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        assert_eq!(1384, magnitude(&parse_snails(&input)));
        input = "[[[[1,1],[2,2]],[3,3]],[4,4]]";
        assert_eq!(445, magnitude(&parse_snails(&input)));
        input = "[[[[3,0],[5,3]],[4,4]],[5,5]]";
        assert_eq!(791, magnitude(&parse_snails(&input)));
        input = "[[[[5,0],[7,4]],[5,5]],[6,6]]";
        assert_eq!(1137, magnitude(&parse_snails(&input)));
        input = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";
        assert_eq!(3488, magnitude(&parse_snails(&input)));
    }
}
