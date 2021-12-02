use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

pub fn read_ints<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    let mut v = vec![];
    for line in br.lines() {
        v.push(line?
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?);
    }
    Ok(v)
}

pub fn read_lines<R: Read>(io: R) -> Result<Vec<String>, Error> {
  let br = BufReader::new(io);
  let mut v: Vec<String> = vec![];
  for line in br.lines() {
    v.push(line?.trim().to_string());
  }
  Ok(v)
}
