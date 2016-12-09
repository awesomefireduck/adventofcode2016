use std::fs::File;
use std::io::{BufRead,BufReader};
use std::vec::Vec;

fn main() {
    let input = File::open("input").expect("input does not exist");
    let count = count_impossible_triangles(input);
    println!("{:?}", count);
}

fn count_impossible_triangles(input: File) -> u64 {
    let mut lined_input = BufReader::new(input).lines();

    let mut count = 0;

    while let Some(line) = lined_input.next() {
        let triangle_line = line.expect("line of edges");
        let sides: Vec<&str> = triangle_line.split_whitespace().collect();
        let (min, mid, max) : (i64, i64, i64) = sort_sides(&sides);
        if (min + mid) > max {
            count = count + 1;
        }
    }
    count

}


fn sort_sides(sides: &Vec<&str>) -> (i64, i64, i64) {
    let side_a = sides[0].parse::<i64>().expect("number");
    let side_b = sides[1].parse::<i64>().expect("number");
    let side_c = sides[2].parse::<i64>().expect("number");

    let mut sorted_sides = [side_a, side_b, side_c];
    sorted_sides.sort();

    (sorted_sides[0], sorted_sides[1], sorted_sides[2])
}

