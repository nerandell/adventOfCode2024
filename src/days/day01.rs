use std::collections::HashMap;

fn part1(input: &Vec<String>) -> i32 {
    let (mut col1, mut col2) = parse_input(input);
    let mut sum = 0;
    col1.sort();
    col2.sort();

    for i in 0..col1.len() {
        sum += (col1[i] - col2[i]).abs()
    }

    sum
}

fn part2(input: &Vec<String>) -> i32 {
    let (col1, col2) = parse_input(input);
    let mut frequency: HashMap<i32, i32> = HashMap::new();
    for n in col2 {
        if let Some(value) = frequency.get(&n) {
            frequency.insert(n, value + 1);
        } else {
            frequency.insert(n, 1);
        }
    }

    let mut score = 0;
    for n in col1 {
        score += n * frequency.get(&n).unwrap_or(&0)
    }
    score
}

fn parse_input(input: &Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    for line in input {
        let cols: Vec<&str> = line.trim().split_whitespace().collect();

        if cols.len() == 2 {
            col1.push(cols[0].parse::<i32>().unwrap());
            col2.push(cols[1].parse::<i32>().unwrap());
        } else {
            eprintln!("Invalid format in line: {}", line);
        }
    }

    (col1, col2)
}

pub fn run(input: &Vec<String>) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
