
pub fn run() {
    println!("Day 05");
    println!("Part 1: {}", part1("src/inputs/day05.txt".to_string()));
    println!("Part 2: {}", part2("src/inputs/day05.txt".to_string()));
}

fn part1(file_name: String) -> i32 {
    return 0
}

fn part2(file_name: String) -> i32 {
    return 0
}

mod tests {
    use super::*;

    #[test]
    fn test_part1_with_example() {
        let solution = part1("src/inputs/day05_test.txt".to_string());
        assert_eq!(solution, 18);
    }

    // #[test]
    // fn test_part2_with_example() {
    //     let solution = part2("src/inputs/day04_test.txt".to_string());
    //     assert_eq!(solution, 9);
    // }
}