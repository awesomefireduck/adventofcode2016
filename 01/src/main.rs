use std::fs::File;
use std::io::{BufReader, BufRead};


fn turn_left(old_heading: i32) -> i32 {
    (old_heading + 3) % 4
}

fn turn_right(old_heading: i32) -> i32 {
    (old_heading + 1) % 4
}

fn main() {
    let input = File::open("input").expect("oeuaeouoeauoeuaeo");

    //    N0
    // W3    E1
    //    S2
    let start_heading = 0;


    // latitude (-W +E), longtitude (-S +N)
    let steps = (0, 0);

    let steplog = find_fastest_route(start_heading, steps, input);

    let &(latitude, longtitude) = steplog.last().expect("aoeu");

    // shortest manhattan distance is lat + lon
    println!("total: {}", (latitude.abs()+longtitude.abs()));

    let position_visited_twice = find_position_visited_twice(steplog);

    let (latitude, longtitude) = position_visited_twice.expect("aa");

    // shortest manhattan distance is lat + lon
    println!("total: {}", (latitude.abs()+longtitude.abs()));
}


fn find_position_visited_twice(steplog: Vec<(i32,i32)>) -> Option<(i32, i32)> {
    let mut sorted_steplog = steplog.clone();
    sorted_steplog.sort();
    let mut steplog = sorted_steplog.iter().peekable();
    let mut double_position : Option<(i32, i32)> = None;

    while let Some(step) = steplog.next() {
        if let Some(next_step) = steplog.peek() {
            if *next_step == step {
                double_position = Some(*step);
            }
        } else {
            break;
        }
    }

    double_position
}

fn find_fastest_route(start_heading: i32, start_steps: (i32, i32), input: File) -> Vec<(i32, i32)> {

    let mut heading = start_heading;
    let mut steps = start_steps;
    let mut steplog : Vec<(i32, i32)> = vec![steps];

    // BufReader makes iterating over lines possible
    let mut lined_input = BufReader::new(input).lines();

    while let Some(line) = lined_input.next() {
        let moves_line = line.expect("eu.up");
        let mut moves = moves_line.split(", ");

        while let Some(single_move) = moves.next() {
            let (turn_direction, steps_forward) = single_move.split_at(1);

            heading = turn_in_direction(heading, turn_direction);
            steps = move_in_heading(steps, heading, steps_forward);
            steplog.push(steps);
        }
    }
    steplog
}

fn turn_in_direction(old_heading: i32, turn_direction: &str) -> i32 {
    match turn_direction {
        "L" => turn_left(old_heading),
        "R" => turn_right(old_heading),
        _   => panic!("invalid input: direction not one of 'L' or 'R'")
    }
}

fn move_in_heading(step_count: (i32, i32), heading: i32, step_string: &str) -> (i32, i32) {

    if let Ok(steps) = step_string.parse::<i32>() {
        let (latitude, longtitude) = step_count;

        let new_step_count = match heading % 4 {
            0 => (latitude, longtitude+steps),
            2 => (latitude, longtitude-steps),

            1 => (latitude+steps, longtitude),
            3 => (latitude-steps, longtitude),

            _ => panic!("unreachable"),
        };

        new_step_count
    } else {
        panic!("oaeaoeuaoeuhtaeoduro'ethbtihadoh'lrsnhb");
    }
}
