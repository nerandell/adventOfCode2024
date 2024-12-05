use regex::Regex;

fn part1(input: &Vec<String>) -> i32 {
    let re = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();
    let mut ans = 0;
    for line in input {
        for (_, [num1, num2]) in re.captures_iter(line).map(|c| c.extract()) {
            ans += num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap();
        }
    }

    ans
}

fn part2(input: &Vec<String>) -> i32 {
    let re = Regex::new(r"mul\(\d+,\d+\)|don'?t\(\)|do\(\)").unwrap();
    let mut ans = 0;
    let mut enabled = true;

    for line in input {
        for cap in re.captures_iter(line) {
            let command = cap.get(0).unwrap().as_str();
            if enabled && command.starts_with("mul") {
                let mut nums = command[4..command.len() - 1].split(",");
                ans += nums.next().unwrap().parse::<i32>().unwrap()
                    * nums.next().unwrap().parse::<i32>().unwrap();
            } else if command == "don't()" {
                enabled = false;
            } else if command == "do()" {
                enabled = true;
            }
        }
    }

    ans
}

pub fn run(input: &Vec<String>) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
