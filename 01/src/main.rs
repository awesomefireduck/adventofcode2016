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

    let least_steps = find_fastest_route(start_heading, steps, input);

    let (latitude, longtitude) = least_steps;

    println!("total: {}", (latitude.abs()+longtitude.abs()));

}


fn find_fastest_route(start_heading: i32, start_steps: (i32, i32), input: File) -> (i32, i32) {

    let mut heading = start_heading;
    let mut steps = start_steps;

    // BufReader makes iterating over lines possible
    let mut lined_input = BufReader::new(input).lines();

    while let Some(line) = lined_input.next() {
        let moves_line = line.expect("eu.up");
        let mut moves = moves_line.split(", ");

        while let Some(single_move) = moves.next() {
            let (turn_direction, steps_forward) = single_move.split_at(1);

            heading = turn_in_direction(heading, turn_direction);
            steps = move_in_heading(steps, heading, steps_forward);
        }
    }
    steps
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
