use std::fs::File;
use std::io::prelude::*;
use std::cmp;


#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT
}

#[derive(Copy, Clone, Debug)]
struct Action {
    direction: Direction,
    distance: i32
}

impl Action {
    fn new(direction: Direction, distance: i32) -> Action {
        Action {
            direction,
            distance
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Line {
    direction: Direction,
    start: Point,
    end: Point
}

impl Line {
    fn new(start: Point, end: Point, direction: Direction) -> Line {
        Line { direction, start, end }
    } 
}

fn intersects(line1: &Line, line2: &Line) -> Option<Point> {

    let parallel: bool = match line1.direction {
        Direction::UP => line2.direction == Direction::UP || line2.direction == Direction::DOWN,
        Direction::DOWN => line2.direction == Direction::UP || line2.direction == Direction::DOWN,
        Direction::LEFT => line2.direction == Direction::LEFT || line2.direction == Direction::RIGHT,
        Direction::RIGHT => line2.direction == Direction::LEFT || line2.direction == Direction::RIGHT,
    };

    if parallel { return Option::None }

    if line1.direction == Direction::UP || line1.direction == Direction::DOWN {
        if line1.start.x > cmp::min(line2.start.x, line2.end.x) && line1.start.x < cmp::max(line2.start.x, line2.end.x) {
            if cmp::min(line1.start.y, line1.end.y) < line2.start.y && cmp::max(line1.start.y, line1.end.y) > line2.start.y {
                return Option::Some(Point::new(line1.start.x, line2.start.y))
            }
        }
    } else {
        if line1.start.y > cmp::min(line2.start.y, line2.end.y) && line1.start.y < cmp::max(line2.start.y, line2.end.y) {
            if cmp::min(line1.start.x, line1.end.x) < line2.start.x && cmp::max(line1.start.x, line1.end.x) > line2.start.x {
                return Option::Some(Point::new(line2.start.x, line1.start.y))
            }
        }
    }
    Option::None
}

fn wire_to_actions(wire: String) -> Vec<Action> {

    wire.split(",")
        .map(|val| {
            let mut chars = val.chars();
            let direction: Direction = match chars.next() {
                Some('U') => Direction::UP,
                Some('D') => Direction::DOWN,
                Some('L') => Direction::LEFT,
                Some('R') => Direction::RIGHT,
                _ => panic!("Invalid Action")
            };
            let num: String = chars.collect();
            let distance = num.parse::<i32>().unwrap();
            Action::new(direction, distance)
        })
        .collect()
}

fn parse_input() -> Vec<Vec<Action>> {

    let mut f = File::open("./src/day3/input.txt").expect("File Not Found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Error Reading File");
  

    let mut lines = contents.lines();
    let wire1 = String::from(lines.next().unwrap());
    let wire2 = String::from(lines.next().unwrap());

    vec![wire_to_actions(wire1), wire_to_actions(wire2)]
}

fn wire_to_coords(wire: Vec<Action>) -> Vec<Line> {

    let mut start = Point::new(0, 0);
    let mut output: Vec<Line> = vec![];

    for action in wire {
        let end = match action.direction {
            Direction::RIGHT => Point::new(start.x + action.distance, start.y),
            Direction::LEFT => Point::new(start.x - action.distance, start.y),
            Direction::UP => Point::new(start.x, start.y + action.distance),
            Direction::DOWN => Point::new(start.x, start.y - action.distance),
        };
        output.push(Line::new(Point::new(start.x, start.y), Point::new(end.x, end.y), action.direction));
        start = end
    }
    output
}

pub fn part1() -> String<> {

    let wires: Vec<Vec<Line>> = parse_input()
        .into_iter()
        .map(wire_to_coords)
        .collect();

    let wire1 = &wires[0];
    let wire2 = &wires[1];

    let mut cross_points = vec![];

    for line1 in wire1 {
        for line2 in wire2 {
            match intersects(line1, line2) {
                Some(point) => cross_points.push(point),
                None => ()
            }
        }
    }

    let closest = cross_points.into_iter()
        .map(|point| point.x.abs() + point.y.abs())
        .min();

    closest.unwrap().to_string()
}


pub fn part2() -> String<> {

    String::new()
}