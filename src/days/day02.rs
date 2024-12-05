fn validate_list(numbers: &Vec<i32>) -> bool {
    let mut result = true;
    if numbers.len() < 2 {
        result = true
    }

    let increasing = numbers[1] > numbers[0];
    let decreasing = numbers[1] < numbers[0];

    if !increasing && !decreasing {
        result = false
    }

    for i in 1..numbers.len() {
        let diff = (numbers[i] - numbers[i - 1]).abs();
        if diff < 1 || diff > 3 {
            result = false;
        }
        if increasing && numbers[i] <= numbers[i - 1] {
            result = false;
        }
        if decreasing && numbers[i] >= numbers[i - 1] {
            result = false;
        }
    }

    result
}

fn part1(input: &Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    for numbers in input {
        if validate_list(numbers) {
            ans += 1;
        }
    }
    ans
}

fn part2(input: &Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    for numbers in input {
        if validate_list(numbers) {
            ans += 1;
        } else {
            for i in 0..numbers.len() {
                let mut temp = numbers.clone();
                temp.remove(i);
                if validate_list(&temp) {
                    ans += 1;
                    break;
                }
            }
        }
    }
    ans
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<i32>> {
    let mut parsed_input = Vec::new();
    for line in input {
        let cols: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        parsed_input.push(cols);
    }
    parsed_input
}

pub fn run(input: &Vec<String>) {
    let parsed_input = parse_input(input);
    println!("Part 1: {}", part1(&parsed_input));
    println!("Part 2: {}", part2(&parsed_input));
}
