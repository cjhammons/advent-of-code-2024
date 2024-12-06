use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type OrderingMap = HashMap<i32, Vec<i32>>;
type PageNumers = Vec<Vec<i32>>;


pub fn run() {
    println!("Day 05");
    println!("Part 1: {}", part1("src/inputs/day05.txt".to_string()));
    println!("Part 2: {}", part2("src/inputs/day05.txt".to_string()));
}

fn part1(file_name: String) -> i32 {
    let (map, pages) = read_file(file_name);
    println!("{:?}", map);
    println!("{:?}", pages);
    return 0
}

fn read_file(file_name: String) -> (OrderingMap, PageNumers) {
    let contents = std::fs::read_to_string(file_name).expect("Failed to read file");
    let groups: Vec<&str> = contents.split("\n\n").collect();

    let mut map = OrderingMap::new();
    for line in groups[0].lines() {
        let parts: Vec<&str> = line.split("|").collect();
        map.entry(parts[0].parse().unwrap())
            .or_insert(Vec::new())
            .push(parts[1].parse().unwrap())
    }

    let mut pages = PageNumers::new();
    for line in groups[1].lines() {
        pages.push(line.split(",").map(|s| s.parse().unwrap()).collect());
    }


    return (map, pages)
}


fn part2(file_name: String) -> i32 {
    return 0
}

mod tests {
    use super::*;

    #[test]
    fn test_part1_with_example() {
        let solution = part1("src/inputs/day05_test.txt".to_string());
        assert_eq!(solution, 143);
    }

    // #[test]
    // fn test_part2_with_example() {
    //     let solution = part2("src/inputs/day04_test.txt".to_string());
    //     assert_eq!(solution, 9);
    // }
}