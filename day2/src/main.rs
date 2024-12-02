use core::panic;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let buf = BufReader::new(file);
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for maybe_line in buf.lines() {
        match maybe_line {
            Ok(line) => {
                reports.push(
                    line.split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect::<Vec<i32>>(),
                );
            }
            Err(_) => panic!("Error reading liens"),
        }
    }

    let safe_reports_part1: Vec<&Vec<i32>> =
        reports.iter().filter(|report| part1(report)).collect();
    println!("Safe Reports Part1: {}", safe_reports_part1.len());

    Ok(())
}

fn part1(report: &[i32]) -> bool {
    let mut state: Option<&str> = None;

    for window in report.windows(2) {
        let (prev, curr) = (window[0], window[1]);

        if (prev - curr).abs() > 3 {
            return false;
        }

        match state {
            Some("up") if prev >= curr => return false,
            Some("down") if prev <= curr => return false,
            _ => {
                state = Some(match prev.cmp(&curr) {
                    std::cmp::Ordering::Less => "up",
                    std::cmp::Ordering::Equal => return false,
                    std::cmp::Ordering::Greater => "down",
                })
            }
        }
    }

    true
}
