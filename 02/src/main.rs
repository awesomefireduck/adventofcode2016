use std::vec::Vec;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {

    let input = File::open("input").expect("input does not exist");

    let code : Vec<_> = find_bathroom_code(input);
    println!("{:?}", code);
}

fn find_bathroom_code(input: File) -> Vec<char> {

    let mut lined_input = BufReader::new(input).lines();

    let mut code = vec![];

    //      1
    //    2 3 4
    //  5 6 7 8 9
    //    A B C
    //      D
    let position = '5';

    while let Some(line) = lined_input.next() {
        let moves_line = line.expect("expected line of commands");
        let position = find_next_position(position, &moves_line);
        // position at the end of a line of instructions
        // so it is part of the access code
        code.push(position);
    }
    code
}

fn find_next_position(old_position: char, moves_line: &str) -> char {

    let mut moves = moves_line.chars();
    let mut position = old_position;

    while let Some(instruction) = moves.next() {
        position = match instruction {
            'U' if "3678ABCD".contains(position) => move_up(&position),
            'D' if "1234678B".contains(position) => move_down(&position),
            'L' if "346789BC".contains(position) => move_left(&position),
            'R' if "235678AB".contains(position) => move_right(&position),
            // other moves are invalid
            _   => position,
        };
    }
    position
}

fn move_right(old_position: &char) -> char {
    let new_position : char = match *old_position {
        '2' => '3',
        '3' => '4',
        '5' => '6',
        '6' => '7',
        '7' => '8',
        '8' => '9',
        'A' => 'B',
        'B' => 'C',
        _   => *old_position,
    };
    new_position
}

fn move_left(old_position: &char) -> char {
    let new_position : char = match *old_position {
        '3' => '2',
        '4' => '3',
        '6' => '5',
        '7' => '6',
        '8' => '7',
        '9' => '8',
        'B' => 'A',
        'C' => 'B',
        _   => *old_position,
    };
    new_position
}
fn move_down(old_position: &char) -> char {
    let new_position : char = match *old_position {
        '1' => '3',
        '2' => '6',
        '3' => '7',
        '4' => '8',
        '6' => 'A',
        '7' => 'B',
        '8' => 'C',
        'B' => 'D',
        _   => *old_position,
    };
    new_position
}

fn move_up(old_position: &char) -> char {
    let new_position : char = match *old_position {
        '3' => '1',
        '6' => '2',
        '7' => '3',
        '8' => '4',
        'A' => '6',
        'B' => '7',
        'C' => '8',
        'D' => 'B',
        _   => *old_position,
    };
    new_position
}
