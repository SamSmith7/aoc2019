extern crate aoc2019;

use std::env;
use aoc2019::day1;
use aoc2019::day2;


struct AocResult {
    part1: String,
    part2: String
}

impl AocResult {
    fn format(&self) -> String<> {
        format!("Part 1: {}, Part 2: {}", self.part1, self.part2)
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let day = args.get(1).unwrap();

    let result = match day.as_ref() {
        "day1" => {
            AocResult {
                part1: day1::part1(),
                part2: day1::part2()
            }
        },
        "day2" => {
            AocResult {
                part1: day2::part1(),
                part2: day2::part2()
            }
        },
        _ => AocResult {
            part1: String::new(),
            part2: String::new()
        }
    };


    println!("{:?}", result.format());
}