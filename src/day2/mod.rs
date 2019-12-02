use std::fs::File;
use std::io::prelude::*;


fn parse_input() -> Vec<usize> {

    let mut f = File::open("./src/day2/input.txt").expect("File Not Found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Error Reading File");

    contents.split(",")
        .filter(|value| value != &"")
        .map(|value| {
            let num = value.parse::<usize>();
            match num {
                Ok(v) => v,
                Err(_e) => 0
            }
        })
        .collect()
}

#[derive(Debug)]
enum Operation {
    ADD,
    MULTIPLY
}

fn run_op(mut program: Vec<usize>, idx: usize, operation: Operation) -> Vec<usize> {

    let a_pos = program[idx + 1];
    let b_pos = program[idx + 2];

    let res = match operation {
        Operation::ADD => program[a_pos] + program[b_pos],
        Operation::MULTIPLY => program[a_pos] * program[b_pos]
    };

    let insert_idx = program[idx + 3];
    program[insert_idx] = res;
    program
}

fn run_steps(program: Vec<usize>, idx: usize) -> Vec<usize> {

    let opcode = program[idx];
    match opcode {
        1 => run_steps(run_op(program, idx, Operation::ADD), idx + 4),
        2 => run_steps(run_op(program, idx, Operation::MULTIPLY), idx + 4),
        99 => program,
        _ => panic!("Unexpected OpCode")
    }
}

pub fn part1() -> String<> {

    let mut program = parse_input();

    // Apply gravity program initial state
    program[1] = 12;
    program[2] = 2;

    let final_prog = run_steps(program, 0);

    final_prog[0].to_string()
}


pub fn part2() -> String<> {

    let program = parse_input();

    for i in 0..99 {
        for j in 0..99 {

            let mut prog_clone = program.clone();
            prog_clone[1] = i;
            prog_clone[2] = j;

            let res = run_steps(prog_clone, 0);

            if res[0] == 19690720 {
                return ((100 * i) + j).to_string() 
            }
        }
    }

    String::new()
}