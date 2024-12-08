use std::collections::HashMap;

fn part1(input: &Vec<String>) -> i32 {
    let (before, questions) = parse_input(input);
    let mut answer = 0;
    for question in questions {
        let mut valid = true;
        for i in 0..question.len() {
            let current = question[i];
            for j in i..question.len() {
                let next = question[j];
                if before.contains_key(&next) {
                    let rule = before.get(&next).unwrap();
                    if rule.contains(&current) {
                        valid = false;
                        break;
                    }
                }
            }
        }

        if valid {
            let mid = if question.len() % 2 == 0 {
                question.len() / 2 - 1
            } else {
                question.len() / 2
            };
            answer += question[mid].parse::<i32>().unwrap();
        }
    }

    answer
}

fn part2(input: &Vec<String>) -> i32 {
    let (before, questions) = parse_input(input);
    let mut answer = 0;
    for mut question in questions {
        let mut valid = false;
        for _ in 0..question.len() - 1 {
            for j in 1..question.len() {
                if before.contains_key(&question[j]) {
                    let rule = before.get(&question[j]).unwrap();
                    if rule.contains(&question[j - 1]) {
                        valid = true;
                        let tmp = question[j];
                        question[j] = question[j - 1];
                        question[j - 1] = tmp;
                    }
                }
            }
        }

        if valid {
            let mid = if question.len() % 2 == 0 {
                question.len() / 2 - 1
            } else {
                question.len() / 2
            };
            answer += question[mid].parse::<i32>().unwrap();
        }
    }

    answer
}

fn parse_input(input: &Vec<String>) -> (HashMap<&str, Vec<&str>>, Vec<Vec<&str>>) {
    let mut before: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut questions: Vec<Vec<&str>> = Vec::new();

    let mut rules = true;
    for line in input {
        if line == "" {
            rules = false;
            continue;
        }

        if rules {
            let parts: Vec<&str> = line.split("|").collect();
            let before_part = parts[0].trim();
            let after_part = parts[1].trim();

            if before.contains_key(before_part) {
                before.get_mut(before_part).unwrap().push(after_part);
            } else {
                let mut vec = Vec::new();
                vec.push(after_part);
                before.insert(before_part, vec);
            }
        } else {
            let question: Vec<&str> = line.split(",").collect();
            questions.push(question);
        }
    }

    (before, questions)
}

pub fn run(input: &Vec<String>) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
