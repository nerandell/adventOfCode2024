mod days;
mod utils;

use utils::read_file;

fn main() {
    let day = std::env::args().nth(1).unwrap_or_else(|| "all".to_string());
    match day.as_str() {
        "1" => days::day01::run(&read_file("input/day01.txt").unwrap()),
        "2" => days::day02::run(&read_file("input/day02.txt").unwrap()),
        "3" => days::day03::run(&read_file("input/day03.txt").unwrap()),
        "all" => {
            days::day01::run(&read_file("input/day01.txt").unwrap());
            days::day02::run(&read_file("input/day02.txt").unwrap());
            days::day03::run(&read_file("input/day03.txt").unwrap());
        }
        _ => eprintln!("Unknown day: {}", day),
    }
}
