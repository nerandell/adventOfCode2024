struct Direction {
    x: i32,
    y: i32,
}

const POSSIBLE_MOVES: [Direction; 8] = [
    Direction { x: 0, y: 1 },
    Direction { x: 0, y: -1 },
    Direction { x: 1, y: 0 },
    Direction { x: -1, y: 0 },
    Direction { x: 1, y: 1 },
    Direction { x: -1, y: 1 },
    Direction { x: 1, y: -1 },
    Direction { x: -1, y: -1 },
];

#[derive(Debug)]
struct Solution {
    ans: i32,
}

impl Solution {
    fn new() -> Solution {
        Solution { ans: 0 }
    }

    fn increment(&mut self) {
        self.ans += 1;
    }
}

fn part1(input: &Vec<String>) -> i32 {
    let rows = input.len();
    let cols = input[0].len();
    let pattern = String::from("XMAS");
    let mut solution = Solution::new();

    for i in 0..rows {
        for j in 0..cols {
            if input[i].chars().nth(j).unwrap() == pattern.chars().nth(0).unwrap() {
                for direction in POSSIBLE_MOVES.iter() {
                    find_word(
                        input,
                        1,
                        &pattern,
                        i as i32,
                        j as i32,
                        &mut solution,
                        direction,
                    );
                }
            }
        }
    }

    solution.ans
}

fn find_word(
    input: &Vec<String>,
    index: i32,
    pattern: &String,
    row: i32,
    col: i32,
    ans: &mut Solution,
    direction: &Direction,
) {
    let new_row = row + direction.x;
    let new_col = col + direction.y;
    if new_row < 0
        || new_row >= input.len() as i32
        || new_col < 0
        || new_col >= input[0].len() as i32
    {
        return;
    }

    if input[new_row as usize]
        .chars()
        .nth(new_col as usize)
        .unwrap()
        == pattern.chars().nth(index as usize).unwrap()
    {
        if index == (pattern.len() as i32 - 1) {
            ans.increment();
        } else if index < (pattern.len() as i32 - 1) {
            find_word(input, index + 1, pattern, new_row, new_col, ans, direction);
        }
    }
}

fn part2(input: &Vec<String>) -> i32 {
    let rows = input.len();
    let cols = input[0].len();
    let mut solution = Solution::new();
    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if input[i].chars().nth(j).unwrap() == 'A' {
                if (input[i - 1].chars().nth(j - 1).unwrap() == 'M'
                    && input[i + 1].chars().nth(j + 1).unwrap() == 'S')
                    || (input[i - 1].chars().nth(j - 1).unwrap() == 'S'
                        && input[i + 1].chars().nth(j + 1).unwrap() == 'M')
                {
                    if (input[i - 1].chars().nth(j + 1).unwrap() == 'M'
                        && input[i + 1].chars().nth(j - 1).unwrap() == 'S')
                        || (input[i - 1].chars().nth(j + 1).unwrap() == 'S'
                            && input[i + 1].chars().nth(j - 1).unwrap() == 'M')
                    {
                        solution.increment();
                    }
                }
            }
        }
    }

    solution.ans
}

pub fn run(input: &Vec<String>) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
