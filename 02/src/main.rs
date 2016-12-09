use std::vec::Vec;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {

    let input = File::open("input").expect("input does not exist");

    let code : Vec<_> = find_bathroom_code(input);
    println!("{:?}", code);
}

fn find_bathroom_code(input: File) -> Vec<i32> {

    let mut lined_input = BufReader::new(input).lines();

    let mut code = vec![];

    // 1 2 3
    // 4 5 6
    // 7 8 9
    let position = 5;

    while let Some(line) = lined_input.next() {
        let moves_line = line.expect("expected line of commands");
        let position = find_next_position(position, &moves_line);
        // position at the end of a line of instructions
        // so it is part of the access code
        code.push(position);
    }
    code
}

fn find_next_position(old_position: i32, moves_line: &str) -> i32 {

    let mut moves = moves_line.chars();
    let mut position = old_position;

    while let Some(instruction) = moves.next() {
        position = match instruction {
            // only valid for bottom two rows
            'U' if [4,5,6,7,8,9].contains(&position) => position - 3,
            // only valid for top two rows
            'D' if [1,2,3,4,5,6].contains(&position) => position + 3,
            // only valid for two rightmost columns
            'L' if [2,3,5,6,8,9].contains(&position) => position - 1,
            // only valid for two leftmost columns
            'R' if [1,2,4,5,7,8].contains(&position) => position + 1,
            // other moves are invalid
            _   => position,
        };
    }
position
}
