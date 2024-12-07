use std::collections::HashSet;

pub fn run() {
    println!("Day 06");
    println!("Part 1: {}", part1("src/inputs/day06.txt".to_string()));
    println!("Part 2: {}", part2("src/inputs/day06.txt".to_string()));
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

type Coordinates = (usize, usize);

struct Guard {
    position: Coordinates,
    current_direction: Direction,
    map: Vec<Vec<char>>,
    in_area: bool
}

impl Guard {

    fn new(raw_map: Vec<Vec<char>>) -> Guard {
        let mut position: Coordinates = (0, 0);
        let mut current_direction: Direction = Direction::Up;
        
        for (i, row) in raw_map.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                if raw_map[i][j] != '.' && raw_map[i][j] != '#' {
                    position = (i, j);
                    match raw_map[i][j] {
                        '^' => current_direction = Direction::Up,
                        'v' => current_direction = Direction::Down,
                        '<' => current_direction = Direction::Left,
                        '>' => current_direction = Direction::Right,
                        _ => panic!("Invalid direction")
                    }
                    break;
                }
            }
        }

        return Guard {
            position,
            current_direction,
            map: raw_map,
            in_area: true
        }
    }

    fn new_from_file(file_name: String) -> Guard {
        let contents = std::fs::read_to_string(file_name).expect("Failed to read file");
        return Guard::new(contents.lines().map(|line| line.chars().collect()).collect());
    }

    fn move_position(&mut self) -> Coordinates {
        match self.current_direction {
            Direction::Up => {
                if self.position.0 == 0 {
                    self.in_area = false;
                    return self.position;
                }
                if self.map[self.position.0 - 1][self.position.1] == '#' {
                    self.current_direction = Direction::Right;
                    return self.move_position();
                }
                self.position.0 -= 1;
            }
            Direction::Down => {
                if self.position.0 + 1 >= self.map.len() {
                    self.in_area = false;
                    return self.position;
                }
                if self.map[self.position.0 + 1][self.position.1] == '#' {
                    self.current_direction = Direction::Left;
                    return self.move_position();
                }
                self.position.0 += 1;
            }
            Direction::Left => {
                if self.position.1 == 0 {
                    self.in_area = false;
                    return self.position;
                }
                if self.map[self.position.0][self.position.1 - 1] == '#' {
                    self.current_direction = Direction::Up;
                    return self.move_position();
                }
                self.position.1 -= 1;
            }
            Direction::Right => {
                if self.position.1 + 1 >= self.map[0].len() {
                    self.in_area = false;
                    return self.position;
                }
                if self.map[self.position.0][self.position.1 + 1] == '#' {
                    self.current_direction = Direction::Down;
                    return self.move_position();
                }
                self.position.1 += 1;
            }
        }

        // Check if the guard has left the area
        if self.position.0 >= self.map.len() {
            self.in_area = false;
        }
        if self.position.1 >= self.map[0].len() {
            self.in_area = false;
        }

        return self.position;
    }
}

fn part1(file_name: String) -> usize {
    let mut guard = Guard::new_from_file(file_name);
    let mut visited: HashSet<Coordinates> = HashSet::new();
    visited.insert(guard.position);
    loop {
        let new_position = guard.move_position();
        if !guard.in_area {
            break; 
        } else {
            visited.insert(new_position);
        }
    }

    return visited.len();
}

fn part2(file_name: String) -> i32 {

    return 0
}


mod tests {
    use super::*;

    #[test]
    fn test_part1_with_example() {
        let solution = part1("src/inputs/day06_test.txt".to_string());
        assert_eq!(solution, 41);
    }
}