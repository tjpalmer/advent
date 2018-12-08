use std::io::{self, BufRead, BufReader};
use std::fs::File;

fn main() -> Try<()> {
    let file = File::open("../../input/input.txt")?;
    let mut count2 = 0;
    let mut count3 = 0;
    for line in BufReader::new(file).lines() {
        // Count chars.
        let mut counts = [0; 256];
        for c in line?.bytes() {
            counts[c as usize] += 1;
        }
        // Find 2s and 3s.
        count2 += counts.iter().any(|count| *count == 2) as i32;
        count3 += counts.iter().any(|count| *count == 3) as i32;
    }
    println!("{}", count2 * count3);
    Ok(())
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
