use std::io::{BufRead, BufReader, Error, Read};

pub fn read_ints<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    let v: Vec<i64> = br.lines()
                      .map(|line| line.unwrap().parse::<i64>().unwrap())
                      .collect::<Vec<i64>>();
    Ok(v)
}

pub fn read_binary<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    let v: Vec<i64> = br.lines()
          .map(|line| i64::from_str_radix(&line.unwrap(), 2).unwrap())
          .collect::<Vec<i64>>();
    Ok(v)
}

pub fn read_lines<R: Read>(io: R) -> Result<Vec<String>, Error> {
  let br = BufReader::new(io);
  let v: Vec<String> = br.lines().collect::<Result<_,_>>().unwrap();
  Ok(v)
}
