use std::collections::HashMap;

pub fn run() {
    println!("Day 01");
    println!("Part 1: {}", part1("src/inputs/day01.txt".to_string()));
    println!("Part 2: {}", part2("src/inputs/day01.txt".to_string()));
}

fn part1(file_name: String) -> i32 {
    let (mut col1, mut col2) = read_input(file_name);

    // Sort column 1
    col1.sort();

    // Sort column 2
    col2.sort();

    // Find diff between sorted columns
    let mut total_diff = 0;
    for i in 0..col1.len(){
        let n1 = col1[i];
        let n2 = col2[i];
        // Subtract smaller from larger
        if n1 > n2 {
            total_diff += n1 - n2;
        } else {
            total_diff += n2 - n1;
        }
    }

    total_diff

}

fn part2(file_name: String) -> i32 {
    let (col1, col2) = read_input(file_name);

    let mut occurence_map: HashMap<i32, i32> = HashMap::new();
    
    for num in &col2 {
        *occurence_map.entry(*num).or_insert(0) += 1;
    }

    let mut simmilarity_score = 0;
    for num in &col1 {
        if occurence_map.contains_key(num) {
            simmilarity_score += num * occurence_map[num];
        }
    }

    simmilarity_score
}



fn read_input(file_name: String) -> (Vec<i32>, Vec<i32>) {
    let input = std::fs::read_to_string(file_name)
        .expect("Failed to read input file");
    
    input.lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let num1 = parts.next().unwrap().parse::<i32>().unwrap();
            let num2 = parts.next().unwrap().parse::<i32>().unwrap();
            (num1, num2)
        })
        .unzip()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_example() {
        let solution = part1("src/inputs/day01_test.txt".to_string());
        assert_eq!(solution, 11);
    }

    #[test]
    fn test_part2_with_example() {
        let solution = part2("src/inputs/day01_test.txt".to_string());
        assert_eq!(solution, 31);
    }
}
