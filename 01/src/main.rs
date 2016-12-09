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

    let positionlog = find_fastest_route(start_heading, input);

    let &(latitude, longtitude) = positionlog.last().expect("aoeu");

    // shortest manhattan distance is lat + lon
    println!("total: {}", (latitude.abs()+longtitude.abs()));

    let position_visited_twice = find_position_visited_twice(positionlog);

    let (latitude, longtitude) = position_visited_twice.expect("aa");

    // shortest manhattan distance is lat + lon
    println!("twice: {}", (latitude.abs()+longtitude.abs()));
}

fn find_position_visited_twice(old_positionlog: Vec<(i32,i32)>) -> Option<(i32, i32)> {
    let positionlog_clone = old_positionlog.clone();
    let mut positionlog = positionlog_clone.iter().enumerate();

    let mut double_position : Option<(i32, i32)> = None;

    while let Some((index, position)) = positionlog.next() {
        if old_positionlog[0..index].contains(position) {
            double_position = Some(*position);
            break;
        }
    }

    double_position
}

fn find_fastest_route(start_heading: i32, input: File) -> Vec<(i32, i32)> {

    let mut heading = start_heading;
    let start_position = (0,0);
    let mut positionlog : Vec<(i32, i32)> = vec![];
    // push starting position
    positionlog.push(start_position);

    // BufReader makes iterating over lines possible
    let mut lined_input = BufReader::new(input).lines();

    while let Some(line) = lined_input.next() {
        let moves_line = line.expect("expected line of commands");
        let mut moves = moves_line.split(", ");

        while let Some(single_move) = moves.next() {
            let (turn_direction, steps_forward) = single_move.split_at(1);

            heading = turn_in_direction(heading, turn_direction);
            positionlog = move_to_position(&positionlog, steps_forward, heading);
        }
    }
    positionlog
}

fn move_to_position(old_positionlog: &Vec<(i32, i32)>, steps_string: &str, heading: i32) -> Vec<(i32, i32)> {
    let steps_forward = steps_string.parse::<i32>().expect("oaeuoeaouoe");
    let mut positionlog = old_positionlog.clone();
    let position = old_positionlog.last().expect("aoeueo");

    // relative movement in two directions
    // latitude (-W +E), longtitude (-S +N)
    let steps = match heading % 4 {
        0 => (0,  steps_forward),
        2 => (0, -steps_forward),

        1 => ( steps_forward, 0),
        3 => (-steps_forward, 0),

        _ => panic!("unreachable!"),
    };
    let new_position = (position.0 + steps.0, position.1 + steps.1);

    if steps.0 != 0 {
        if steps.0 > 0 {
            for latitude in (position.0+1..new_position.0+1) {
                positionlog.push((latitude, new_position.1));
            }
        } else {
            for latitude in (new_position.0-1..position.0-1).rev() {
                positionlog.push((latitude, new_position.1));
            }
        };
    } else if steps.1 != 0 {
        if steps.1 > 0 {
            for longtitude in (position.1+1..new_position.1+1) {
                positionlog.push((new_position.0, longtitude));
            }
        } else {
            for longtitude in (new_position.1-1..position.1-1).rev() {
                positionlog.push((new_position.0, longtitude));
            }
        }
    } else {
        panic!("AAAAAAAAH");
    }

    positionlog
}

fn turn_in_direction(old_heading: i32, turn_direction: &str) -> i32 {
    match turn_direction {
        "L" => turn_left(old_heading),
        "R" => turn_right(old_heading),
        _   => panic!("invalid input: direction not one of 'L' or 'R'")
    }
}

