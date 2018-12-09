use std::io::{self, BufRead, BufReader};
use std::fs::File;

fn main() -> Try<()> {
    let file = File::open("../../input/input.txt")?;
    let mut ids: Vec<Vec<char>> = Vec::new();
    'lines: for line in BufReader::new(file).lines() {
        let id: Vec<char> = line?.chars().collect();
        'others: for other in ids.iter() {
            let mut diff_index = -1;
            for (index, (c0, c1)) in id.iter().zip(other.iter()).enumerate() {
                if c0 != c1 {
                    if diff_index >= 0 {
                        // More than one different.
                        continue 'others;
                    } else {
                        diff_index = index as i32;
                    }
                }
            }
            if diff_index >= 0 {
                // If we get here, it's because we had only one different.
                let a: String = id[..diff_index as usize].iter().collect();
                let b: String = id[diff_index as usize + 1..].iter().collect();
                println!("{}{}", a, b);
                break 'lines;
            }
        }
        ids.push(id);
    }
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
