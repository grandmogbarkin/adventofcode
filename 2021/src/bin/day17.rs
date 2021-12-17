use std::error::Error;

use advent2021::exercise::{Exercise, SolutionT};
use advent2021::read;
use regex::Regex;

struct Solution {}

impl SolutionT for Solution {
    fn day(&self) -> &str {
        "17"
    }

    fn test1_result(&self) -> i64 {
        45
    }
    fn test2_result(&self) -> i64 {
        112
    }

    fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let input = read::read_lines(filename)?;

        let re =
            Regex::new(r"^target area: x=([-\d]+)\.\.([-\d]+), y=([-\d]+)..([-\d]+)$").unwrap();
        let cap = re.captures(&input[0]).unwrap();

        println!("Bounds: {:?}", cap);
        let y_bottom = cap[3].parse::<i64>().unwrap();

        // Vertical speed has no relation to horizontal position, and since horizontally the probe
        //   will always stop moving at some point, only vertical speed matters.
        // Vertically, any speed we shoot up with, will eventually be the speed the probe will be
        //   coming down at. Maximum height means maximum downwards velocity, which will be the speed
        //   at which point the probe at 0 is exactly 1 step away from the bottom of the target area,
        //   which is a distance of y_bottom.
        // So we need to shoot up at a velocity of n = -y_bottom - 1.
        // Max height is n/2 * n+1
        let n = -y_bottom - 1;
        Ok(n * (n + 1) / 2)
    }

    fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>> {
        let input = read::read_lines(filename)?;

        let re =
            Regex::new(r"^target area: x=([-\d]+)\.\.([-\d]+), y=([-\d]+)..([-\d]+)$").unwrap();
        let cap = re.captures(&input[0]).unwrap();

        println!("Bounds: {:?}", cap);
        let (x_left, x_right, y_bottom, y_top) = (
            cap[1].parse::<i64>().unwrap(),
            cap[2].parse::<i64>().unwrap(),
            cap[3].parse::<i64>().unwrap(),
            cap[4].parse::<i64>().unwrap(),
        );
        // We're assuming t_x>=0, t_y<=0 because rust ranges don't work backwards.
        let mut count = 0;
        // have to push to the right, max initial velocity is x_right.
        for x_velocity in 0..x_right + 1 {
            // Already established max upwards y velocity is -y_bottom - 1 in part 1.
            // Also max downwards velocity is y_bottom.
            for y_velocity in y_bottom..-y_bottom {
                let mut x_v = x_velocity;
                let mut y_v = y_velocity;
                let (mut x, mut y) = (0, 0);
                while x <= x_right && y >= y_bottom {
                    if x >= x_left && y <= y_top {
                        // println!("{},{}", x_velocity, y_velocity);
                        count += 1;
                        break;
                    }
                    x += x_v;
                    y += y_v;
                    y_v -= 1;
                    x_v = std::cmp::max(0, x_v - 1);
                }
            }
        }
        Ok(count)
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
