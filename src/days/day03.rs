use clap::builder::Str;
use regex::Regex;

pub fn run() {
    println!("Day 03");
    println!("Part 1: {}", part1("src/inputs/day03.txt".to_string()));
    println!("Part 2: {}", part2("src/inputs/day03.txt".to_string()));
}

const MUL_REGEX: &str = r"mul\((-?\d+),(-?\d+)\)";

fn part1(file_name: String) -> i32 {
    let input = read_file(file_name);
    let matches = extract_matches(&input, MUL_REGEX).unwrap();
    let mut total = 0;  
    for m in matches {
        total += parse_and_multiply(&m);
    }
    return total
}

fn part2(file_name: String) -> i32 {
    0
}

fn read_file(file_name: String) -> String{
    std::fs::read_to_string(file_name)
        .expect("Failed to read input file")
}

fn extract_matches(text: &str, pattern: &str) -> Result<Vec<String>, regex::Error> {
    let regex = Regex::new(pattern)?;
    Ok(regex.find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect())
}

fn parse_and_multiply(input: &str) -> i32 {
    let trimmed = &input[4..input.len()-1];
    let parts: Vec<&str> = trimmed.split(",").collect();
    let num1: i32 = parts[0].parse().unwrap();
    let num2: i32 = parts[1].parse().unwrap();
    return num1 * num2
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_example() {
        let solution = part1("src/inputs/day03_test.txt".to_string());
        assert_eq!(solution, 161);
    }
}