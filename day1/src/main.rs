use std::fs;
use std::collections::HashSet;

/// https://adventofcode.com/2020/day/1
fn main() {
   part_one();
   part_two();
}

fn read_file(file_name: String) -> Vec<i32> {
    let file = fs::read_to_string(file_name).unwrap();
    file
    .lines()
    .map(|line| line.trim().parse::<i32>().unwrap())
    .collect()
}


fn part_one() {
    const YEAR: i32 = 2020;

    let mut complements: HashSet<i32> = HashSet::new();
    let file = read_file(String::from("input.txt"));

    for val in &file { 
        let complement = YEAR - val;
        if complements.contains(&complement) {
            println!("{}", val * complement);
        } else {
            complements.insert(*val);
        }
    }
}

fn part_two() {
    const YEAR: i32 = 2020;
    let mut file = read_file(String::from("input.txt"));
    file.sort();

    for i in 0..file.len() {
        let mut left = i + 1;
        let mut right = file.len() - 1;
        let val = file[i];
        while left < right {
            let l_val = file[left];
            let r_val = file[right];
            if val + l_val + r_val == YEAR {
                println!("{}", val * l_val * r_val);
                break;
            } else if val + l_val + r_val < YEAR {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
}