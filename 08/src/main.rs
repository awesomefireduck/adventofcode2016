#![feature(field_init_shorthand)]

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashSet;
use std::vec::Vec;
use std::fmt::Debug;

type Screen = HashSet<(u8, u8)>;

enum Operation {
    Fill {a: u8, b: u8},
    RowRotate {row: u8, shift: u8},
    ColumnRotate {column: u8, shift: u8}
}

fn print_display(display: &Screen) {
    let mut output = [['.';50]; 6];
    for &(x, y) in display.iter() {
        if y > 5 {
            println!("{:?}", (x,y));
        }
        output[y as usize][x as usize] = '#';
    }
    for line in output.iter() {
        for pixel in line.iter() {
            print!("{}", pixel);
        }
        println!("");
    }
}

fn main() {
    let input = File::open("input").expect("oeuaeouoeauoeuaeo");
    let mut lined_input = BufReader::new(input).lines().peekable();
    let mut operations : Vec<Operation> = vec![];

    while let Some(line) = lined_input.next() {
        if let Ok(operation) = parse_instruction(&line.expect("instruction")) {
            operations.push(operation);
        }
    }
    let display : Screen = HashSet::new();

    let message = execute_instructions(&operations, &display);
    print_display(&message);
    println!("{}", message.len());
    //let lit_pixels = count_lit_pixels(&message);
}

fn parse_instruction(instruction: &str) -> Result<Operation, &str> {

    if instruction.starts_with("rotate") {
        // "rotate column x=3 by 2" => "column x=3 by 2"
        parse_rotate(&instruction[7..])
    } else if instruction.starts_with("rect") {
        // "rect 3x3" => "3x3"
        parse_rect(&instruction[5..])
    } else {
        Err("not a proper instruction")
    }
}

fn parse_rotate(instruction: &str) -> Result<Operation, &str> {
    if instruction.starts_with("column") {
        let column;
        let shift;
        // "column x=5 by 3" => "5 by 3"
        let details = &instruction[9..];
        let mut split_details = details.split(" by ");
        if let Some(column_str) = split_details.next() {
            column = column_str.parse().expect("column");
        } else {
            return Err("not a proper instruction")
        }
        if let Some(shift_str) = split_details.next() {
            shift = shift_str.parse().expect("shift");
        } else {
            return Err("not a proper instruction")
        }

        Ok(Operation::ColumnRotate{column, shift})

    } else if instruction.starts_with("row") {
        let row;
        let shift;
        // "row y=5 by 3" => "5 by 3"
        let details = &instruction[6..];
        let mut split_details = details.split(" by ");

        if let Some(row_str) = split_details.next() {
            row = row_str.parse().expect("row");
        } else {
            return Err("not a proper instruction")
        }
        if let Some(shift_str) = split_details.next() {
            shift = shift_str.parse().expect("shift");
        } else {
            return Err("not a proper instruction")
        }

        Ok(Operation::RowRotate{row, shift})

    } else {
        Err("not a proper instruction")
    }
}

fn parse_rect(instruction: &str) -> Result<Operation, &str> {
    let a;
    let b;
    let mut split = instruction.split("x");
    if let Some(a_str) = split.next() {
        a = a_str.parse().expect("A");
    } else {
        return Err("not a proper instruction")
    }
    if let Some(b_str) = split.next() {
        b = b_str.parse().expect("B");
    } else {
        return Err("not a proper instruction")
    }
    Ok(Operation::Fill{a, b})
}

fn execute_instructions(instructions: &Vec<Operation>, display: &Screen) -> Screen {
    let mut display : Screen = display.clone();
    for instruction in instructions {

        display = match instruction {
            &Operation::Fill{a, b} => {println!("{}, {}",a,b); get_rect(&display, a, b) },
            &Operation::ColumnRotate{column, shift} => rotate_column(&display, column, shift),
            &Operation::RowRotate{row, shift} => rotate_row(&display, row, shift),
        };
    }
    display
}

fn get_rect(prev_display: &Screen, a: u8, b: u8) -> Screen {
    let mut display = prev_display.clone();

    for x in 0..a {
        for y in 0..b {
            display.insert((x,y));
        }
    }

    display
}

fn rotate_column(prev_display: &Screen, column: u8, shift: u8) -> Screen {
    let mut display = Screen::new();
    for prev_pixel_pos in prev_display.iter() {
        let pixel_pos = match prev_pixel_pos {
            &(x, y) if x == column => (x, (y+shift)%6),
            _ => *prev_pixel_pos,
        };
        display.insert(pixel_pos);
    }
    display
}

fn rotate_row(prev_display: &Screen, row: u8, shift: u8) -> Screen {
    let mut display = Screen::new();
    for prev_pixel_pos in prev_display.iter() {
        let pixel_pos = match prev_pixel_pos {
            &(x, y) if y == row => ((x+shift)%50, y),
            _ => *prev_pixel_pos,
        };
        display.insert(pixel_pos);
    }
    display
}


#[cfg(test)]
mod tests {
    use super::Screen;
    use std::collections::HashSet;
    use super::print_display;

    #[test]
    fn get_rect() {
        let testcases = [
            (2,2, vec![(0,0), (0,1),(1,0), (1,1)]),
            (0,0, vec![]),
        ];

        for case in testcases.iter() {
            println!("Testing {}x{}:", case.0, case.1);

            let mut expected_display = Screen::new();
            for pixel in case.2.iter() {
                expected_display.insert(*pixel);
            }

            let display = super::get_rect(&Screen::new(), case.0, case.1);

            println!("expected:");
            print_display(&expected_display);
            println!("got display:");
            print_display(&display);

            assert_eq!(display.symmetric_difference(&expected_display).count(), 0);
        }
    }
    #[test]
    fn rotate_column() {
        let testcases = [
            (0,2, vec![(0,0), (0,1),(1,0), (1,1)],vec![(0,2), (0,3),(1,0), (1,1)]),
        ];

        for case in testcases.iter() {
            println!("Testing {}x{}:", case.0, case.1);

            let mut start_display = Screen::new();
            for pixel in case.2.iter() {
                start_display.insert(*pixel);
            }

            let mut expected_display = Screen::new();
            for pixel in case.3.iter() {
                expected_display.insert(*pixel);
            }

            let display = super::rotate_column(&start_display, case.0, case.1);

            println!("expected:");
            print_display(&expected_display);
            println!("got display:");
            print_display(&display);

            assert_eq!(display.symmetric_difference(&expected_display).count(), 0);
        }
    }
    #[test]
    fn rotate_row() {
        let testcases = [
            (0,2, vec![(0,0), (0,1),(1,0), (1,1)],vec![(2,0), (0,1),(3,0), (1,1)]),
        ];

        for case in testcases.iter() {
            println!("Testing {}x{}:", case.0, case.1);

            let mut start_display = Screen::new();
            for pixel in case.2.iter() {
                start_display.insert(*pixel);
            }

            let mut expected_display = Screen::new();
            for pixel in case.3.iter() {
                expected_display.insert(*pixel);
            }


            let display = super::rotate_row(&start_display, case.0, case.1);

            println!("expected:");
            print_display(&expected_display);
            println!("got display:");
            print_display(&display);

            assert_eq!(display.symmetric_difference(&expected_display).count(), 0);
        }
    }
}
