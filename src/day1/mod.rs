use std::fs::File;
use std::io::prelude::*;


fn parse_input() -> Vec<i64> {

    let mut f = File::open("./src/day1/input.txt").expect("File Not Found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Error Reading File");
  
    contents.split("\n")
        .filter(|value| value != &"")
        .map(|value| {
            let num = value.parse::<i64>();
            match num {
                Ok(v) => v,
                Err(_e) => 0
            }
        })
        .collect()
}


pub fn part1() -> String<> {

    let result = parse_input()
        .into_iter()
        .fold(0, |acc, value| acc + ((value / 3) - 2));

    result.to_string()
}

fn add_extra_fuel(fuel: i64) -> i64 {

    if fuel <= 0 { return 0 }

    let extra = (fuel / 3) - 2;
    fuel + add_extra_fuel(extra)
}

pub fn part2() -> String<> {

    let result = parse_input()
        .into_iter()
        .fold(0, |acc, value| {
            
            let mod_fuel: i64 = (value / 3) - 2;
            acc + add_extra_fuel(mod_fuel)
        });

    result.to_string()
}