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
  fn task_1(&self, filename: String) -> Result<i64, Box<dyn Error>>;
  fn task_2(&self, filename: String) -> Result<i64, Box<dyn Error>>;
}

pub struct Exercise { }

impl Exercise {
  pub fn run(args: Vec<String>, solution: &dyn SolutionT)
        -> Result<i64, Box<dyn Error>> {
    let config: Config = Config::new(&args).unwrap();
    println!("Running {}", config.task);
    println!("With file {}", config.filename);

    let r = match &config.task[..] {
      "1" => solution.task_1(config.filename),
      "2" => solution.task_2(config.filename),
      _ => simple_error::bail!("Invalid task: {}", config.task),
    };
    println!("Result for task {}: {:?}", config.task, r);
    r
  }
}