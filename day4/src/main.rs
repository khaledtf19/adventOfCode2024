use std::{
    char,
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const XMAS: &str = "XMAS";

fn main() {
    // let split = XMAS.split("").enumerate();

    let file = File::open("input.txt").unwrap();
    let buf = BufReader::new(file);
    let mut map: Vec<Vec<char>> = vec![];

    for maybe_line in buf.lines() {
        match maybe_line {
            Ok(line) => {
                map.push(line.chars().collect());
            }
            Err(_) => todo!(),
        }
    }
    part1(&map);
}

fn part1(map: &[Vec<char>]) {
    let mut count = 0;
    for i in 0..map.len() {
        for ii in 0..map[0].len() {
            if map[i][ii] == 'X' {
                for &dir in DIRS.iter() {
                    if walk(dir, map, (i as i32, ii as i32), 0) {
                        count += 1;
                    }
                }
            }
        }
    }
    println!("{}", count);
}

fn walk(dir: (i32, i32), map: &[Vec<char>], curr_point: (i32, i32), curr_num: usize) -> bool {
    if curr_num == 4 {
        return true;
    }
    let curr_char = XMAS.chars().nth(curr_num).unwrap();

    if curr_point.0 < map.len() as i32
        && curr_point.1 < map[0].len() as i32
        && curr_point.1 >= 0
        && curr_point.0 >= 0
        && curr_num < XMAS.len()
        && map[curr_point.0 as usize][curr_point.1 as usize] == curr_char
    {
        return walk(
            dir,
            map,
            (curr_point.0 + dir.0, curr_point.1 + dir.1),
            curr_num + 1,
        );
    }
    false
}
