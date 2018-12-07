use std::collections::HashSet;
use std::io::{self, BufRead, BufReader};
use std::fs::File;

fn main() -> Try<()> {
    let values = read_values()?;
    let mut prevs = HashSet::new();
    let mut total = 0;
    'outer: loop {
        for value in &values {
            if prevs.contains(&total) {
                println!("{}", total);
                break 'outer;
            }
            prevs.insert(total);
            total += value;
        }
    }
    Ok(())
}

fn read_values() -> Try<Vec<i32>> {
    let file = File::open("../../input/input.txt")?;
    let mut values = Vec::new();
    for line in BufReader::new(file).lines() {
        values.push(line?.parse::<i32>()?);
    }
    Ok(values)
}

#[derive(Debug)]
pub struct Err {
    pub message: String,
}

type Try<Value> = Result<Value, Err>;

impl From<io::Error> for Err {
    fn from(_: io::Error) -> Err {
        Err{message: "io error".to_string()}
    }
}

impl From<std::num::ParseIntError> for Err {
    fn from(_: std::num::ParseIntError) -> Err {
        Err{message: "parse int error".to_string()}
    }
}
