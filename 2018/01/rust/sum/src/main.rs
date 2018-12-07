use std::io::{self, BufRead, BufReader};
use std::fs::File;

fn main() -> io::Result<()> {
    let values = read_values()?;
    println!("{}", values.iter().sum::<i32>());
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
