use std::collections::HashMap;

fn part1(input: &Vec<String>) -> i32 {
    let table = parse_input(input);
    let (x, y) = find_first_position(&table);
    let mut ans = 1;
    let mut visited = HashMap::new();
    visited.insert((x, y), 0);
    let mut current = (x, y);
    let mut direction = (-1, 0);
    loop {
        let new_x = current.0 + direction.0;
        let new_y = current.1 + direction.1;
        if new_x < 0 || new_x >= table.len() as i32 || new_y < 0 || new_y >= table[0].len() as i32 {
            return ans;
        } else {
            if table[new_x as usize][new_y as usize] == '#' {
                direction = match direction {
                    (-1, 0) => (0, 1),
                    (0, 1) => (1, 0),
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    _ => panic!("Invalid direction"),
                }
            } else {
                if !visited.contains_key(&(new_x, new_y)) {
                    visited.insert((new_x, new_y), 0);
                    ans += 1;
                }
                current = (new_x, new_y);
            }
        }
    }
}

fn part2(input: &Vec<String>) -> i32 {
    let table = parse_input(input);
    let (x, y) = find_first_position(&table);
    let mut ans = 0;
    for i in 0..table.len() {
        for j in 0..table[0].len() {
            if i == x as usize && j == y as usize {
                continue;
            }
            let mut new_table = table.clone();
            new_table[i][j] = '#';
            let mut current = (x, y);
            let mut visited = HashMap::new();
            let mut direction = (-1, 0);
            visited.insert((x, y, direction), 0);
            loop {
                let new_x = current.0 + direction.0;
                let new_y = current.1 + direction.1;
                if new_x < 0
                    || new_x >= new_table.len() as i32
                    || new_y < 0
                    || new_y >= new_table[0].len() as i32
                {
                    break;
                } else {
                    if new_table[new_x as usize][new_y as usize] == '#' {
                        direction = match direction {
                            (-1, 0) => (0, 1),
                            (0, 1) => (1, 0),
                            (1, 0) => (0, -1),
                            (0, -1) => (-1, 0),
                            _ => panic!("Invalid direction"),
                        }
                    } else {
                        if visited.contains_key(&(new_x, new_y, direction)) {
                            ans += 1;
                            break;
                        } else {
                            visited.insert((new_x, new_y, direction), 0);
                        }
                        current = (new_x, new_y);
                    }
                }
            }
        }
    }
    ans
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<char>> {
    let mut table = Vec::new();

    for line in input {
        let char_vec: Vec<char> = line.chars().collect();
        table.push(char_vec)
    }

    table
}

fn find_first_position(input: &Vec<Vec<char>>) -> (i32, i32) {
    let mut row = 0;
    let mut col = 0;
    for i in 0..input.len() {
        if input[i].contains(&'^') {
            row = i;
            col = input[i].iter().position(|&r| r == '^').unwrap();
        }
    }
    (row as i32, col as i32)
}

pub fn run(input: &Vec<String>) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
