mod day1;
mod day2;
use core::num;
use std::fs::File;
use std::i64;
use std::io::{self, BufRead};
use std::iter::zip;
use std::path::Path;
use std::str::FromStr;

fn main() {
    let mut safe_count = 0;
    if let Ok(lines) = read_lines("./day2b.txt") {
        for line in lines.flatten() {
            let nums: Vec<_> = line
                .split_whitespace()
                .map(i32::from_str)
                .map(|i| i.unwrap())
                .collect();
            if check_safe(&nums) {
                safe_count += 1;
            }
        }
    }
    dbg!(safe_count);

    safe_count = 0;
    if let Ok(lines) = read_lines("./day2b.txt") {
        for line in lines.flatten() {
            let nums: Vec<_> = line
                .split_whitespace()
                .map(i32::from_str)
                .map(|i| i.unwrap())
                .collect();
            if check_safe(&nums) {
                safe_count += 1;
            } else {
                let mut safe_found = false;
                for i in 0..nums.len() {
                    if check_safe(&[&nums[..i], &nums[i + 1..]].concat()) {
                        safe_found = true;
                        break;
                    }
                }
                if safe_found {
                    safe_count += 1;
                }
            }
        }
    }
    dbg!(safe_count);
}

fn check_safe(nums: &Vec<i32>) -> bool {
    let sign = i32::signum(nums[1] - nums[0]);
    if sign == 0 {
        return false;
    }
    for i in 0..(nums.len() - 1) {
        let diff = nums[i + 1] - nums[i];
        if diff.signum() != sign || diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
    }
    return true;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
