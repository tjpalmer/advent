use std::collections::HashSet;
use std::io::{self, BufRead, BufReader};
use std::fs::File;

fn main() -> io::Result<()> {
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

fn read_values() -> io::Result<Vec<i32>> {
    let file = File::open("../../input/input.txt")?;
    let mut values = Vec::new();
    for line in BufReader::new(file).lines() {
        values.push(line.unwrap().parse::<i32>().unwrap());
    }
    Ok(values)
}
