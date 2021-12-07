use std::fs;
use std::io::{BufRead, BufReader, Error};

pub fn read_ints(filename: String) -> Result<Vec<i64>, Error> {
  let io = fs::File::open(filename)?;
  let br = BufReader::new(io);
  let v: Vec<i64> = br.lines()
                    .map(|line| line.unwrap().parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
  Ok(v)
}

pub fn read_binary(filename: String) -> Result<Vec<i64>, Error> {
  let io = fs::File::open(filename)?;
  let br = BufReader::new(io);
  let v: Vec<i64> = br.lines()
        .map(|line| i64::from_str_radix(&line.unwrap(), 2).unwrap())
        .collect::<Vec<i64>>();
  Ok(v)
}

pub fn read_lines(filename: String) -> Result<Vec<String>, Error> {
  let io = fs::File::open(filename)?;
  let br = BufReader::new(io);
  let v: Vec<String> = br.lines().collect::<Result<_,_>>().unwrap();
  Ok(v)
}
