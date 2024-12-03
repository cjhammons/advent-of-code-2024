pub fn run() {
    println!("Day 03");
    println!("Part 1: {}", part1("src/inputs/day03.txt".to_string()));
    println!("Part 2: {}", part2("src/inputs/day03.txt".to_string()));
}

fn part1(file_name: String) -> i32 {
    0
}

fn part2(file_name: String) -> i32 {
    0
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