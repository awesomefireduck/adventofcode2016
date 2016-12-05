use std::fs::File;


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
    let direction = 0;
}
