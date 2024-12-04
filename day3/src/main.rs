use regex::Regex;
use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    // let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    reader.read_to_string(&mut buf).unwrap();

    part1(&buf);
}

fn part1(data: &str) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches: Vec<_> = re
        .captures_iter(data)
        .map(|cap| {
            (
                cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            )
        })
        .collect();
    let res: i32 = matches.iter().fold(0, |state, m| (m.0 * m.1) + state);
    println!("{:?}", res);
}
