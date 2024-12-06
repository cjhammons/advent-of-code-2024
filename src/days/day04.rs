use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run() {
    println!("Day 04");
    println!("Part 1: {}", part1("src/inputs/day04.txt".to_string()));
    println!("Part 2: {}", part2("src/inputs/day04.txt".to_string()));
}

fn part1(file_name: String) -> i32 {
    let input = read_file(file_name);
    let mut count = 0;
    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'X' {
                // Check horizontal right
                if (j + 3 < line.len()) 
                    && (input[i][j+1] == 'M' && input[i][j+2] == 'A' && input[i][j+3] == 'S') {
                    count += 1;
                }

                // Check horizontal left
                if (j >= 3) 
                    && (input[i][j-1] == 'M' && input[i][j-2] == 'A' && input[i][j-3] == 'S') {
                    count += 1;
                }

                // Check vertical down
                if (i + 3 < input.len()) 
                    && (j < input[i+1].len() && j < input[i+2].len() && j < input[i+3].len())
                    && (input[i+1][j] == 'M' && input[i+2][j] == 'A' && input[i+3][j] == 'S') {
                    count += 1;
                }

                // Check vertical up
                if (i >= 3) 
                    && (j < input[i-1].len() && j < input[i-2].len() && j < input[i-3].len())  
                    && (input[i-1][j] == 'M' && input[i-2][j] == 'A' && input[i-3][j] == 'S') {
                    count += 1;
                }

                // Check Diagonal down-right
                if (i + 3 < input.len() && j + 3 < line.len()) 
                    && (j+1 < input[i+1].len() && j+2 < input[i+2].len() && j+3 < input[i+3].len()) 
                    && (input[i+1][j+1] == 'M' && input[i+2][j+2] == 'A' && input[i+3][j+3] == 'S') {
                    count += 1;
                }

                // Check Diagonal up-left
                if (i >= 3 && j >= 3) 
                    && (j-1 < input[i-1].len() && j-2 < input[i-2].len() && j-3 < input[i-3].len())  
                    && (input[i-1][j-1] == 'M' && input[i-2][j-2] == 'A' && input[i-3][j-3] == 'S') {
                    count += 1;
                }

                // Check Diagonal down-left
                if (i + 3 < input.len() && j >= 3)
                    && (j-1 < input[i+1].len() && j-2 < input[i+2].len() && j-3 < input[i+3].len())
                    && (input[i+1][j-1] == 'M' && input[i+2][j-2] == 'A' && input[i+3][j-3] == 'S') {
                    count += 1;
                }

                // Check Diagonal up-right
                if (i >= 3 && j + 3 < line.len())
                    && (j+1 < input[i-1].len() && j+2 < input[i-2].len() && j+3 < input[i-3].len())
                    && (input[i-1][j+1] == 'M' && input[i-2][j+2] == 'A' && input[i-3][j+3] == 'S') {
                    count += 1;
                }
            }
        }
    }
    return count
}

fn part2(file_name: String) -> i32 {
    let input = read_file(file_name);
    let mut count = 0;
    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'A' {
                if i > 0 && i < input.len() - 1 && j > 0 && j < line.len() - 1 {
                    let top_left = input[i-1][j-1];
                    let top_right = input[i-1][j+1];
                    let bottom_left  = input[i+1][j-1];
                    let bottom_right = input[i+1][j+1];

                    let is_mas = |start: char, end: char| {
                        (start == 'M' && end == 'S') || (start == 'S' && end == 'M')
                    };

                    let diagnols = [
                        (is_mas(top_left, bottom_right) && is_mas(top_right, bottom_left))
                    ];

                    if diagnols.iter().any(|&valid| valid) {
                        count += 1;
                    }
                }
            }
        }
    }
    return count
}

fn read_file(file_name: String) -> Vec<Vec<char>> {
    let file = File::open(file_name).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut res = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(l) => {
                let chars: Vec<char> = l.chars().collect();
                res.push(chars);
            }
            Err(e) => eprintln!("Error reading line: {}", e)
        }
    }
    return res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_example() {
        let solution = part1("src/inputs/day04_test.txt".to_string());
        assert_eq!(solution, 18);
    }

    #[test]
    fn test_part2_with_example() {
        let solution = part2("src/inputs/day04_test.txt".to_string());
        assert_eq!(solution, 9);
    }
}