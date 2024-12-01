use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line_maybe in reader.lines() {
        match line_maybe {
            Ok(line) => {
                let arr = line.split_whitespace().collect::<Vec<&str>>();
                left.push(arr[0].parse().unwrap());
                right.push(arr[1].parse().unwrap());
            }
            Err(_) => todo!(),
        }
    }
    left.sort();
    right.sort();
    let mut res = 0;
    for i in 0..left.len() {
        res += (left[i] - right[i]).abs();
    }

    println!("{}", res);
    Ok(())
}
