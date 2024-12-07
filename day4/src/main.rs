use std::{
    char,
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

const DIRS_PART1: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const DIRS_PART2: [(i32, i32); 4] = [(-1, -1), (1, 1), (-1, 1), (1, -1)];

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
    part2(&map);
}

fn part2(map: &[Vec<char>]) {
    let mut count = 0;
    for i in 0..map.len() {
        for ii in 0..map[0].len() {
            if map[i][ii] == 'A' && walk_part2(map, (i as i32, ii as i32)) {
                count += 1;
            }
        }
    }
    println!("part2 => {}", count);
}

fn walk_part2(map: &[Vec<char>], main_point: (i32, i32)) -> bool {
    let mut res: String = "".to_string();
    for dir in DIRS_PART2 {
        let new_point = (main_point.0 + dir.0, main_point.1 + dir.1);

        if new_point.0 < map.len() as i32
            && new_point.1 < map[0].len() as i32
            && new_point.1 >= 0
            && new_point.0 >= 0
        // && map[new_point.0 as usize][new_point.1 as usize] == curr_char
        {
            res.push(map[new_point.0 as usize][new_point.1 as usize]);
        } else {
            return false;
        }
    }
    let split = res.split_at(2);
    let c = ["MS", "SM"];
    if c.contains(&split.0) && c.contains(&split.1) {
        return true;
    }
    false
}

fn part1(map: &[Vec<char>]) {
    let mut count = 0;
    for i in 0..map.len() {
        for ii in 0..map[0].len() {
            if map[i][ii] == 'X' {
                for &dir in DIRS_PART1.iter() {
                    if walk(dir, map, (i as i32, ii as i32), 0) {
                        count += 1;
                    }
                }
            }
        }
    }
    println!("part1 => {}", count);
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
