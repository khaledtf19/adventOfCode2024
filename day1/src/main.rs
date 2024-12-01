use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut scores: HashMap<i32, i32> = HashMap::new();

    for line_maybe in reader.lines() {
        match line_maybe {
            Ok(line) => {
                let arr = line.split_whitespace().collect::<Vec<&str>>();
                left.push(arr[0].parse().unwrap());
                right.push(arr[1].parse().unwrap());
                if let Some(score) = scores.get(&arr[1].parse().unwrap()) {
                    scores.insert(arr[1].parse().unwrap(), score + 1);
                } else {
                    scores.insert(arr[1].parse().unwrap(), 1);
                }
            }
            Err(_) => todo!(),
        }
    }
    left.sort();
    right.sort();
    part1(&left, &right);
    part2(&left, &scores);

    Ok(())
}

fn part1(left: &Vec<i32>, right: &Vec<i32>) {
    let mut res = 0;
    for i in 0..left.len() {
        res += (left[i] - right[i]).abs();
    }

    println!("{}", res);
}

fn part2(left: &Vec<i32>, scores: &HashMap<i32, i32>) {
    let mut res = 0;
    for value in left.iter() {
        if let Some(score) = scores.get(value) {
            res += (score * value)
        }
    }
    println!("{}", res);
}
