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
    let mut triangles : Vec<(i64, i64, i64)> = vec!();

    while let Some(line1) = lined_input.next() {
        let triangle_line1 = line1.expect("line of edges");
        let sides1: Vec<&str> = triangle_line1.split_whitespace().collect();

        let line2 = lined_input.next().expect("line of edges");
        let triangle_line2 = line2.expect("line of edges");
        let sides2: Vec<&str> = triangle_line2.split_whitespace().collect();

        let line3 = lined_input.next().expect("line of edges");
        let triangle_line3 = line3.expect("line of edges");
        let sides3: Vec<&str> = triangle_line3.split_whitespace().collect();

        let left_triangle   = (sides1[0].parse().expect("A"), sides2[0].parse().expect("A"), sides3[0].parse().expect("A"));
        let mid_triangle    = (sides1[1].parse().expect("A"), sides2[1].parse().expect("A"), sides3[1].parse().expect("A"));
    let right_triangle  = (sides1[2].parse().expect("A"), sides2[2].parse().expect("A"), sides3[2].parse().expect("A"));

        triangles.push(left_triangle);
        triangles.push(mid_triangle);
        triangles.push(right_triangle);

    }

    let mut triangle_list = triangles.iter();

    while let Some(triangle) = triangle_list.next() {
        let (min, mid, max) : (i64, i64, i64) = sort_sides(&triangle);

        if (min + mid) > max {
            count = count + 1;
        }
    }
    count
}


fn sort_sides(sides: &(i64, i64, i64)) -> (i64, i64, i64) {
    let side_a = sides.0;
    let side_b = sides.1;
    let side_c = sides.2;

    let mut sorted_sides = [side_a, side_b, side_c];
    sorted_sides.sort();

    (sorted_sides[0], sorted_sides[1], sorted_sides[2])
}

