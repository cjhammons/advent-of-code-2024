use std::collections::HashMap;
use std::collections::HashSet;

type OrderingMap = HashMap<i32, Vec<i32>>;
type PageNumers = Vec<Vec<i32>>;


pub fn run() {
    println!("Day 05");
    let (res1, res2) = both_parts("src/inputs/day05.txt".to_string());
    println!("Part 1: {}", res1);
    println!("Part 2: {}", res2);
}

/*  I broke with my convention on this one and did both parts in the same function.
    The reasoning: Part 2 takes the complement of the page list from part 1, so all values in
    the file get used. re-reading the same file an doing the same sorting but reversed seemed 
    like a waste of resources
*/
fn both_parts(file_name: String) -> (i32, i32) {
    // Get the two parts from the file
    let (map, page_sequences) = read_file(file_name);

    // Figure out which page sequences are valid
    let mut res1= 0;
    let mut res2 = 0;
    for page_sequence in page_sequences {
        if is_valid_sequence(&map, &page_sequence) {
            let midpoint = page_sequence.len() / 2;
            res1 += page_sequence[midpoint];
        } else {
            let ordered = correct_order(&map, &page_sequence);
            let midpoint = ordered.len() / 2;
            res2 += ordered[midpoint];
        }
    }

    return (res1, res2)
}

/*
Given a page sequence in the incorrect order, this function returns the sequence sorted so that it 
follows the rules given in the ordering map.
*/
fn correct_order(map: &OrderingMap, page_nums: &Vec<i32>) -> Vec<i32> {
    

    return page_nums;
}

// Checks each sequence of pages and checks if they follow the rules in the Ordering map
fn is_valid_sequence(map: &OrderingMap, page_nums: &Vec<i32>) -> bool {
    let mut preceding: HashSet<i32> = HashSet::new();
    for page in page_nums {
        if let Some(nums) = map.get(page) {
            for num in nums {
                if preceding.contains(num) {
                    return false;
                }
            }
        }
        preceding.insert(*page);
    }
    return true
}

// Separates the two part file into the ordering specs and the page sequences
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

mod tests {
    use super::*;

    #[test]
    fn test_part1_with_example() {
        let solution = both_parts("src/inputs/day05_test.txt".to_string());
        assert_eq!(solution.0, 143);
        assert_eq!(solution.1, 123);
    }
}