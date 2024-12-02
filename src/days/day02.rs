use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run() {
    println!("Day 02");
    println!("Part 1: {}", part1("src/inputs/day02.txt".to_string()));}

fn part1(file_name: String) -> i32 {
    let file = File::open(file_name).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut num_safe = 0;
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse number"))
            .collect();
        
        if is_safe(&numbers) {
            num_safe += 1
        }
    }
    return num_safe
}

fn is_safe(nums: &Vec<i32>) -> bool {
    let mut increasing: Option<bool> = None; 
    for (i, _) in nums.iter().enumerate() {
        if i + 1 >= nums.len() {
            break;
        }

        let diff = nums[i+1] - nums[i];

        if diff == 0 {
            return false;
        }

        // Set direction on first difference
        if increasing.is_none() {
            increasing = Some(diff > 0);
            if diff.abs() > 3 {
                return false;
            }
            continue;
        }

        // Check if direction matches established pattern
        if (diff > 0) != increasing.unwrap() {
            return false;
        }

        // Check if difference is in acceptable range
        if diff.abs() > 3 || diff.abs() < 1  {
            return false
        }
    }
    return true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_example() {
        let solution = part1("src/inputs/day02_test.txt".to_string());
        assert_eq!(solution, 2);
    }
}