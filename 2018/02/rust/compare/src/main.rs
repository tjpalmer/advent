use std::io::{self, BufRead, BufReader};
use std::fs::File;

fn main() -> Try<()> {
    let file = File::open("../../input/input.txt")?;
    let mut ids: Vec<String> = Vec::new();
    'lines: for line in BufReader::new(file).lines() {
        // Count chars.
        let id = line?;
        'others: for other in ids.iter() {
            let mut diff_index = -1;
            for (index, (c0, c1)) in id.chars().zip(other.chars()).enumerate() {
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
                let (a, b) = id.split_at(diff_index as usize);
                let c: String = b.chars().skip(1).collect();
                println!("{}{}", a, c);
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
