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
    let safe_reports_part2: Vec<&Vec<i32>> =
        reports.iter().filter(|report| part2(report)).collect();
    println!("Safe Reports Part2: {:?}", safe_reports_part2.len());

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

fn part2(report: &[i32]) -> bool {
    for i in 0..report.len() {
        let filtered: Vec<i32> = report
            .iter()
            .enumerate()
            .filter(|&(index, _)| index != i)
            .map(|(_, &val)| val)
            .collect();

        if is_valid_sequence(&filtered) {
            return true;
        }
    }
    false
}

fn is_valid_sequence(seq: &[i32]) -> bool {
    let is_increasing = seq.windows(2).all(|w| w[0] < w[1]);
    let is_decreasing = seq.windows(2).all(|w| w[0] > w[1]);

    let valid_differences = seq.windows(2).all(|w| (w[0] - w[1]).abs() <= 3);

    (is_increasing || is_decreasing) && valid_differences
}
